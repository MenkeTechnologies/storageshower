//! Binary smoke: `--bar-style`, `--color`, `--units` combinations.

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
fn gradient_cyan_version() {
    let o = output(&["--bar-style", "gradient", "--color", "cyan", "-V"]);
    assert!(o.status.success());
}

#[test]
fn solid_red_help() {
    let o = output(&["-b", "solid", "--color", "red", "--help"]);
    assert!(o.status.success());
}

#[test]
fn thin_matrix_units_bytes() {
    let o = output(&[
        "--bar-style",
        "thin",
        "--color",
        "matrix",
        "--units",
        "bytes",
        "-V",
    ]);
    assert!(o.status.success());
}

#[test]
fn ascii_sakura_list_colors() {
    let o = output(&["-b", "ascii", "--color", "sakura", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}
