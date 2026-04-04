//! Binary smoke: `cyan` and `toxic-waste` color flags.

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
fn cyan_show_all_version() {
    let o = output(&["--color", "cyan", "--virtual", "-V"]);
    assert!(o.status.success());
}

#[test]
fn toxic_waste_theme_help() {
    let o = output(&["--color", "toxic-waste", "--theme", "x", "--help"]);
    assert!(o.status.success());
}
