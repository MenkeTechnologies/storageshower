//! Binary smoke: `bio-hazard` and `megacorp` color flags.

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
fn bio_hazard_list_colors() {
    let o = output(&["--color", "bio-hazard", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn megacorp_local_only_version() {
    let o = output(&["--color", "megacorp", "-l", "-V"]);
    assert!(o.status.success());
}
