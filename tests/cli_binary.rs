//! Smoke tests for the real `storageshower` binary (requires `cargo test` so
//! `CARGO_BIN_EXE_storageshower` is set).

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
fn help_short_exits_zero() {
    let o = output(&["-h"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(
        s.contains("transmission") || s.contains("storageshower"),
        "unexpected help stdout: {s}"
    );
}

#[test]
fn help_long_exits_zero() {
    let o = output(&["--help"]);
    assert!(o.status.success());
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(!s.is_empty());
}

#[test]
fn version_short_exits_zero() {
    let o = output(&["-V"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(
        s.contains(env!("CARGO_PKG_VERSION")),
        "expected version string in: {s}"
    );
}

#[test]
fn version_long_exits_zero() {
    let o = output(&["--version"]);
    assert!(o.status.success());
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(s.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn list_colors_exits_zero() {
    let o = output(&["--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(
        s.len() > 50,
        "expected non-trivial color listing, got len {}",
        s.len()
    );
}

#[test]
fn export_theme_exits_zero() {
    let o = output(&["--export-theme"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(
        s.contains('[') || s.contains("blue") || s.contains("theme"),
        "unexpected export-theme output: {}",
        s.chars().take(200).collect::<String>()
    );
}

#[test]
fn unknown_flag_fails() {
    let o = output(&["--not-a-real-flag-xyz"]);
    assert!(!o.status.success());
}

#[test]
fn help_does_not_open_tui() {
    let o = output(&["-h"]);
    assert!(o.status.success());
    let e = String::from_utf8_lossy(&o.stderr);
    assert!(
        !e.contains("panic") && !e.contains("thread 'main' panicked"),
        "stderr: {e}"
    );
}

#[test]
fn version_stderr_quiet_on_success() {
    let o = output(&["-V"]);
    assert!(o.status.success());
    assert!(
        o.stderr.is_empty(),
        "expected empty stderr on -V, got: {:?}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn list_colors_idempotent() {
    let a = output(&["--list-colors"]);
    let b = output(&["--list-colors"]);
    assert!(a.status.success() && b.status.success());
    assert_eq!(
        String::from_utf8_lossy(&a.stdout),
        String::from_utf8_lossy(&b.stdout)
    );
}

#[test]
fn help_stdout_differs_from_version_stdout() {
    let h = output(&["-h"]);
    let v = output(&["-V"]);
    assert_ne!(
        String::from_utf8_lossy(&h.stdout),
        String::from_utf8_lossy(&v.stdout)
    );
}

#[test]
fn sort_name_flag_parses_with_help() {
    // Exercises CLI + early exit; must not start TUI.
    let o = output(&["--sort", "name", "-h"]);
    assert!(o.status.success());
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(s.len() > 100, "expected full help text");
}

#[test]
fn units_gib_flag_parses_with_version() {
    let o = output(&["--units", "gib", "-V"]);
    assert!(o.status.success());
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(s.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn export_theme_with_config_path_exits_zero() {
    let o = output(&["--config", "/dev/null", "--export-theme"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn help_with_refresh_flag_exits_zero() {
    let o = output(&["--refresh", "5", "--help"]);
    assert!(o.status.success());
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(s.len() > 80, "expected help text");
}

#[test]
fn bar_style_gradient_with_help_exits_zero() {
    let o = output(&["--bar-style", "gradient", "--help"]);
    assert!(o.status.success());
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(s.len() > 80);
}
