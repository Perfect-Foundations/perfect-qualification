use perfect_error_correction::{CodeError, Hamming, PackedBitOrder, PackedCorrection};
const DATA: &str = include_str!("../vectors/hamming_packed_v1.tsv");
fn from_hex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks_exact(2)
        .map(|v| u8::from_str_radix(core::str::from_utf8(v).unwrap(), 16).unwrap())
        .collect()
}
#[test]
fn independent_packed_reference_through_caller_buffers() {
    let mut count = 0;
    for line in DATA
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let v: Vec<_> = line.split_whitespace().collect();
        let h = Hamming::new(v[0].parse().unwrap(), v[1] == "1").unwrap();
        let order = if v[2] == "1" {
            PackedBitOrder::MostSignificantFirst
        } else {
            PackedBitOrder::LeastSignificantFirst
        };
        let data = from_hex(v[6]);
        let expected = from_hex(v[7]);
        let mut word = vec![0u8; expected.len()];
        h.encode_packed_into(&data, h.message_bits(), order, &mut word)
            .unwrap();
        assert_eq!(word, expected);
        let mut corrected = vec![0u8; word.len()];
        let mut recovered = vec![0u8; data.len()];
        assert_eq!(
            h.decode_packed_into(&word, h.code_bits(), order, &mut corrected, &mut recovered)
                .unwrap(),
            PackedCorrection::Clean
        );
        assert_eq!(recovered, data);
        for index in [0, h.code_bits() / 2, h.code_bits() - 1] {
            let mut damaged = word.clone();
            let bit = if order == PackedBitOrder::MostSignificantFirst {
                7 - index % 8
            } else {
                index % 8
            };
            damaged[index / 8] ^= 1 << bit;
            assert_eq!(
                h.decode_packed_into(
                    &damaged,
                    h.code_bits(),
                    order,
                    &mut corrected,
                    &mut recovered
                )
                .unwrap(),
                PackedCorrection::Corrected { position: index }
            );
            assert_eq!(corrected, word);
            assert_eq!(recovered, data);
        }
        let mut malformed = word.clone();
        if h.code_bits() % 8 != 0 {
            let padding = if order == PackedBitOrder::MostSignificantFirst {
                1
            } else {
                128
            };
            *malformed.last_mut().unwrap() |= padding;
            assert!(matches!(
                h.decode_packed_into(
                    &malformed,
                    h.code_bits(),
                    order,
                    &mut corrected,
                    &mut recovered
                ),
                Err(CodeError::InvalidPadding)
            ));
        }
        count += 1;
    }
    assert_eq!(count, 48);
}
