//! TOML for individual column width fields (`col_pct_w`, `col_mount_w`, `col_bar_end_w`).

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

const BASE: &str = r#"
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
"#;

#[test]
fn col_pct_w_only() {
    let s = format!(
        r#"
{BASE}
col_pct_w = 11
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.col_pct_w, 11);
}

#[test]
fn col_mount_w_only() {
    let s = format!(
        r#"
{BASE}
col_mount_w = 24
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.col_mount_w, 24);
}

#[test]
fn col_bar_end_w_only() {
    let s = format!(
        r#"
{BASE}
col_bar_end_w = 38
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.col_bar_end_w, 38);
}
