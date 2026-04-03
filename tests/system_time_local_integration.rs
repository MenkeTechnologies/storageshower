//! Stable properties of `chrono_now` and `epoch_to_local` (no fixed calendar day — TZ-dependent).

use storageshower::system::{chrono_now, epoch_to_local};

#[test]
fn epoch_to_local_same_epoch_twice_matches() {
    let epoch = 1_728_460_800i64;
    assert_eq!(epoch_to_local(epoch), epoch_to_local(epoch));
}

#[test]
fn epoch_to_local_distinct_epochs_can_differ() {
    let a = epoch_to_local(0);
    let b = epoch_to_local(86_400);
    assert_ne!(a, b);
}

#[test]
fn chrono_now_date_has_two_dots_and_time_has_two_colons() {
    let (date, time) = chrono_now();
    assert_eq!(
        date.chars().filter(|c| *c == '.').count(),
        2,
        "date={date:?}"
    );
    assert_eq!(
        time.chars().filter(|c| *c == ':').count(),
        2,
        "time={time:?}"
    );
}

#[test]
fn chrono_now_date_is_ten_chars_yyyy_mm_dd() {
    let (date, _) = chrono_now();
    assert_eq!(date.len(), 10);
    assert!(date.as_bytes()[4] == b'.' && date.as_bytes()[7] == b'.');
}

#[test]
fn chrono_now_time_is_eight_chars_hh_mm_ss() {
    let (_, time) = chrono_now();
    assert_eq!(time.len(), 8);
}
