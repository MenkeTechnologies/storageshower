//! TOML roundtrip for column width fields on `Prefs`.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

const PREFIX: &str = r#"
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
fn deserialize_col_mount_bar_pct() {
    let s = format!(
        r#"
{PREFIX}
col_mount_w = 33
col_bar_end_w = 44
col_pct_w = 7
"#
    );
    let p: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(p.col_mount_w, 33);
    assert_eq!(p.col_bar_end_w, 44);
    assert_eq!(p.col_pct_w, 7);
}

#[test]
fn roundtrip_nonzero_column_widths() {
    let mut p = Prefs::default();
    p.col_mount_w = 19;
    p.col_bar_end_w = 28;
    p.col_pct_w = 12;
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.col_mount_w, 19);
    assert_eq!(q.col_bar_end_w, 28);
    assert_eq!(q.col_pct_w, 12);
}

#[test]
fn zeros_omitted_or_explicit_roundtrip() {
    let mut p = Prefs::default();
    p.col_mount_w = 0;
    p.col_bar_end_w = 0;
    p.col_pct_w = 0;
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.col_mount_w, 0);
    assert_eq!(q.col_bar_end_w, 0);
    assert_eq!(q.col_pct_w, 0);
}
