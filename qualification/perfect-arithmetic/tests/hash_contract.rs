//! Independent qualification of the public, versioned canonical Hash byte stream.
//! Expected streams come from Python's integer arithmetic, not crate internals.
use core::hash::{Hash, Hasher};
use perfect_arithmetic::{Integer, Natural};

const VECTORS: &str = include_str!("../vectors/canonical_hash.tsv");

#[derive(Default)]
struct ByteCapture(Vec<u8>);
impl Hasher for ByteCapture {
    fn finish(&self) -> u64 {
        0
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }
}
#[test]
fn canonical_public_hash_bytes_match_independent_integer_vectors() {
    let mut checked = 0_usize;
    for line in VECTORS
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 5, "malformed vector: {line}");
        let kind = fields[0];
        let sign = fields[1];
        let shift: i64 = fields[2].parse().unwrap();
        let added: u64 = fields[3].parse().unwrap();
        let magnitude = if shift == -1 {
            Natural::zero()
        } else {
            assert!(shift >= 0);
            Natural::one().shl_bits(shift as u64)
        }
        .add(&Natural::from(added));
        let mut capture = ByteCapture::default();
        match kind {
            "N" => {
                assert_eq!(sign, "+");
                magnitude.hash(&mut capture);
            }
            "I" => {
                let integer = Integer::from(magnitude);
                match sign {
                    "+" => integer.hash(&mut capture),
                    "-" => integer.neg().hash(&mut capture),
                    _ => panic!("unknown sign: {sign}"),
                }
            }
            _ => panic!("unknown kind: {kind}"),
        };
        let observed: String = capture.0.iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(observed, fields[4], "hash stream differs for: {line}");
        checked += 1;
    }
    assert_eq!(checked, 25);
}
