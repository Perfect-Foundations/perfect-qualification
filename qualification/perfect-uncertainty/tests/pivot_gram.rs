//! Independent exact-dyadic sign-Gram PSD + exact Fraction variance fixtures.
//! Matrices R=S*S^T/16 are mathematically PSD and rank<=16.
use perfect_uncertainty::{CorrelationMatrix, CorrelationPolicy, propagate_first_order};
const FIXTURES: &str = include_str!("../vectors/pivot_gram_v1.tsv");
fn parse_hex(s: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(s, 16).unwrap())
}
#[test]
fn gram_18_24_32_positive_semidefinite_in_both_orders() {
    let mut cases = 0;
    for row in FIXTURES
        .lines()
        .filter(|x| !x.starts_with('#') && !x.is_empty())
    {
        let cols: Vec<&str> = row.split_whitespace().collect();
        let n: usize = cols[0].parse().unwrap();
        assert!(matches!(n, 18 | 24 | 32));
        assert_eq!(cols.len(), 2 * n + 2);
        let masks: Vec<u16> = cols[1..=n].iter().map(|v| v.parse().unwrap()).collect();
        let original: Vec<f64> = cols[1 + n..=2 * n].iter().map(|v| parse_hex(v)).collect();
        let expected = parse_hex(cols[2 * n + 1]);
        assert!(expected > 0.0 && expected.is_finite());
        for backwards in [false, true] {
            let mut entries = vec![0.0; n * n];
            let mut g = vec![0.0; n];
            for i in 0..n {
                let si = if backwards { n - 1 - i } else { i };
                g[i] = original[si];
                for j in 0..n {
                    let sj = if backwards { n - 1 - j } else { j };
                    let equal = 16 - (masks[si] ^ masks[sj]).count_ones() as i32;
                    entries[i * n + j] = f64::from(2 * equal - 16) / 16.0;
                }
            }
            let corr = CorrelationMatrix::new(
                n,
                entries,
                CorrelationPolicy {
                    max_dimension: 32,
                    psd_tolerance: 1e-12,
                },
            )
            .unwrap_or_else(|e| {
                panic!("valid Gram case {cases}, n={n}, backwards={backwards}: {e:?}")
            });
            let result = propagate_first_order(&g, &corr).unwrap();
            let relative = ((result.variance() - expected) / expected).abs();
            assert!(
                relative <= 1e-10,
                "case={cases} n={n} backwards={backwards} got={} exact={expected} relative={relative}",
                result.variance()
            );
        }
        cases += 1;
    }
    assert_eq!(cases, 90);
}
