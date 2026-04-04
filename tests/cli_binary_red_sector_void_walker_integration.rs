//! Binary smoke: `red` (Red Sector) and `void-walker` color flags.

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
fn red_units_mib_version() {
    let o = output(&["--color", "red", "-u", "mib", "-V"]);
    assert!(o.status.success());
}

#[test]
fn void_walker_list_colors() {
    let o = output(&["--color", "void-walker", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}
