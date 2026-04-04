//! Binary smoke: `void-walker` and `toxic-waste` color flags.

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
fn void_walker_no_border_version() {
    let o = output(&["--color", "void-walker", "--no-border", "-V"]);
    assert!(o.status.success());
}

#[test]
fn toxic_waste_config_dev_null_help() {
    let o = output(&["--color", "toxic-waste", "--config", "/dev/null", "--help"]);
    assert!(o.status.success());
}
