use perfect_algebra::{AlgebraError, ExtensionField, ExtensionLimits, PrimeField};
use perfect_arithmetic::Natural;
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
const PRIME: &str = include_str!("../vectors/prime_v1.tsv");
const EXT: &str = include_str!("../vectors/extension_v1.tsv");
fn field(p: u64) -> PrimeField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(p) else {
        panic!("reference prime");
    };
    PrimeField::new(PrimeModulus::from_proven(proof).unwrap())
}
fn extension(id: &str) -> ExtensionField {
    let (p, poly) = match id {
        "g22" => (2, &[1_u64, 1, 1][..]),
        "g23" => (2, &[1, 1, 0, 1][..]),
        "g32" => (3, &[1, 0, 1][..]),
        _ => panic!("invalid fixture family"),
    };
    let base = field(p);
    let v = poly
        .iter()
        .map(|&x| base.element(Natural::from(x)).unwrap())
        .collect();
    ExtensionField::new(base, v, ExtensionLimits::default()).unwrap()
}
fn element(f: &ExtensionField, mut n: u64) -> perfect_algebra::ExtensionFieldElement {
    let p = f.base().characteristic().try_to_u64().unwrap().into_value();
    let mut v = Vec::new();
    for _ in 0..f.degree() {
        v.push(f.base().element(Natural::from(n % p)).unwrap());
        n /= p;
    }
    f.element(v).unwrap()
}
fn fingerprint<T: Hash>(x: &T) -> u64 {
    let mut state = DefaultHasher::new();
    x.hash(&mut state);
    state.finish()
}
#[test]
fn all_independent_prime_field_integer_vectors() {
    let mut count = 0_usize;
    for row in PRIME.lines().filter(|x| !x.starts_with('#')) {
        let n: Vec<u64> = row.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let [p, a, b, plus, minus, times]: [u64; 6] = n.try_into().unwrap();
        let f = field(p);
        let a = f.element(Natural::from(a)).unwrap();
        let b = f.element(Natural::from(b)).unwrap();
        assert_eq!(a.add(&b).unwrap().representative(), &Natural::from(plus));
        assert_eq!(a.sub(&b).unwrap().representative(), &Natural::from(minus));
        assert_eq!(a.mul(&b).unwrap().representative(), &Natural::from(times));
        count += 1;
    }
    assert_eq!(count, 208);
}
#[test]
fn all_independent_extension_polynomial_vectors() {
    let mut count = 0_usize;
    for row in EXT.lines().filter(|x| !x.starts_with('#')) {
        let parts: Vec<&str> = row.split_whitespace().collect();
        let id = parts[0];
        let numbers: Vec<u64> = parts[1..].iter().map(|x| x.parse().unwrap()).collect();
        let [p, a, b, plus, times]: [u64; 5] = numbers.try_into().unwrap();
        let f = extension(id);
        assert_eq!(f.base().characteristic(), &Natural::from(p));
        let a = element(&f, a);
        let b = element(&f, b);
        assert_eq!(a.add(&b).unwrap(), element(&f, plus));
        assert_eq!(a.mul(&b).unwrap(), element(&f, times));
        if !a.is_zero() {
            assert_eq!(a.mul(&a.inverse().unwrap()).unwrap(), f.one());
        }
        count += 1;
    }
    assert_eq!(count, 161);
}
#[test]
fn proof_structural_parent_and_reducibility_boundaries() {
    let a = field(7);
    let b = field(7);
    assert_eq!(a, b);
    assert_eq!(fingerprint(&a), fingerprint(&b));
    assert_eq!(a.one(), b.one());
    let c = field(5);
    assert!(matches!(
        a.one().add(&c.one()),
        Err(AlgebraError::IncompatibleParent)
    ));
    let f = field(2);
    for poly in [&[1_u64, 0, 1][..], &[1, 0, 1, 0, 1][..]] {
        let v = poly
            .iter()
            .map(|&x| f.element(Natural::from(x)).unwrap())
            .collect();
        assert!(matches!(
            ExtensionField::new(f.clone(), v, ExtensionLimits::default()),
            Err(AlgebraError::ReduciblePolynomial)
        ));
    }
    let x = extension("g22");
    let y = extension("g23");
    assert!(matches!(
        x.one().add(&y.one()),
        Err(AlgebraError::IncompatibleParent)
    ));
}

#[test]
fn refusal_for_oversized_element_and_extension_degree() {
    let field = field(2);
    let polynomial = [1_u8, 1, 1]
        .iter()
        .map(|&v| field.element(Natural::from(v)).unwrap())
        .collect();
    let limited = ExtensionLimits {
        max_degree: 1,
        ..ExtensionLimits::default()
    };
    assert!(matches!(
        ExtensionField::new(field.clone(), polynomial, limited),
        Err(AlgebraError::ResourceLimit {
            dimension: "degree",
            ..
        })
    ));
    let gf4 = extension("g22");
    let oversized = vec![field.one(); 65];
    assert!(matches!(
        gf4.element(oversized),
        Err(AlgebraError::ResourceLimit {
            dimension: "input_coefficients",
            ..
        })
    ));
    assert!(matches!(
        gf4.zero().inverse(),
        Err(AlgebraError::InverseOfZero)
    ));
}

#[test]
fn shared_parent_keeps_exact_independent_field_semantics() {
    let x = extension("g23");
    let y = extension("g23");
    assert_eq!(x, y);
    assert_eq!(fingerprint(&x), fingerprint(&y));
    let a = element(&x, 3);
    let b = element(&y, 5);
    assert_eq!(a.add(&b).unwrap(), element(&x, 6));
    let zero = x.element(vec![x.base().zero(); 64]).unwrap();
    assert_eq!(zero, x.zero());
    let f = extension("g22");
    assert!(matches!(
        x.one().mul(&f.one()),
        Err(AlgebraError::IncompatibleParent)
    ));
}
