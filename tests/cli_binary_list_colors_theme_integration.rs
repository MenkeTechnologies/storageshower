//! Binary smoke: `--list-colors` combined with `--theme` and `--color`.

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
fn list_colors_with_theme_name() {
    let o = output(&["--theme", "custom_slot", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn list_colors_color_zaibatsu_sort_size() {
    let o = output(&["--color", "zaibatsu", "--list-colors", "--sort", "size"]);
    assert!(o.status.success());
}
