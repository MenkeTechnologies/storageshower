//! Binary smoke: `default` and `cyan` color flags.

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
fn default_color_version() {
    let o = output(&["--color", "default", "-V"]);
    assert!(o.status.success());
}

#[test]
fn cyan_list_colors() {
    let o = output(&["--color", "cyan", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}
