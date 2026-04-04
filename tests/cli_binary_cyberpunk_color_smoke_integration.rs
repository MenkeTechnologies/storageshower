//! Binary smoke: cyberpunk `ColorMode` flags with `-V` / `--list-colors`.

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
fn bio_hazard_version() {
    let o = output(&["--color", "bio-hazard", "-V"]);
    assert!(o.status.success());
}

#[test]
fn glitch_pop_list_colors() {
    let o = output(&["--color", "glitch-pop", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn void_walker_help() {
    let o = output(&["--color", "void-walker", "--help"]);
    assert!(o.status.success());
}
