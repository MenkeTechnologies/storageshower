//! `format_uptime` for multi-week durations.

use storageshower::helpers::format_uptime;

#[test]
fn fourteen_days() {
    assert_eq!(format_uptime(14 * 86400), "14d0h0m");
}

#[test]
fn thirty_days_with_hour() {
    assert_eq!(format_uptime(30 * 86400 + 3600), "30d1h0m");
}
