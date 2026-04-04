//! Binary smoke: several `ColorMode` flags (`dark-signal`, `holo-shift`, `night-city`, `deep-net`, `plasma-core`).

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
fn dark_signal_version() {
    let o = output(&["--color", "dark-signal", "-V"]);
    assert!(o.status.success());
}

#[test]
fn holo_shift_help() {
    let o = output(&["--color", "holo-shift", "--help"]);
    assert!(o.status.success());
}

#[test]
fn night_city_list_colors() {
    let o = output(&["--color", "night-city", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn deep_net_units_mib() {
    let o = output(&["--color", "deep-net", "--units", "mib", "-V"]);
    assert!(o.status.success());
}

#[test]
fn plasma_core_config_dev_null() {
    let o = output(&["--color", "plasma-core", "--config", "/dev/null", "--help"]);
    assert!(o.status.success());
}
