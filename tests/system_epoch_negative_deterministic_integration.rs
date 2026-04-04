//! `epoch_to_local` is deterministic for negative epochs (integration path).

use storageshower::system::epoch_to_local;

#[test]
fn negative_one_twice_matches() {
    assert_eq!(epoch_to_local(-1), epoch_to_local(-1));
}

#[test]
fn negative_large_twice_matches() {
    let e = -86_400i64 * 400;
    assert_eq!(epoch_to_local(e), epoch_to_local(e));
}
