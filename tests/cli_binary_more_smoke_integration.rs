//! Additional smoke tests for the real `storageshower` binary (`CARGO_BIN_EXE_storageshower`).

use std::process::Command;

fn storageshower_exe() -> &'static str {
    env!("CARGO_BIN_EXE_storageshower")
}

fn output(args: &[&str]) -> std::process::Output {
    Command::new(storageshower_exe())
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to spawn {}: {e}", storageshower_exe()))
}

#[test]
fn list_colors_with_version_exits_zero() {
    let o = output(&["--list-colors", "-V"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(s.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn export_theme_with_help_exits_zero() {
    let o = output(&["--export-theme", "--help"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).len() > 80);
}

#[test]
fn theme_flag_with_help_exits_zero() {
    let o = output(&["--theme", "x", "--help"]);
    assert!(o.status.success());
}

#[test]
fn local_only_with_list_colors_exits_zero() {
    let o = output(&["--local-only", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn config_path_with_version_exits_zero() {
    let o = output(&["--config", "/dev/null", "-V"]);
    assert!(o.status.success());
}

#[test]
fn units_mib_with_help_exits_zero() {
    let o = output(&["--units", "mib", "--help"]);
    assert!(o.status.success());
}

#[test]
fn color_neon_noir_with_version_exits_zero() {
    let o = output(&["--color", "neon-noir", "-V"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn sort_size_reverse_stack_exits_zero() {
    let o = output(&["--sort", "size", "--reverse", "-V"]);
    assert!(o.status.success());
}

#[test]
fn col_width_flags_with_help_exits_zero() {
    let o = output(&[
        "--col-mount",
        "20",
        "--col-bar-end",
        "30",
        "--col-pct",
        "6",
        "--help",
    ]);
    assert!(o.status.success());
}

#[test]
fn virtual_flag_with_help_exits_zero() {
    let o = output(&["--virtual", "--help"]);
    assert!(o.status.success());
}
