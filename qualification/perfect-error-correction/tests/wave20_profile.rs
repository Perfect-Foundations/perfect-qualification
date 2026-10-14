use perfect_error_correction::{
    CodeError, CodingProfile, ExtensionField, ExtensionLimits, Hamming, HammingCodeProfile,
    Natural, PackedBitOrder, PrimeField, ProfileProvenance, ReedSolomon, ReedSolomonCodeProfile,
    RsBatchLimits, RsBatchWord, RsDecodeOutcome, RsField, RsSymbol,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
const GF17: &str = include_str!("../vectors/rs_gf17_n16_v1.tsv");
const GF16: &str = include_str!("../vectors/rs_gf16_v1.tsv");
fn prime(p: u64) -> PrimeField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(p) else {
        panic!("prime");
    };
    PrimeField::new(PrimeModulus::from_proven(proof).unwrap())
}
fn ord(field: &RsField, raw: &[u64]) -> Vec<RsSymbol> {
    raw.iter().map(|&v| field.ordinal(v).unwrap()).collect()
}
#[test]
fn independent_profile_session_application_roundtrip_prime_and_extension() {
    let ham = HammingCodeProfile::new(
        Hamming::new(4, true).unwrap(),
        PackedBitOrder::LeastSignificantFirst,
    )
    .with_documented_reference("informational only")
    .unwrap();
    assert_eq!(
        CodingProfile::Hamming(ham.clone()).provenance(),
        ham.provenance()
    );
    let hmsg = vec![0_u8; ham.message_bits().div_ceil(8)];
    let codeword = ham.encode_bytes(&hmsg).unwrap();
    assert!(ham.decode_block(&codeword).unwrap().message.is_some());

    let rs = ReedSolomon::new(RsField::Prime(prime(17)), 16, 12, 4096).unwrap();
    let p = ReedSolomonCodeProfile::from_code(&rs)
        .with_documented_reference("not CCSDS or cyclic RS")
        .unwrap();
    assert!(matches!(
        p.provenance(),
        ProfileProvenance::DocumentedReference(_)
    ));
    assert_eq!(
        CodingProfile::ReedSolomon(p.clone()).provenance(),
        p.provenance()
    );
    assert_eq!(p.validated_trial_budget(&rs).unwrap(), 4096);
    let nums = GF17
        .lines()
        .find(|s| !s.is_empty() && !s.starts_with('#'))
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(nums.len(), 28);
    let msg = ord(rs.field(), &nums[..12]);
    let expected = ord(rs.field(), &nums[12..]);
    assert_eq!(p.encode_with(&rs, &msg).unwrap(), expected);
    let mut session = p.session(rs).unwrap();
    let mut altered = expected.clone();
    altered[3] = altered[3].add(&session.code().field().one()).unwrap();
    assert_eq!(
        p.decode_session(&mut session, &altered, &[]).unwrap(),
        RsDecodeOutcome::Corrected
    );
    assert_eq!(session.last_successful().codeword, expected);
    let old = session.last_successful().clone();
    let short = &expected[..15];
    let words = [
        RsBatchWord {
            received: &altered,
            erasures: &[],
        },
        RsBatchWord {
            received: short,
            erasures: &[],
        },
        RsBatchWord {
            received: &expected,
            erasures: &[],
        },
    ];
    let limits = RsBatchLimits {
        max_words: 3,
        max_input_items: 48,
        max_result_slots: 200,
    };
    let mut statuses = Vec::new();
    p.decode_batch_session(&mut session, &words, limits, &mut statuses)
        .unwrap();
    assert!(matches!(statuses[0], Ok(RsDecodeOutcome::Corrected)));
    assert!(matches!(statuses[1], Err(CodeError::InvalidLength { .. })));
    assert!(matches!(statuses[2], Ok(RsDecodeOutcome::Clean)));
    assert_eq!(session.batch_outputs()[0].corrected_positions, [3]);
    assert_eq!(*session.last_successful(), old);
    assert!(matches!(
        p.decode_batch_session(
            &mut session,
            &words,
            RsBatchLimits {
                max_words: 2,
                ..limits
            },
            &mut statuses
        ),
        Err(CodeError::BatchLimit { .. })
    ));
    assert_eq!(session.batch_outputs().len(), 3);
    session.release_batch_output_storage();
    assert!(session.batch_outputs().is_empty());
    assert_eq!(session.retained_batch_output_capacity(), 0);

    let base = prime(2);
    let poly = [1_u64, 1, 0, 0, 1]
        .iter()
        .map(|&v| base.element(Natural::from(v)).unwrap())
        .collect();
    let extension =
        RsField::Extension(ExtensionField::new(base, poly, ExtensionLimits::default()).unwrap());
    let next = ReedSolomon::new(extension, 8, 4, 4096).unwrap();
    assert!(matches!(
        p.rebind_session(&mut session, next.clone()),
        Err(CodeError::IncompatibleProfile)
    ));
    assert_eq!(*session.last_successful(), old);
    let second = ReedSolomonCodeProfile::from_code(&next);
    second.rebind_session(&mut session, next).unwrap();
    assert!(session.last_successful().message.is_empty());
    let gf16 = GF16
        .lines()
        .find(|s| !s.is_empty() && !s.starts_with('#'))
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(gf16.len(), 12);
    let emsg = ord(session.code().field(), &gf16[..4]);
    let ecode = ord(session.code().field(), &gf16[4..]);
    assert_eq!(second.encode_with(session.code(), &emsg).unwrap(), ecode);
    let mut ewrong = ecode.clone();
    ewrong[0] = ewrong[0].add(&session.code().field().one()).unwrap();
    assert_eq!(
        second.decode_session(&mut session, &ewrong, &[]).unwrap(),
        RsDecodeOutcome::Corrected
    );
    assert_eq!(session.last_successful().message, emsg);
    assert_eq!(session.last_successful().codeword, ecode);
}
