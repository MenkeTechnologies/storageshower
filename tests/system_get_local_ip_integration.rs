//! `get_local_ip` returns a non-empty string (usually an IP).

use storageshower::system::get_local_ip;

#[test]
fn get_local_ip_non_empty_and_digit_or_loopback() {
    let s = get_local_ip();
    assert!(!s.is_empty());
    assert!(
        s.chars().any(|c| c.is_ascii_digit()),
        "expected digits in {s:?}"
    );
}
