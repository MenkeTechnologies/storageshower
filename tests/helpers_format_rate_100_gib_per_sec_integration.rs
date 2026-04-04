//! `format_rate` at 100 GiB/s (helpers API).

use storageshower::helpers::format_rate;

#[test]
fn hundred_gib_per_sec() {
    assert_eq!(format_rate(100.0 * 1_073_741_824.0), "100.0G/s");
}
