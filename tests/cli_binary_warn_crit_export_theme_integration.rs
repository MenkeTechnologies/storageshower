//! Binary smoke: `--warn` / `--crit` with `--export-theme`, `--help`, `-V`.

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
fn warn_crit_export_theme_help() {
    let o = output(&["--warn", "1", "--crit", "99", "--export-theme", "--help"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).len() > 80);
}

#[test]
fn warn_crit_with_version() {
    let o = output(&["-w", "5", "-C", "95", "-V"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn warn_crit_config_dev_null_version() {
    let o = output(&[
        "--config",
        "/dev/null",
        "--warn",
        "10",
        "--crit",
        "80",
        "-V",
    ]);
    assert!(o.status.success());
}

#[test]
fn warn_crit_sort_pct_list_colors() {
    let o = output(&[
        "--warn",
        "20",
        "--crit",
        "90",
        "--sort",
        "pct",
        "--list-colors",
    ]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}
