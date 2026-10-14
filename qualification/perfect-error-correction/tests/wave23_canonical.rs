use perfect_error_correction::{
    PrimeField, ReedSolomon, RsDecodeOutcome, RsDecodeSession, RsField,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
#[test]
fn canonical_gf7_reed_solomon_values() {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(7) else {
        panic!("prime");
    };
    let f = RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()));
    let code = ReedSolomon::new(f.clone(), 6, 3, 8192).unwrap();
    let mut session = RsDecodeSession::new(code.clone()).unwrap();
    for case in 0..48_u64 {
        let c = [(3 * case + 1) % 7, (5 * case + 2) % 7, (case + 4) % 7];
        let message = c.iter().map(|&v| f.ordinal(v).unwrap()).collect::<Vec<_>>();
        let codeword = code.encode(&message).unwrap();
        let expected = (0..6_u64)
            .map(|x| (c[0] + c[1] * x + c[2] * x * x) % 7)
            .collect::<Vec<_>>();
        for (v, &n) in codeword.iter().zip(&expected) {
            assert_eq!(v, &f.ordinal(n).unwrap());
        }
        let position = case as usize % 6;
        let mut corrupted = codeword.clone();
        corrupted[position] = f.ordinal((expected[position] + 1) % 7).unwrap();
        assert_eq!(
            session.decode(&corrupted, &[]).unwrap(),
            RsDecodeOutcome::Corrected
        );
        assert_eq!(session.last_successful().message, message);
        assert_eq!(session.last_successful().codeword, codeword);
        let values = session
            .last_successful()
            .codeword
            .iter()
            .map(|v| {
                (0..7_u64)
                    .find(|&n| v == &f.ordinal(n).unwrap())
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join(",");
        println!("CANONICAL|RS|{case:02}|{values}|{position}");
    }
}
