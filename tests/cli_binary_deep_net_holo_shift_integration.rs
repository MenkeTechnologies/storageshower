//! Binary smoke: `deep-net` and `holo-shift` color flags.

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
fn deep_net_units_gib_version() {
    let o = output(&["--color", "deep-net", "-u", "gib", "-V"]);
    assert!(o.status.success());
}

#[test]
fn holo_shift_list_colors() {
    let o = output(&["--color", "holo-shift", "--list-colors"]);
    assert!(
        o.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&o.stderr)
    );
}
