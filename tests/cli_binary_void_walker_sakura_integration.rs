//! Binary smoke: `void-walker` and `sakura` color flags.

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
fn void_walker_show_all_version() {
    let o = output(&["--color", "void-walker", "--virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn sakura_no_reverse_help() {
    let o = output(&["--color", "sakura", "--no-reverse", "--help"]);
    assert!(o.status.success());
}
