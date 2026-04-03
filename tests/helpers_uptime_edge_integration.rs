//! `format_uptime` edge cases from the public `helpers` API.

use storageshower::helpers::format_uptime;

#[test]
fn one_second_below_one_minute() {
    assert_eq!(format_uptime(59), "0m");
}

#[test]
fn exactly_one_minute() {
    assert_eq!(format_uptime(60), "1m");
}

#[test]
fn fifty_nine_minutes_no_hours() {
    assert_eq!(format_uptime(59 * 60), "59m");
}

#[test]
fn exactly_one_hour() {
    assert_eq!(format_uptime(3600), "1h0m");
}

#[test]
fn one_hour_one_minute() {
    assert_eq!(format_uptime(3600 + 60), "1h1m");
}

#[test]
fn twenty_three_hours_fifty_nine_minutes() {
    assert_eq!(format_uptime(23 * 3600 + 59 * 60), "23h59m");
}

#[test]
fn one_day_exactly() {
    assert_eq!(format_uptime(86400), "1d0h0m");
}

#[test]
fn one_day_one_hour_zero_minutes() {
    assert_eq!(format_uptime(86400 + 3600), "1d1h0m");
}

#[test]
fn two_days_with_hours_and_minutes() {
    assert_eq!(format_uptime(2 * 86400 + 3 * 3600 + 15 * 60), "2d3h15m");
}

#[test]
fn large_uptime_no_panic() {
    let _ = format_uptime(u64::MAX / 86400 * 86400);
}
