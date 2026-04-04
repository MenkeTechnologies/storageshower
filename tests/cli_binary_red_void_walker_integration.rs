//! Binary smoke: `red` and `void-walker` color flags.

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
fn red_sort_size_version() {
    let o = output(&["--color", "red", "-s", "size", "-V"]);
    assert!(o.status.success());
}

#[test]
fn void_walker_tooltips_help() {
    let o = output(&["--color", "void-walker", "--tooltips", "--help"]);
    assert!(o.status.success());
}
