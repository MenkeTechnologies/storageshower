//! Binary smoke: `--warn` / `--crit` with help or version (`CARGO_BIN_EXE_storageshower`).

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
fn warn_crit_with_help_exits_zero() {
    let o = output(&["--warn", "50", "--crit", "85", "--help"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).len() > 80);
}

#[test]
fn warn_only_with_version_exits_zero() {
    let o = output(&["--warn", "1", "-V"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn crit_only_with_help_exits_zero() {
    let o = output(&["--crit", "100", "--help"]);
    assert!(o.status.success());
}

#[test]
fn warn_crit_with_list_colors_exits_zero() {
    let o = output(&["--warn", "60", "--crit", "95", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn warn_crit_with_sort_pct_exits_zero() {
    let o = output(&["--warn", "55", "--crit", "88", "--sort", "pct", "-V"]);
    assert!(o.status.success());
}
