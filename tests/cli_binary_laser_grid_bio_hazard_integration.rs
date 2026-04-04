//! Binary smoke: `laser-grid` and `bio-hazard` color flags.

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
fn laser_grid_list_colors() {
    let o = output(&["--color", "laser-grid", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn bio_hazard_bar_style_thin_version() {
    let o = output(&["--color", "bio-hazard", "-b", "thin", "-V"]);
    assert!(o.status.success());
}
