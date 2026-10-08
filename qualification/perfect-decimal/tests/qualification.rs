use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use perfect_arithmetic::{Integer, Natural};
use perfect_decimal::{
    Decimal, DecimalContext, DecimalError, DecimalExponent, DecimalLimits, DecimalPrecision,
};
use perfect_numeric::RoundingMode;
use perfect_rational::Rational;

fn limits() -> DecimalLimits {
    DecimalLimits::new(64).unwrap()
}
fn make(coefficient: i32, exponent: i64) -> Decimal {
    Decimal::from_parts(
        coefficient.is_negative(),
        Natural::from(u64::from(coefficient.unsigned_abs())),
        DecimalExponent::new(exponent),
    )
}
fn reference(value: i128, exponent: i64, negative_zero: bool) -> Decimal {
    let negative = value < 0 || (value == 0 && negative_zero);
    Decimal::from_parts(
        negative,
        Natural::from(value.unsigned_abs()),
        DecimalExponent::new(exponent),
    )
}
#[test]
fn independent_primitive_integer_oracle_add_sub_mul_and_order() {
    // i128 powers-of-ten are independent of Decimal's arbitrary-precision algorithms.
    for a in -12_i32..=12 {
        for b in -12_i32..=12 {
            for ea in -2_i64..=2 {
                for eb in -2_i64..=2 {
                    let da = make(a, ea);
                    let db = make(b, eb);
                    let target = ea.min(eb);
                    let va = i128::from(a) * 10_i128.pow(u32::try_from(ea - target).unwrap());
                    let vb = i128::from(b) * 10_i128.pow(u32::try_from(eb - target).unwrap());
                    let sum = da.add(&db, limits()).unwrap();
                    let difference = da.sub(&db, limits()).unwrap();
                    let product = da.mul(&db, limits()).unwrap();
                    assert!(
                        sum.same_representation(&reference(va + vb, target, false)),
                        "{a}e{ea} + {b}e{eb}"
                    );
                    assert!(
                        difference.same_representation(&reference(va - vb, target, false)),
                        "{a}e{ea} - {b}e{eb}"
                    );
                    assert!(
                        product.same_representation(&reference(
                            i128::from(a) * i128::from(b),
                            ea + eb,
                            (a < 0) ^ (b < 0)
                        )),
                        "{a}e{ea} * {b}e{eb}"
                    );
                    assert_eq!(da.cmp(&db), va.cmp(&vb));
                }
            }
        }
    }
}
#[test]
fn public_rational_interoperability_is_exact_or_explicit() {
    for numerator in -25_i32..=25 {
        for denominator in 1_i32..=25 {
            let rational =
                Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap();
            match Decimal::from_rational_exact(&rational, limits()) {
                Ok(decimal) => assert_eq!(decimal.to_rational(limits()).unwrap(), rational),
                Err(DecimalError::NonTerminatingDecimal) => {
                    let mut d = denominator;
                    let gcd = gcd_u32(numerator.unsigned_abs(), denominator as u32);
                    d /= i32::try_from(gcd).unwrap();
                    while d % 2 == 0 {
                        d /= 2;
                    }
                    while d % 5 == 0 {
                        d /= 5;
                    }
                    assert_ne!(d, 1);
                }
                other => panic!("unanticipated Rational result: {other:?}"),
            }
        }
    }
}
fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
fn mode(token: &str) -> RoundingMode {
    match token {
        "ZERO" => RoundingMode::TowardZero,
        "NEG" => RoundingMode::TowardNegative,
        "POS" => RoundingMode::TowardPositive,
        "EVEN" => RoundingMode::NearestTiesToEven,
        "AWAY" => RoundingMode::NearestTiesAway,
        _ => panic!("invalid vector mode"),
    }
}
#[test]
fn python_decimal_quantization_vectors_validate_public_rounding_and_relation() {
    const VECTORS: &str = include_str!("../vectors/quantize.tsv");
    let mut checked = 0_usize;
    for line in VECTORS
        .lines()
        .filter(|x| !x.starts_with('#') && !x.trim().is_empty())
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 5, "invalid qualification row: {line}");
        let input = Decimal::parse(fields[0], limits()).unwrap();
        let target: i64 = fields[1].parse().unwrap();
        let coefficient: u64 = fields[3].parse().unwrap();
        let expected = Decimal::from_parts(
            input.is_negative(),
            Natural::from(coefficient),
            DecimalExponent::new(target),
        );
        let relation = match fields[4] {
            "L" => Ordering::Less,
            "G" => Ordering::Greater,
            "E" => Ordering::Equal,
            _ => panic!("invalid relation"),
        };
        let actual = input
            .quantize(DecimalExponent::new(target), mode(fields[2]), limits())
            .unwrap();
        assert!(
            actual.value().same_representation(&expected),
            "input={} target={} mode={}",
            fields[0],
            target,
            fields[2]
        );
        assert_eq!(actual.relation(), relation);
        checked += 1;
    }
    assert!(
        checked >= 120,
        "inadequate external quantization vectors: {checked}"
    );
}
#[derive(Default)]
struct ByteHash(Vec<u8>);
impl Hasher for ByteHash {
    fn finish(&self) -> u64 {
        0
    }
    fn write(&mut self, b: &[u8]) {
        self.0.extend_from_slice(b)
    }
}
fn bytes(value: &Decimal) -> Vec<u8> {
    let mut h = ByteHash::default();
    value.hash(&mut h);
    h.0
}
#[test]
fn numeric_hash_zero_quantum_and_representation_contract() {
    for a in ["1", "10e-1", "100e-2", "1000e-3"] {
        assert_eq!(Decimal::parse(a, limits()).unwrap(), make(1, 0));
        assert_eq!(
            bytes(&Decimal::parse(a, limits()).unwrap()),
            bytes(&make(1, 0))
        );
    }
    for z in ["0", "-0", "0e-100", "-0e100"] {
        let d = Decimal::parse(z, limits()).unwrap();
        assert_eq!(d, Decimal::zero());
        assert_eq!(bytes(&d), bytes(&Decimal::zero()));
    }
}
#[test]
fn extreme_exponent_failures_are_bounded_without_materialization() {
    let huge = Decimal::from_parts(false, Natural::one(), DecimalExponent::new(i64::MAX));
    let tiny = Decimal::from_parts(false, Natural::one(), DecimalExponent::new(i64::MIN));
    assert!(matches!(
        huge.add(&tiny, limits()),
        Err(DecimalError::CoefficientLimitExceeded { .. })
    ));
    assert!(
        tiny.quantize(
            DecimalExponent::new(i64::MAX),
            RoundingMode::TowardZero,
            limits()
        )
        .is_ok()
    );
    assert_eq!(
        Decimal::parse("1e9223372036854775808", limits()),
        Err(DecimalError::ExponentOutOfRange)
    );
}
#[test]
fn rounded_ratio_reports_direction() {
    let context = DecimalContext::new(
        DecimalPrecision::new(2).unwrap(),
        RoundingMode::TowardZero,
        limits(),
    )
    .unwrap();
    let one = Decimal::one();
    let three = Decimal::from(3_u8);
    let r = one.div(&three, context).unwrap();
    assert_eq!(r.value(), &Decimal::parse("33e-2", limits()).unwrap());
    assert_eq!(r.relation(), Ordering::Less);
    let neg = one.neg().div(&three, context).unwrap();
    assert_eq!(neg.relation(), Ordering::Greater);
}

#[test]
fn independent_cross_platform_decimal_representation_fingerprint() {
    const VECTORS: &str = include_str!("../vectors/representation.tsv");
    const FINGERPRINT: &str = include_str!("../semantic-fingerprint.txt");
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    let mut checked = 0_usize;
    for line in VECTORS
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let cols: Vec<_> = line.split_whitespace().collect();
        assert_eq!(cols.len(), 3, "invalid representation vector: {line}");
        let parsed = Decimal::parse(cols[0], limits()).unwrap();
        let original = parsed.to_string();
        let normalized = parsed.normalized().unwrap().to_string();
        assert_eq!(original, cols[1], "source={}", cols[0]);
        assert_eq!(normalized, cols[2], "source={}", cols[0]);
        for byte in original
            .bytes()
            .chain(core::iter::once(b'|'))
            .chain(normalized.bytes())
            .chain(core::iter::once(b'\n'))
        {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
        checked += 1;
    }
    assert_eq!(checked, 21);
    assert_eq!(format!("{hash:016x}"), FINGERPRINT.trim());
}
