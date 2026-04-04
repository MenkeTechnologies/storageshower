//! `format_uptime` for multi-day values (helpers API).

use storageshower::helpers::format_uptime;

#[test]
fn seven_days() {
    assert_eq!(format_uptime(7 * 86400), "7d0h0m");
}

#[test]
fn ten_days_with_hours_minutes() {
    assert_eq!(format_uptime(10 * 86400 + 5 * 3600 + 30 * 60), "10d5h30m");
}
