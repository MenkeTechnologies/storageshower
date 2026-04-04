//! `format_rate` at 256 KiB/s (helpers API).

use storageshower::helpers::format_rate;

#[test]
fn two_fifty_six_kib_per_sec() {
    let bps = 256.0 * 1024.0;
    assert_eq!(format_rate(bps), "256.0K/s");
}

#[test]
fn just_below_256_kib_stays_kilo_band() {
    let bps = 256.0 * 1024.0 - 1.0;
    let s = format_rate(bps);
    assert!(s.ends_with("K/s"), "got {s}");
}
