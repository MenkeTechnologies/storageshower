//! TOML `unit_mode` for each `UnitMode` string.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;
use storageshower::types::UnitMode;

fn with_unit(mode: &str) -> String {
    format!(
        r#"
sort_mode = "Name"
sort_rev = false
show_local = false
refresh_rate = 1
bar_style = "Gradient"
color_mode = "Default"
thresh_warn = 70
thresh_crit = 90
show_bars = true
show_border = true
show_header = true
compact = false
show_used = true
full_mount = false
unit_mode = "{mode}"
"#
    )
}

#[test]
fn unit_mode_human() {
    let p: Prefs = toml::from_str(&with_unit("Human")).unwrap();
    assert_eq!(p.unit_mode, UnitMode::Human);
}

#[test]
fn unit_mode_gib() {
    let p: Prefs = toml::from_str(&with_unit("GiB")).unwrap();
    assert_eq!(p.unit_mode, UnitMode::GiB);
}

#[test]
fn unit_mode_mib() {
    let p: Prefs = toml::from_str(&with_unit("MiB")).unwrap();
    assert_eq!(p.unit_mode, UnitMode::MiB);
}

#[test]
fn unit_mode_bytes() {
    let p: Prefs = toml::from_str(&with_unit("Bytes")).unwrap();
    assert_eq!(p.unit_mode, UnitMode::Bytes);
}
