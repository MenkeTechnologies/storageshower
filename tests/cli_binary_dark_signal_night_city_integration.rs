//! Binary smoke: `dark-signal` and `night-city` color flags.

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
fn dark_signal_list_colors() {
    let o = output(&["--color", "dark-signal", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn night_city_version_reverse_sort() {
    let o = output(&["--color", "night-city", "-R", "--sort", "pct", "-V"]);
    assert!(o.status.success());
}
