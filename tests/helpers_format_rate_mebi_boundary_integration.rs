//! `format_rate` just below the Mebibyte/s boundary.

use storageshower::helpers::format_rate;

#[test]
fn just_below_one_mebib_per_sec_is_kilo_band() {
    let s = format_rate(1_048_576.0 - 1.0);
    assert!(s.ends_with("K/s"), "got {s}");
}

#[test]
fn exactly_one_mebib_per_sec() {
    assert_eq!(format_rate(1_048_576.0), "1.0M/s");
}
