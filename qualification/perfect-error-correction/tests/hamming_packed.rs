use perfect_error_correction::{
    CodeError, DecodeStatus, Hamming, HammingCodeProfile, PackedBitOrder, PackedBlock,
};

const SOURCE: &str = include_str!("../vectors/hamming_packed_v1.tsv");
fn hex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let text = core::str::from_utf8(pair).unwrap();
            u8::from_str_radix(text, 16).unwrap()
        })
        .collect()
}
#[test]
fn independently_computed_packed_hamming_every_supported_size_and_order() {
    let mut count = 0;
    for line in SOURCE
        .lines()
        .filter(|s| !s.starts_with('#') && !s.is_empty())
    {
        let row: Vec<_> = line.split_whitespace().collect();
        assert_eq!(row.len(), 8);
        let parity: u8 = row[0].parse().unwrap();
        let extended = row[1] == "1";
        let msb = row[2] == "1";
        let order = if msb {
            PackedBitOrder::MostSignificantFirst
        } else {
            PackedBitOrder::LeastSignificantFirst
        };
        let data_bits: usize = row[4].parse().unwrap();
        let code_bits: usize = row[5].parse().unwrap();
        let message = hex(row[6]);
        let expected = hex(row[7]);
        let h = Hamming::new(parity, extended).unwrap();
        let profile = HammingCodeProfile::new(h, order);
        assert_eq!(profile.message_bits(), data_bits);
        assert_eq!(profile.code_bits(), code_bits);
        let encoded = h.encode_packed(&message, data_bits, order).unwrap();
        assert_eq!(encoded.bytes(), expected, "packed reference row {count}");
        let typed = PackedBlock::new(message.clone(), data_bits, order).unwrap();
        assert_eq!(profile.encode_block(&typed).unwrap().bytes(), expected);
        let typed_word = PackedBlock::new(expected.clone(), code_bits, order).unwrap();
        assert_eq!(
            profile.decode_block(&typed_word).unwrap().status,
            DecodeStatus::Clean
        );
        assert_eq!(profile.syndrome_block(&typed_word).unwrap().position, 0);
        let opposite = if msb {
            PackedBitOrder::LeastSignificantFirst
        } else {
            PackedBitOrder::MostSignificantFirst
        };
        let mismatch = PackedBlock::new(vec![0u8; message.len()], data_bits, opposite).unwrap();
        assert!(matches!(
            profile.encode_block(&mismatch),
            Err(CodeError::IncompatibleProfile)
        ));
        let decoded = h.decode_packed(&expected, code_bits, order).unwrap();
        assert_eq!(decoded.status, DecodeStatus::Clean);
        assert_eq!(decoded.message.unwrap().bytes(), message);
        for position in [0, code_bits / 2, code_bits - 1] {
            let mut corrupted = expected.clone();
            let shift = if msb { 7 - position % 8 } else { position % 8 };
            corrupted[position / 8] ^= 1 << shift;
            let decoded = h.decode_packed(&corrupted, code_bits, order).unwrap();
            assert_eq!(
                decoded.status,
                DecodeStatus::Corrected {
                    positions: vec![position]
                }
            );
            assert_eq!(decoded.codeword.unwrap().bytes(), expected);
        }
        count += 1;
    }
    assert_eq!(count, 48);
}
