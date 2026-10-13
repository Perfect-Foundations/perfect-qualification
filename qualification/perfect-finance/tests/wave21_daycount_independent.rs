use perfect_finance::{Date, DayCount, Month, Rational};

// Independent Python datetime.date/Fraction day-count oracle, frozen 2026-10-13.
// Columns: start Y/M/D, end Y/M/D, actual days, 30/360 US days, 30E/360 days.
type DayCase = ([i32; 3], [i32; 3], i64, i64, i64);
const CASES: &[DayCase] = &[
    ([2024, 2, 28], [2024, 3, 1], 2, 3, 3),
    ([2024, 2, 29], [2024, 3, 31], 31, 30, 31),
    ([2023, 2, 28], [2023, 3, 31], 31, 30, 32),
    ([2024, 1, 31], [2024, 2, 29], 29, 29, 29),
    ([2024, 2, 29], [2025, 2, 28], 365, 360, 359),
    ([2023, 2, 28], [2024, 2, 29], 366, 360, 361),
    ([2024, 2, 29], [2024, 2, 29], 0, 0, 0),
    ([2024, 3, 31], [2024, 4, 30], 30, 30, 30),
    ([2024, 4, 30], [2024, 5, 31], 31, 30, 30),
    ([2024, 12, 31], [2025, 1, 1], 1, 1, 1),
    ([2023, 12, 31], [2024, 1, 31], 31, 30, 30),
    ([2000, 2, 29], [2000, 3, 1], 1, 1, 2),
    ([1900, 2, 28], [1900, 3, 1], 1, 1, 3),
    ([2024, 2, 28], [2024, 2, 29], 1, 1, 1),
    ([2024, 1, 1], [2025, 1, 1], 366, 360, 360),
    ([2024, 3, 31], [2024, 2, 29], -31, -31, -31),
    ([2024, 2, 29], [2024, 1, 31], -29, -30, -29),
    ([2024, 3, 1], [2024, 2, 29], -1, -2, -2),
    ([2025, 2, 28], [2024, 2, 29], -365, -360, -359),
    ([2024, 2, 29], [2023, 2, 28], -366, -360, -361),
    ([2024, 5, 31], [2024, 4, 30], -31, -30, -30),
    ([2024, 6, 15], [2024, 6, 15], 0, 0, 0),
    ([2024, 6, 15], [2024, 6, 16], 1, 1, 1),
    ([2024, 6, 16], [2024, 6, 15], -1, -1, -1),
];

fn date(parts: [i32; 3]) -> Date {
    Date::from_calendar_date(
        parts[0],
        Month::try_from(parts[1] as u8).unwrap(),
        parts[2] as u8,
    )
    .unwrap()
}
fn exact(n: i64, d: i64) -> Rational {
    Rational::from(n).div(&Rational::from(d)).unwrap()
}
#[test]
fn independently_calculated_day_count_cross_century_and_reversed_boundaries() {
    for &(start, end, actual, us, european) in CASES {
        let (a, b) = (date(start), date(end));
        let cases = [
            (DayCount::Actual360, exact(actual, 360)),
            (DayCount::Actual365Fixed, exact(actual, 365)),
            (DayCount::Thirty360Us, exact(us, 360)),
            (DayCount::ThirtyE360, exact(european, 360)),
        ];
        for (convention, expected) in cases {
            assert_eq!(
                convention.year_fraction(a, b).unwrap(),
                expected,
                "convention={convention:?} start={start:?} end={end:?}"
            );
            assert_eq!(
                convention.year_fraction(a, b).unwrap(),
                expected,
                "repeat determinism {convention:?}"
            );
        }
    }
}
