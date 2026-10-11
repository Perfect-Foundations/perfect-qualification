//! Independent public-API qualification of exact Number Theory M1.
//! Primitive and Python stdlib reference values are retained in vectors.
use perfect_arithmetic::{Integer, Natural};
use perfect_number_theory::{
    CertificateError, CertificateLimits, CompositeEvidence, Modulus, NumberTheoryError,
    PocklingtonFactor, PrimalityResult, PrimeCertificate, Residue, ResourceKind, chinese_remainder,
    classify_u64, miller_rabin_with_bases, verify_prime_certificate,
};

const MODULAR: &str = include_str!("../vectors/modular.tsv");
const CRT: &str = include_str!("../vectors/crt.tsv");
const PRIMES: &str = include_str!("../vectors/primality.tsv");

fn lines(text: &str) -> impl Iterator<Item = &str> {
    text.lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
}
fn u64_of(n: &Natural) -> u64 {
    n.try_to_u64().unwrap().into_value()
}

#[test]
fn exact_modular_arithmetic_matches_independent_python_vectors() {
    let mut count = 0;
    for line in lines(MODULAR) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 10, "malformed modular row {line}");
        let m = fields[0].parse::<u64>().unwrap();
        let a = fields[1].parse::<i64>().unwrap();
        let b = fields[2].parse::<i64>().unwrap();
        let exp = fields[3].parse::<u64>().unwrap();
        let expected: Vec<u64> = fields[4..9].iter().map(|s| s.parse().unwrap()).collect();
        let inverse = fields[9].parse::<i64>().unwrap();
        let modulus = Modulus::new(Natural::from(m)).unwrap();
        let left = Residue::from_integer(Integer::from(a), modulus.clone()).unwrap();
        let right = Residue::from_integer(Integer::from(b), modulus.clone()).unwrap();
        assert_eq!(
            u64_of(left.add(&right).unwrap().representative()),
            expected[0],
            "{line}"
        );
        assert_eq!(
            u64_of(left.sub(&right).unwrap().representative()),
            expected[1],
            "{line}"
        );
        assert_eq!(
            u64_of(left.mul(&right).unwrap().representative()),
            expected[2],
            "{line}"
        );
        assert_eq!(
            u64_of(left.pow(&Natural::from(exp)).unwrap().representative()),
            expected[3],
            "{line}"
        );
        if inverse >= 0 {
            let actual = left.inverse().unwrap();
            assert_eq!(u64_of(actual.representative()), inverse as u64, "{line}");
        } else {
            assert_eq!(
                left.inverse(),
                Err(NumberTheoryError::NonInvertible {
                    gcd: Natural::from(expected[4])
                }),
                "{line}",
            );
        }
        count += 1;
    }
    assert_eq!(count, 300);
}

#[test]
fn crt_matches_independent_primitive_congruence_enumeration() {
    let mut count = 0;
    for line in lines(CRT) {
        let fields: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(fields.len(), 5, "malformed CRT row {line}");
        let (m, a, n, b, expected) = (fields[0], fields[1], fields[2], fields[3], fields[4]);
        let residues = [
            Residue::from_natural(Natural::from(a), Modulus::new(Natural::from(m)).unwrap())
                .unwrap(),
            Residue::from_natural(Natural::from(b), Modulus::new(Natural::from(n)).unwrap())
                .unwrap(),
        ];
        let observed = chinese_remainder(&residues).unwrap();
        assert_eq!(u64_of(observed.representative()), expected, "{line}");
        assert_eq!(u64_of(observed.modulus().value()), m * n, "{line}");
        count += 1;
    }
    assert_eq!(count, 189);
}

#[test]
fn deterministic_u64_proofs_match_independent_prime_reference() {
    let mut count = 0;
    for line in lines(PRIMES) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 2, "malformed primality row {line}");
        let n = fields[0].parse::<u64>().unwrap();
        let observed = classify_u64(n);
        match (fields[1], observed) {
            ("P", PrimalityResult::ProvenPrime(proof)) => {
                assert_eq!(proof.value(), &Natural::from(n), "{line}");
            }
            ("C", PrimalityResult::Composite(_)) => {}
            (expected, other) => {
                panic!("primality contract violated n={n} expected={expected} actual={other:?}")
            }
        }
        count += 1;
    }
    assert_eq!(count, 4103);
}

fn certificate_thirteen() -> PrimeCertificate {
    PrimeCertificate::pocklington(
        Natural::from(13_u8),
        vec![
            PocklingtonFactor::new(
                2,
                Natural::from(2_u8),
                PrimeCertificate::deterministic_u64(2),
            ),
            PocklingtonFactor::new(
                1,
                Natural::from(2_u8),
                PrimeCertificate::deterministic_u64(3),
            ),
        ],
    )
}

#[test]
fn probable_prime_class_never_becomes_mathematical_proof() {
    let candidate = Natural::from(2047_u64);
    let classification = miller_rabin_with_bases(&candidate, &[Natural::from(2_u8)]).unwrap();
    assert!(matches!(classification, PrimalityResult::ProbablePrime(_)));
    assert!(matches!(classify_u64(2047), PrimalityResult::Composite(_)));
}

#[test]
fn valid_and_adversarial_pocklington_proofs_obey_typed_limits() {
    let certificate = certificate_thirteen();
    let proven = verify_prime_certificate(&certificate, CertificateLimits::default()).unwrap();
    assert_eq!(proven.value(), &Natural::from(13_u8));

    assert_eq!(
        verify_prime_certificate(
            &PrimeCertificate::deterministic_u64(4),
            CertificateLimits::default()
        ),
        Err(NumberTheoryError::InvalidCertificate(
            CertificateError::DeterministicLeafComposite
        ))
    );
    assert_eq!(
        verify_prime_certificate(&certificate, CertificateLimits::new(64, 1, 64)),
        Err(NumberTheoryError::ResourceLimitExceeded {
            resource: ResourceKind::CertificateFactors,
            limit: 1
        })
    );
    assert_eq!(
        verify_prime_certificate(&certificate, CertificateLimits::new(64, 4_096, 3)),
        Err(NumberTheoryError::ResourceLimitExceeded {
            resource: ResourceKind::CandidateBits,
            limit: 3
        })
    );
    let invalid_witness = PrimeCertificate::pocklington(
        Natural::from(13_u8),
        vec![PocklingtonFactor::new(
            2,
            Natural::one(),
            PrimeCertificate::deterministic_u64(2),
        )],
    );
    assert_eq!(
        verify_prime_certificate(&invalid_witness, CertificateLimits::default()),
        Err(NumberTheoryError::InvalidCertificate(
            CertificateError::WitnessOutOfRange
        ))
    );
}

#[test]
fn empty_and_non_coprime_crt_fail_with_structured_errors() {
    assert_eq!(
        chinese_remainder(&[]),
        Err(NumberTheoryError::EmptyCrtSystem)
    );
    let left = Residue::from_natural(
        Natural::from(1_u8),
        Modulus::new(Natural::from(6_u8)).unwrap(),
    )
    .unwrap();
    let right = Residue::from_natural(
        Natural::from(3_u8),
        Modulus::new(Natural::from(9_u8)).unwrap(),
    )
    .unwrap();
    assert_eq!(
        chinese_remainder(&[left, right]),
        Err(NumberTheoryError::NonCoprimeModuli {
            gcd: Natural::from(3_u8)
        })
    );
    assert!(matches!(
        classify_u64(0),
        PrimalityResult::Composite(CompositeEvidence::BelowTwo)
    ));
}
