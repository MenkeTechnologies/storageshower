//! `format_rate` at 4 KiB/s (helpers API).

use storageshower::helpers::format_rate;

#[test]
fn four_kib_per_sec() {
    let bps = 4.0 * 1024.0;
    assert_eq!(format_rate(bps), "4.0K/s");
}

#[test]
fn just_below_4_kib_stays_kilo_band() {
    let bps = 4.0 * 1024.0 - 1.0;
    let s = format_rate(bps);
    assert!(s.ends_with("K/s"), "got {s}");
}
