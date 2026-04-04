//! `format_rate` at 512 KiB/s (helpers API).

use storageshower::helpers::format_rate;

#[test]
fn five_twelve_kib_per_sec() {
    let bps = 512.0 * 1024.0;
    assert_eq!(format_rate(bps), "512.0K/s");
}

#[test]
fn just_below_512_kib_stays_kilo_band() {
    let bps = 512.0 * 1024.0 - 1.0;
    let s = format_rate(bps);
    assert!(s.ends_with("K/s"), "got {s}");
}
