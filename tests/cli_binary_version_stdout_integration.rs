//! Binary `-V` prints package version on stdout.

use std::process::Command;

fn exe() -> &'static str {
    env!("CARGO_BIN_EXE_storageshower")
}

#[test]
fn dash_v_stdout_contains_cargo_version() {
    let o = Command::new(exe())
        .arg("-V")
        .output()
        .unwrap_or_else(|e| panic!("spawn: {e}"));
    assert!(o.status.success());
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(s.contains(env!("CARGO_PKG_VERSION")), "stdout={s:?}");
}

#[test]
fn long_version_flag_matches_short() {
    let v_short = Command::new(exe())
        .arg("-V")
        .output()
        .unwrap_or_else(|e| panic!("spawn: {e}"));
    let v_long = Command::new(exe())
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("spawn: {e}"));
    assert!(v_short.status.success());
    assert!(v_long.status.success());
    assert_eq!(
        String::from_utf8_lossy(&v_short.stdout),
        String::from_utf8_lossy(&v_long.stdout)
    );
}
