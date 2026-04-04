//! `get_username` matches `USER` / `USERNAME` when set (typical on CI and dev machines).

use storageshower::system::get_username;

#[test]
fn get_username_matches_env_when_present() {
    let from_env = std::env::var("USER").or_else(|_| std::env::var("USERNAME"));
    if let Ok(expect) = from_env {
        assert_eq!(get_username().as_deref(), Some(expect.as_str()));
    }
}
