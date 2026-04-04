//! TOML `Prefs` with `col_pct_w`, `col_mount_w`, and `col_bar_end_w` together.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

#[test]
fn deserialize_all_three_column_widths() {
    let t = r#"
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
col_pct_w = 9
col_mount_w = 27
col_bar_end_w = 41
"#;
    let p: Prefs = toml::from_str(t).unwrap();
    assert_eq!(p.col_pct_w, 9);
    assert_eq!(p.col_mount_w, 27);
    assert_eq!(p.col_bar_end_w, 41);
}
