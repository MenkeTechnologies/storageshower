//! Binary smoke: `night-city` and `glitch-pop` color flags.

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
fn night_city_tooltips_version() {
    let o = output(&["--color", "night-city", "--tooltips", "-V"]);
    assert!(o.status.success());
}

#[test]
fn glitch_pop_local_only_help() {
    let o = output(&["--color", "glitch-pop", "-l", "--help"]);
    assert!(o.status.success());
}
