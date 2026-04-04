//! Binary smoke: `toxic-waste` and `chrome-heart` color flags.

use std::process::Command;

fn exe() -> &'static str {
    env!("CARGO_BIN_EXE_storageshower")
}

fn output(args: &[&str]) -> std::process::Output {
    Command::new(exe())
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", exe()))
}

#[test]
fn toxic_waste_no_compact_version() {
    let o = output(&["--color", "toxic-waste", "--no-compact", "-V"]);
    assert!(o.status.success());
}

#[test]
fn chrome_heart_sort_size_help() {
    let o = output(&["--color", "chrome-heart", "-s", "size", "--help"]);
    assert!(o.status.success());
}
