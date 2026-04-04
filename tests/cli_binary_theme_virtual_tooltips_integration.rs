//! Binary smoke: `--theme`, `--virtual` / `--no-virtual`, `--tooltips` with help/version.

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
fn theme_with_help_exits_zero() {
    let o = output(&["--theme", "test_theme", "--help"]);
    assert!(o.status.success());
    let stdout = String::from_utf8_lossy(&o.stdout);
    assert!(stdout.len() > 100);
}

#[test]
fn no_virtual_with_version_exits_zero() {
    let o = output(&["--no-virtual", "-V"]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn virtual_with_list_colors_exits_zero() {
    let o = output(&["--virtual", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn no_tooltips_with_sort_name_exits_zero() {
    let o = output(&["--no-tooltips", "--sort", "name", "-V"]);
    assert!(o.status.success());
}
