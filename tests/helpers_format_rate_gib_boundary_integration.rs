//! `format_rate` just below the Gibibyte/s boundary.

use storageshower::helpers::format_rate;

#[test]
fn just_below_one_gib_per_sec_is_mega_band() {
    let s = format_rate(1_073_741_824.0 - 1.0);
    assert!(s.ends_with("M/s"), "got {s}");
}

#[test]
fn exactly_one_gib_per_sec() {
    assert_eq!(format_rate(1_073_741_824.0), "1.0G/s");
}
