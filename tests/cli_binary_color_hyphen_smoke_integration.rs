//! Binary smoke: hyphenated `--color` values with `-V` / `--help`.

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
fn cyber_frost_version() {
    let o = output(&["--color", "cyber-frost", "-V"]);
    assert!(o.status.success());
}

#[test]
fn plasma_core_help() {
    let o = output(&["--color", "plasma-core", "--help"]);
    assert!(o.status.success());
}

#[test]
fn steel_nerve_list_colors() {
    let o = output(&["--color", "steel-nerve", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn night_city_units_mib_version() {
    let o = output(&["--color", "night-city", "--units", "mib", "-V"]);
    assert!(o.status.success());
}
