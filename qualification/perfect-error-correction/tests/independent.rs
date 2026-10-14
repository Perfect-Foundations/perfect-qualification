use perfect_error_correction::{
    DecodeStatus, ExtensionField, ExtensionLimits, Hamming, Natural, PrimeField, ReedSolomon,
    RsField,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
const HAM: &str = include_str!("../vectors/hamming_v1.tsv");
const GF5: &str = include_str!("../vectors/rs_gf5_v1.tsv");
const GF16: &str = include_str!("../vectors/rs_gf16_v1.tsv");
fn prime(p: u64) -> PrimeField {
    let PrimalityResult::ProvenPrime(pr) = classify_u64(p) else {
        panic!("prime proof missing")
    };
    PrimeField::new(PrimeModulus::from_proven(pr).unwrap())
}
fn gf16() -> RsField {
    let base = prime(2);
    let poly = [1_u64, 1, 0, 0, 1]
        .into_iter()
        .map(|x| base.element(Natural::from(x)).unwrap())
        .collect();
    RsField::Extension(ExtensionField::new(base, poly, ExtensionLimits::default()).unwrap())
}
fn bits(s: &str) -> Vec<bool> {
    s.bytes()
        .map(|v| match v {
            b'0' => false,
            b'1' => true,
            _ => panic!("fixture"),
        })
        .collect()
}
#[test]
fn python_reference_hamming_all_messages_single_and_double_injection() {
    let mut count = 0;
    let plain = Hamming::new(3, false).unwrap();
    let secded = Hamming::new(3, true).unwrap();
    for line in HAM.lines().filter(|v| !v.starts_with('#')) {
        let col: Vec<_> = line.split_whitespace().collect();
        let m: usize = col[0].parse().unwrap();
        let data = (0..4).map(|i| (m & (1 << i)) != 0).collect::<Vec<_>>();
        assert_eq!(plain.encode(&data).unwrap(), bits(col[1]));
        let encoded = secded.encode(&data).unwrap();
        assert_eq!(encoded, bits(col[2]));
        for a in 0..8 {
            let mut word = encoded.clone();
            word[a] = !word[a];
            let result = secded.decode(&word).unwrap();
            assert_eq!(result.codeword, Some(encoded.clone()));
            assert_eq!(result.message, Some(data.clone()));
            assert_eq!(
                result.status,
                DecodeStatus::Corrected { positions: vec![a] }
            );
            for b in a + 1..8 {
                let mut double = word.clone();
                double[b] = !double[b];
                assert_eq!(
                    secded.decode(&double).unwrap().status,
                    DecodeStatus::DetectedUncorrectable
                );
            }
        }
        count += 1;
    }
    assert_eq!(count, 16);
}
#[test]
fn python_reference_gf5_evaluation_all_25_messages_and_errors() {
    let rs = ReedSolomon::new(RsField::Prime(prime(5)), 4, 2, 200).unwrap();
    let mut count = 0;
    for line in GF5.lines().filter(|v| !v.starts_with('#')) {
        let v: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let message = v[..2]
            .iter()
            .map(|&x| rs.field().ordinal(x).unwrap())
            .collect::<Vec<_>>();
        let expected = v[2..]
            .iter()
            .map(|&x| rs.field().ordinal(x).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(rs.encode(&message).unwrap(), expected);
        for idx in 0..4 {
            let mut wrong = expected.clone();
            wrong[idx] = wrong[idx].add(&rs.field().one()).unwrap();
            let got = rs.decode(&wrong, &[]).unwrap();
            assert_eq!(got.message, Some(message.clone()));
            assert_eq!(got.codeword, Some(expected.clone()));
        }
        count += 1;
    }
    assert_eq!(count, 25);
}
#[test]
fn python_gf16_reference_and_mixed_error_erasure() {
    let rs = ReedSolomon::new(gf16(), 8, 4, 1024).unwrap();
    let mut count = 0;
    for line in GF16.lines().filter(|v| !v.starts_with('#')) {
        let values: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let message = values[..4]
            .iter()
            .map(|&v| rs.field().ordinal(v).unwrap())
            .collect::<Vec<_>>();
        let expected = values[4..]
            .iter()
            .map(|&v| rs.field().ordinal(v).unwrap())
            .collect::<Vec<_>>();
        assert!(
            rs.encode(&message).unwrap() == expected,
            "Independent GF16 polynomial reference row {count}"
        );
        let mut received = expected.clone();
        received[2] = received[2].add(&rs.field().one()).unwrap();
        received[6] = received[6].add(&rs.field().one()).unwrap();
        assert!(
            rs.decode(&received, &[]).unwrap().message == Some(message.clone()),
            "GF16 two hard errors row {count}"
        );
        // 2 hard errors + 1 erasure would exceed n-k=4; independently
        // check a new bounded-radius pattern with 1 hard error + 1 erasure.
        received[6] = expected[6].clone();
        received[0] = rs.field().zero();
        assert!(
            rs.decode(&received, &[0]).unwrap().codeword == Some(expected.clone()),
            "GF16 mixed correctable pattern row {count}"
        );
        count += 1;
    }
    assert_eq!(count, 4);
}
