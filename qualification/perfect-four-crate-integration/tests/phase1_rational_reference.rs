//! Qualification-owned reference for Rational current main at 21a7fba5f22ab7f5db12b480a1c910edd1797d77.
//! Adapted from Rational tests/phase65_bounded_cmp.rs; expected results
//! use the independent num-rational / num-bigint oracle.
use core::cmp::Ordering;
use num_bigint::BigInt;
use num_rational::BigRational;
use perfect_arithmetic::{Integer, Natural};
use perfect_rational::Rational;

fn perfect_num(bits: u64, salt: u64) -> Natural {
    Natural::one()
        .shl_bits(bits - 1)
        .add(&Natural::one().shl_bits(bits / 2))
        .add(&Natural::from(salt))
}
fn reference_num(bits: u64, salt: u64) -> BigInt {
    (BigInt::from(1u8) << usize::try_from(bits - 1).unwrap())
        + (BigInt::from(1u8) << usize::try_from(bits / 2).unwrap())
        + BigInt::from(salt)
}
fn signed(n: Natural, neg: bool) -> Integer {
    let v = Integer::from(n);
    if neg { v.neg() } else { v }
}
fn reference_signed(n: BigInt, neg: bool) -> BigInt {
    if neg { -n } else { n }
}
#[test]
fn randomized_large_fraction_order_matches_independent_reference() {
    let mut cases = 0usize;
    for width in [128_u64, 512, 2048, 4096, 8192, 16384] {
        for k in 0..24_u64 {
            let widths = [
                width,
                width - 1 + (k % 2),
                width - 2 + (k % 3),
                width - 3 + (k % 4),
            ];
            let salts = [k * 193 + 17, k * 53 + 71, k * 113 + 11, k * 79 + 103];
            for sign_code in 0..4 {
                let left = Rational::new(
                    signed(perfect_num(widths[0], salts[0]), sign_code & 1 != 0),
                    signed(perfect_num(widths[1], salts[1]), false),
                )
                .unwrap();
                let right = Rational::new(
                    signed(perfect_num(widths[2], salts[2]), sign_code & 2 != 0),
                    signed(perfect_num(widths[3], salts[3]), false),
                )
                .unwrap();
                let refleft = BigRational::new(
                    reference_signed(reference_num(widths[0], salts[0]), sign_code & 1 != 0),
                    reference_num(widths[1], salts[1]),
                );
                let refright = BigRational::new(
                    reference_signed(reference_num(widths[2], salts[2]), sign_code & 2 != 0),
                    reference_num(widths[3], salts[3]),
                );
                let expected = refleft.cmp(&refright);
                assert_eq!(
                    left.cmp(&right),
                    expected,
                    "width={width}, k={k}, sign={sign_code}"
                );
                assert_eq!(right.cmp(&left), expected.reverse());
                cases += 2;
            }
        }
    }
    assert_eq!(cases, 1152);
}

#[test]
fn continued_fraction_common_prefix_and_ties_preserve_order() {
    let mut pairs = 0;
    for bits in [512u64, 1024, 4096] {
        let a = perfect_num(bits, 73);
        let b = perfect_num(bits - 1, 17);
        let rational = Rational::new(Integer::from(a.clone()), Integer::from(b.clone())).unwrap();
        let same = Rational::new(
            Integer::from(a.mul(&Natural::from(7_u8))),
            Integer::from(b.mul(&Natural::from(7_u8))),
        )
        .unwrap();
        assert_eq!(rational.cmp(&same), Ordering::Equal);
        for step in 0..16u64 {
            let n = a.add(&Natural::from(step + 1));
            let increased = Rational::new(Integer::from(n), Integer::from(b.clone())).unwrap();
            assert_eq!(increased.cmp(&rational), Ordering::Greater);
            assert_eq!(rational.cmp(&increased), Ordering::Less);
            pairs += 1;
        }
    }
    assert_eq!(pairs, 48);
}

#[test]
fn long_common_quotient_prefix_uses_exact_fallback() {
    for steps in [120_usize, 380, 750, 1400] {
        let (mut a, mut b) = (Natural::one(), Natural::one());
        let (mut ra, mut rb) = (BigInt::from(1_u8), BigInt::from(1_u8));
        for _ in 0..steps {
            let next = a.add(&b);
            a = b;
            b = next;
            let reference_next = &ra + &rb;
            ra = rb;
            rb = reference_next;
        }
        let left = Rational::new(Integer::from(b.clone()), Integer::from(a.clone())).unwrap();
        let right = Rational::new(Integer::from(a.add(&b)), Integer::from(b.clone())).unwrap();
        let left_ref = BigRational::new(rb.clone(), ra.clone());
        let right_ref = BigRational::new(&ra + &rb, rb.clone());
        let expected = left_ref.cmp(&right_ref);
        assert_eq!(left.cmp(&right), expected, "steps={steps}");
        assert_eq!(right.cmp(&left), expected.reverse(), "steps={steps}");
        let negative_left = Rational::new(Integer::from(b).neg(), Integer::from(a)).unwrap();
        assert_eq!(negative_left.cmp(&right), Ordering::Less);
    }
}
