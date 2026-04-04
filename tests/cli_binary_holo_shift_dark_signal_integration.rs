//! Binary smoke: `holo-shift` and `dark-signal` color flags.

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
fn holo_shift_col_pct_version() {
    let o = output(&["--color", "holo-shift", "--col-pct", "11", "-V"]);
    assert!(o.status.success());
}

#[test]
fn dark_signal_no_bars_version() {
    let o = output(&["--color", "dark-signal", "--no-bars", "-V"]);
    assert!(o.status.success());
}
