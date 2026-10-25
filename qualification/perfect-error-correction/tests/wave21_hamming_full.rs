use perfect_error_correction::{CodeError, Hamming, PackedBitOrder, PackedCorrection};

fn set(bytes: &mut [u8], pos: usize, order: PackedBitOrder) {
    let bit = match order {
        PackedBitOrder::MostSignificantFirst => 7 - pos % 8,
        PackedBitOrder::LeastSignificantFirst => pos % 8,
    };
    bytes[pos / 8] ^= 1 << bit;
}
fn pack(bits: &[bool], order: PackedBitOrder) -> Vec<u8> {
    let mut data = vec![0; bits.len().div_ceil(8)];
    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            set(&mut data, i, order);
        }
    }
    data
}
// Independent binary even-parity encoder: no calls to production encoding
// or syndrome logic. parity-check positions are 1-indexed powers of two.
fn parity_oracle(input: &[bool], parity: u8, extended: bool) -> Vec<bool> {
    let n = (1usize << parity) - 1;
    let mut codeword = vec![false; n + usize::from(extended)];
    let mut at = 0;
    for pos in 1..=n {
        if !pos.is_power_of_two() {
            codeword[pos - 1] = input[at];
            at += 1;
        }
    }
    for bit in 0..parity {
        let mask = 1usize << bit;
        codeword[mask - 1] = (1..=n)
            .filter(|&pos| pos != mask && pos & mask != 0)
            .fold(false, |v, pos| v ^ codeword[pos - 1]);
    }
    if extended {
        codeword[n] = codeword[..n].iter().fold(false, |v, &b| v ^ b);
    }
    codeword
}
#[test]
fn all_hamming_sizes_packed_match_independent_even_parity_and_detect_two_errors() {
    for r in 3..=8 {
        for extended in [false, true] {
            let h = Hamming::new(r, extended).unwrap();
            for order in [
                PackedBitOrder::MostSignificantFirst,
                PackedBitOrder::LeastSignificantFirst,
            ] {
                let bits: Vec<bool> = (0..h.message_bits())
                    .map(|i| (i * 17 + usize::from(r)) % 7 < 3)
                    .collect();
                let input = pack(&bits, order);
                let expected = pack(&parity_oracle(&bits, r, extended), order);
                let actual = h.encode_packed(&input, h.message_bits(), order).unwrap();
                assert_eq!(actual.bytes(), expected);
                let mut msg = vec![0; input.len()];
                let mut cw = vec![0; expected.len()];
                for pos in 0..h.code_bits() {
                    let mut wrong = expected.clone();
                    set(&mut wrong, pos, order);
                    let syndrome = h.syndrome_packed(&wrong, h.code_bits(), order).unwrap();
                    assert_eq!(
                        syndrome.position,
                        if extended && pos == h.code_bits() - 1 {
                            0
                        } else {
                            pos + 1
                        }
                    );
                    assert_eq!(syndrome.overall_odd, extended);
                    assert_eq!(
                        h.decode_packed_into(&wrong, h.code_bits(), order, &mut cw, &mut msg)
                            .unwrap(),
                        PackedCorrection::Corrected { position: pos }
                    );
                    assert_eq!(cw, expected);
                    assert_eq!(msg, input);
                }
                if extended {
                    for i in 0..h.code_bits() {
                        for j in i + 1..h.code_bits() {
                            let mut wrong = expected.clone();
                            set(&mut wrong, i, order);
                            set(&mut wrong, j, order);
                            cw.fill(0x5a);
                            msg.fill(0x6b);
                            assert_eq!(
                                h.decode_packed_into(
                                    &wrong,
                                    h.code_bits(),
                                    order,
                                    &mut cw,
                                    &mut msg
                                )
                                .unwrap(),
                                PackedCorrection::DetectedUncorrectable
                            );
                            assert!(cw.iter().all(|&v| v == 0x5a));
                            assert!(msg.iter().all(|&v| v == 0x6b));
                        }
                    }
                } else {
                    let mut wrong = expected.clone();
                    set(&mut wrong, h.code_bits(), order);
                    cw.fill(0x5a);
                    msg.fill(0x6b);
                    assert!(matches!(
                        h.decode_packed_into(&wrong, h.code_bits(), order, &mut cw, &mut msg),
                        Err(CodeError::InvalidPadding)
                    ));
                    assert!(cw.iter().all(|&v| v == 0x5a));
                    assert!(msg.iter().all(|&v| v == 0x6b));
                }
            }
        }
    }
}
