//! Binary smoke: `purple` and `green` color flags.

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
fn purple_list_colors() {
    let o = output(&["--color", "purple", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn green_bar_style_ascii_version() {
    let o = output(&["--color", "green", "-b", "ascii", "-V"]);
    assert!(o.status.success());
}
