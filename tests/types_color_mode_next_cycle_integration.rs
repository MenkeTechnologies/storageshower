//! `ColorMode::next` wraps and visits only palette variants from `ColorMode::ALL`.

use storageshower::types::ColorMode;

#[test]
fn next_advances_from_default() {
    assert_eq!(ColorMode::Default.next(), ColorMode::ALL[1]);
}

#[test]
fn next_wraps_last_to_first() {
    let last = *ColorMode::ALL.last().expect("ColorMode::ALL non-empty");
    assert_eq!(last.next(), ColorMode::ALL[0]);
}

#[test]
fn every_next_is_in_all() {
    for &m in ColorMode::ALL {
        let n = m.next();
        assert!(
            ColorMode::ALL.contains(&n),
            "{m:?}.next() = {n:?} not in ALL"
        );
    }
}
