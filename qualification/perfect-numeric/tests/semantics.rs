use perfect_numeric::{
    Conversion, ConversionError, ConversionLoss, ConversionStatus, Exactness, RangeDirection,
    RoundingMode,
};
use qualify_perfect_numeric::semantic_fingerprint;

const ROUNDING_VECTORS: &str = include_str!("../vectors/rounding.tsv");
const INTEGER_SUMMARY: &str = include_str!("../vectors/integer-summary.txt");

const FLAGS: [ConversionLoss; 5] = [
    ConversionLoss::ROUNDED,
    ConversionLoss::TRUNCATED,
    ConversionLoss::SATURATED,
    ConversionLoss::UNDERFLOWED,
    ConversionLoss::OVERFLOWED,
];

const MODES: [RoundingMode; 5] = [
    RoundingMode::TowardNegative,
    RoundingMode::TowardPositive,
    RoundingMode::TowardZero,
    RoundingMode::NearestTiesToEven,
    RoundingMode::NearestTiesAway,
];

fn loss_from_mask(mask: u8) -> ConversionLoss {
    let mut loss = ConversionLoss::NONE;
    for (bit, flag) in FLAGS.into_iter().enumerate() {
        if mask & (1 << bit) != 0 {
            loss |= flag;
        }
    }
    loss
}

fn checked_u16_to_u8(value: u16) -> Result<Conversion<u8>, ConversionError> {
    match u8::try_from(value) {
        Ok(value) => Ok(Conversion::exact(value)),
        Err(_) => Err(ConversionError::OutOfRange {
            direction: RangeDirection::Above,
        }),
    }
}

fn checked_i16_to_u8(value: i16) -> Result<Conversion<u8>, ConversionError> {
    if value < 0 {
        Err(ConversionError::OutOfRange {
            direction: RangeDirection::Below,
        })
    } else {
        checked_u16_to_u8(value as u16)
    }
}

fn rust_round(value: f64, mode: RoundingMode) -> f64 {
    match mode {
        RoundingMode::TowardNegative => value.floor(),
        RoundingMode::TowardPositive => value.ceil(),
        RoundingMode::TowardZero => value.trunc(),
        RoundingMode::NearestTiesToEven => value.round_ties_even(),
        RoundingMode::NearestTiesAway => value.round(),
    }
}

fn summary_counts(name: &str) -> (usize, usize, usize) {
    for line in INTEGER_SUMMARY.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields[0] == name {
            return (
                fields[1].parse().unwrap(),
                fields[2].parse().unwrap(),
                fields[3].parse().unwrap(),
            );
        }
    }
    panic!("missing retained summary entry: {name}");
}

#[test]
fn all_loss_states_obey_the_public_algebra() {
    for a_mask in 0_u8..32 {
        let a = loss_from_mask(a_mask);
        let a_status = ConversionStatus::from_loss(a);

        assert_eq!(a.is_empty(), a_mask == 0);
        assert_eq!(a_status.is_exact(), a_mask == 0);
        assert_eq!(
            a_status.exactness(),
            if a_mask == 0 {
                Exactness::Exact
            } else {
                Exactness::Inexact
            }
        );

        for (bit, flag) in FLAGS.into_iter().enumerate() {
            assert_eq!(a.contains(flag), a_mask & (1 << bit) != 0);
        }

        for b_mask in 0_u8..32 {
            let b = loss_from_mask(b_mask);
            let union = a.union(b);
            let composed = a_status.compose(ConversionStatus::from_loss(b));

            assert_eq!(union, b.union(a));
            assert_eq!(a.union(a), a);
            assert_eq!(composed.loss(), union);
            assert_eq!(composed.is_exact(), a_mask == 0 && b_mask == 0);

            for c_mask in 0_u8..32 {
                let c = loss_from_mask(c_mask);
                assert_eq!(a.union(b).union(c), a.union(b.union(c)));
            }
        }
    }
}

#[test]
fn exhaustive_primitive_consumer_conversions_match_rust() {
    let mut u_counts = [0_usize; 3];
    for value in u16::MIN..=u16::MAX {
        let ours = checked_u16_to_u8(value);
        let rust = u8::try_from(value);
        assert_eq!(ours.is_ok(), rust.is_ok());

        match ours {
            Ok(value) => {
                assert_eq!(value.into_value(), rust.unwrap());
                assert!(value.status().is_exact());
                u_counts[0] += 1;
            }
            Err(ConversionError::OutOfRange {
                direction: RangeDirection::Above,
            }) => u_counts[2] += 1,
            other => panic!("unexpected u16 conversion classification: {other:?}"),
        }
    }
    assert_eq!(
        (u_counts[0], u_counts[1], u_counts[2]),
        summary_counts("u16_to_u8")
    );

    let mut i_counts = [0_usize; 3];
    for value in i16::MIN..=i16::MAX {
        let ours = checked_i16_to_u8(value);
        let rust = u8::try_from(value);
        assert_eq!(ours.is_ok(), rust.is_ok());

        match ours {
            Ok(value) => {
                assert_eq!(value.into_value(), rust.unwrap());
                assert!(value.status().is_exact());
                i_counts[0] += 1;
            }
            Err(ConversionError::OutOfRange {
                direction: RangeDirection::Below,
            }) => i_counts[1] += 1,
            Err(ConversionError::OutOfRange {
                direction: RangeDirection::Above,
            }) => i_counts[2] += 1,
            other => panic!("unexpected i16 conversion classification: {other:?}"),
        }
    }
    assert_eq!(
        (i_counts[0], i_counts[1], i_counts[2]),
        summary_counts("i16_to_u8")
    );
}

#[test]
fn retained_rounding_vectors_match_documented_modes() {
    let mut checked = 0_usize;

    for line in ROUNDING_VECTORS.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 6, "malformed rounding vector: {line}");
        let input: f64 = fields[0].parse().unwrap();

        for (mode, expected) in MODES.into_iter().zip(&fields[1..]) {
            let expected: f64 = expected.parse().unwrap();
            assert_eq!(rust_round(input, mode), expected);
            checked += 1;
        }
    }

    assert_eq!(checked, 60);
}

#[test]
fn structured_error_categories_remain_machine_actionable() {
    let loss = ConversionLoss::ROUNDED | ConversionLoss::TRUNCATED;

    assert_eq!(
        ConversionError::OutOfRange {
            direction: RangeDirection::Below,
        },
        ConversionError::OutOfRange {
            direction: RangeDirection::Below,
        }
    );
    assert_eq!(
        ConversionError::LossDisallowed { loss },
        ConversionError::LossDisallowed { loss }
    );
    assert_eq!(
        ConversionError::UnsupportedRoundingMode {
            mode: RoundingMode::NearestTiesAway,
        },
        ConversionError::UnsupportedRoundingMode {
            mode: RoundingMode::NearestTiesAway,
        }
    );
}

#[test]
fn qualification_fingerprint_is_stable_within_a_process() {
    assert_eq!(semantic_fingerprint(), semantic_fingerprint());
}
