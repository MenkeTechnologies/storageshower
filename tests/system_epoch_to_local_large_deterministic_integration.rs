//! `epoch_to_local` is deterministic for fixed large epochs.

use storageshower::system::epoch_to_local;

#[test]
fn epoch_2025ish_twice_matches() {
    let e = 1_735_689_600i64;
    assert_eq!(epoch_to_local(e), epoch_to_local(e));
}

#[test]
fn epoch_two_thousand_twice_matches() {
    let e = 946_684_800i64;
    assert_eq!(epoch_to_local(e), epoch_to_local(e));
}
