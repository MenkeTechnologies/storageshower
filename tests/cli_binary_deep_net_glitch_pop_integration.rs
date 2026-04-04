//! Binary smoke: `deep-net` and `glitch-pop` color flags.

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
fn deep_net_col_bar_end_version() {
    let o = output(&["--color", "deep-net", "--col-bar-end", "20", "-V"]);
    assert!(o.status.success());
}

#[test]
fn glitch_pop_bars_version() {
    let o = output(&["--color", "glitch-pop", "--bars", "-V"]);
    assert!(o.status.success());
}
