//! `chrono_now` returns date and time parts with expected punctuation.

use storageshower::system::chrono_now;

#[test]
fn chrono_now_date_and_time_non_empty() {
    let (date, time) = chrono_now();
    assert!(!date.is_empty());
    assert!(!time.is_empty());
}

#[test]
fn chrono_now_matches_prior_shape_checks() {
    let (date, time) = chrono_now();
    assert_eq!(date.chars().filter(|c| *c == '.').count(), 2);
    assert_eq!(time.chars().filter(|c| *c == ':').count(), 2);
}
