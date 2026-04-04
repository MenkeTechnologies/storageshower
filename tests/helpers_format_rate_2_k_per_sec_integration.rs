//! `format_rate` at 2 KiB/s (helpers API).

use storageshower::helpers::format_rate;

#[test]
fn two_kib_per_sec() {
    let bps = 2.0 * 1024.0;
    assert_eq!(format_rate(bps), "2.0K/s");
}

#[test]
fn just_below_2_kib_stays_kilo_band() {
    let bps = 2.0 * 1024.0 - 1.0;
    let s = format_rate(bps);
    assert!(s.ends_with("K/s"), "got {s}");
}
