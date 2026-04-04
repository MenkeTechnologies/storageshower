//! `get_battery` is callable without panicking (returns `None` on unsupported hosts).

use storageshower::system::get_battery;

#[test]
fn get_battery_does_not_panic() {
    let _ = get_battery();
}
