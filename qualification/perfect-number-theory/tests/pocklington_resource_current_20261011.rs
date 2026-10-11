//! Independent public-API resource and certificate proof classification checks.
//! No source-private helper or production primality routine supplies an oracle.
use perfect_arithmetic::Natural;
use perfect_number_theory::{
    CertificateError, CertificateLimits, NumberTheoryError, PocklingtonFactor, PrimeCertificate,
    verify_prime_certificate,
};
#[test]
fn impossible_prime_power_decline_is_exact_and_bounded() {
    let big = Natural::one().shl_bits(16_384).add(&Natural::one());
    let certificate = PrimeCertificate::pocklington(
        big,
        vec![PocklingtonFactor::new(
            16_385,
            Natural::from(2_u8),
            PrimeCertificate::deterministic_u64(2),
        )],
    );
    assert_eq!(
        verify_prime_certificate(&certificate, CertificateLimits::default()),
        Err(NumberTheoryError::InvalidCertificate(
            CertificateError::FactorProductNotDivisor
        ))
    );
    let huge_exponent = PrimeCertificate::pocklington(
        Natural::from(13_u8),
        vec![PocklingtonFactor::new(
            u32::MAX,
            Natural::from(2_u8),
            PrimeCertificate::deterministic_u64(2),
        )],
    );
    assert_eq!(
        verify_prime_certificate(&huge_exponent, CertificateLimits::default()),
        Err(NumberTheoryError::InvalidCertificate(
            CertificateError::FactorProductNotDivisor
        ))
    );
}
#[test]
fn independently_certified_thirteen_remains_proven_after_preflight() {
    // 13 - 1 = 2^2 * 3; 2^12 = 1 (mod 13),
    // gcd(2^6-1,13)=1 and gcd(2^4-1,13)=1.
    // Every declared prime-power and Pocklington witness is independently
    // certified using the primitive integers 2,3,13.
    let certificate = PrimeCertificate::pocklington(
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
    );
    let proof = verify_prime_certificate(&certificate, CertificateLimits::default()).unwrap();
    assert_eq!(proof.value(), &Natural::from(13_u8));
}
