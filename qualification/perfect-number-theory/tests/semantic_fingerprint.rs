//! Deterministic semantic fingerprint of *observed* public API results.
//! Expected digest is generated independently by Python stdlib, never by the SUT.
use perfect_arithmetic::{Integer, Natural};
use perfect_number_theory::{
    Modulus, NumberTheoryError, PrimalityResult, Residue, chinese_remainder, classify_u64,
};

const MODULAR: &str = include_str!("../vectors/modular.tsv");
const CRT: &str = include_str!("../vectors/crt.tsv");
const PRIMALITY: &str = include_str!("../vectors/primality.tsv");
const EXPECTED: &str = include_str!("../semantic-fingerprint.txt");

fn rows(contents: &str) -> impl Iterator<Item = &str> {
    contents
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
}

fn native(n: &Natural) -> u64 {
    n.try_to_u64().unwrap().into_value()
}

fn update(state: &mut u64, text: &str) {
    for byte in text.bytes() {
        *state = (*state ^ u64::from(byte)).wrapping_mul(0x100000001b3);
    }
}

#[test]
fn observed_public_outputs_match_cross_platform_reference_fingerprint() {
    let mut hash = 0xcbf29ce484222325_u64;
    let mut count = 0_usize;
    for row in rows(MODULAR) {
        let fields: Vec<&str> = row.split_ascii_whitespace().collect();
        assert_eq!(fields.len(), 10);
        let modulus = fields[0].parse::<u64>().unwrap();
        let a = fields[1].parse::<i64>().unwrap();
        let b = fields[2].parse::<i64>().unwrap();
        let exp = fields[3].parse::<u64>().unwrap();
        let m = Modulus::new(Natural::from(modulus)).unwrap();
        let x = Residue::from_integer(Integer::from(a), m.clone()).unwrap();
        let y = Residue::from_integer(Integer::from(b), m).unwrap();
        let (gcd, inverse) = match x.inverse() {
            Ok(value) => (1_u64, native(value.representative()) as i64),
            Err(NumberTheoryError::NonInvertible { gcd }) => (native(&gcd), -1_i64),
            Err(error) => panic!("unexpected modular inverse failure: {error:?}"),
        };
        let observed = format!(
            "{modulus} {a} {b} {exp} {} {} {} {} {gcd} {inverse}\n",
            native(x.add(&y).unwrap().representative()),
            native(x.sub(&y).unwrap().representative()),
            native(x.mul(&y).unwrap().representative()),
            native(x.pow(&Natural::from(exp)).unwrap().representative())
        );
        assert_eq!(observed.trim_end(), row);
        update(&mut hash, &observed);
        count += 1;
    }
    for row in rows(CRT) {
        let values: Vec<u64> = row
            .split_ascii_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();
        assert_eq!(values.len(), 5);
        let (m, a, n, b) = (values[0], values[1], values[2], values[3]);
        let system = [
            Residue::from_natural(Natural::from(a), Modulus::new(Natural::from(m)).unwrap())
                .unwrap(),
            Residue::from_natural(Natural::from(b), Modulus::new(Natural::from(n)).unwrap())
                .unwrap(),
        ];
        let observed = chinese_remainder(&system).unwrap();
        let line = format!("{m} {a} {n} {b} {}\n", native(observed.representative()));
        assert_eq!(line.trim_end(), row);
        update(&mut hash, &line);
        count += 1;
    }
    for row in rows(PRIMALITY) {
        let mut parts = row.split_ascii_whitespace();
        let candidate = parts.next().unwrap().parse::<u64>().unwrap();
        let class = match classify_u64(candidate) {
            PrimalityResult::ProvenPrime(proof) => {
                assert_eq!(proof.value(), &Natural::from(candidate));
                "P"
            }
            PrimalityResult::Composite(_) => "C",
            PrimalityResult::ProbablePrime(_) => {
                panic!("deterministic u64 proof downgraded to probable prime")
            }
        };
        let line = format!("{candidate} {class}\n");
        assert_eq!(line.trim_end(), row);
        update(&mut hash, &line);
        count += 1;
    }
    assert_eq!(count, 300 + 189 + 4103);
    assert_eq!(format!("{hash:016x}"), EXPECTED.trim());
}
