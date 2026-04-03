//! More `CARGO_BIN_EXE_storageshower` smoke: `--export-theme`, `--list-colors`, and flag stacks.

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
fn export_theme_stdout_nonempty() {
    let o = output(&["--export-theme"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert!(!o.stdout.is_empty(), "export-theme should print to stdout");
}

#[test]
fn list_colors_contains_default_or_neon() {
    let o = output(&["--list-colors"]);
    assert!(o.status.success());
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(
        s.to_lowercase().contains("default") || s.contains("Neon") || s.contains("palette"),
        "unexpected list-colors: {}",
        s.chars().take(120).collect::<String>()
    );
}

#[test]
fn export_theme_with_color_flag_exits_zero() {
    let o = output(&["--export-theme", "--color", "matrix"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn list_colors_with_sort_flag_exits_zero() {
    let o = output(&["--list-colors", "--sort", "pct"]);
    assert!(o.status.success());
}

#[test]
fn export_theme_with_theme_name_exits_zero() {
    let o = output(&["--export-theme", "--theme", "custom"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn version_with_export_theme_exits_zero() {
    let o = output(&["-V", "--export-theme"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_with_list_colors_exits_zero() {
    let o = output(&["--help", "--list-colors"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).len() > 50);
}
