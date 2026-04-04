//! Binary: `-h` and `--help` both exit successfully (`CARGO_BIN_EXE_storageshower`).

use std::process::Command;

fn exe() -> &'static str {
    env!("CARGO_BIN_EXE_storageshower")
}

#[test]
fn short_help_exits_zero() {
    let o = Command::new(exe())
        .arg("-h")
        .output()
        .unwrap_or_else(|e| panic!("spawn: {e}"));
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).len() > 200);
}

#[test]
fn long_help_exits_zero() {
    let o = Command::new(exe())
        .arg("--help")
        .output()
        .unwrap_or_else(|e| panic!("spawn: {e}"));
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).len() > 200);
}
