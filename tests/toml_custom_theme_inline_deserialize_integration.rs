//! `toml::from_str` for `Prefs` with inline `[custom_themes.*]` table.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

#[test]
fn deserialize_custom_theme_with_active() {
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
active_theme = "mine"

[custom_themes.mine]
blue = 1
green = 2
purple = 3
light_purple = 4
royal = 5
dark_purple = 6
"#;
    let p: Prefs = toml::from_str(t).unwrap();
    assert_eq!(p.active_theme.as_deref(), Some("mine"));
    let th = p.custom_themes.get("mine").expect("mine");
    assert_eq!(th.blue, 1);
    assert_eq!(th.dark_purple, 6);
}

#[test]
fn deserialize_two_theme_keys() {
    let t = r#"
sort_mode = "Pct"
sort_rev = false
show_local = false
refresh_rate = 2
bar_style = "Solid"
color_mode = "Blue"
thresh_warn = 50
thresh_crit = 95
show_bars = true
show_border = true
show_header = true
compact = false
show_used = true
full_mount = false

[custom_themes.a]
blue = 10
green = 11
purple = 12
light_purple = 13
royal = 14
dark_purple = 15

[custom_themes.b]
blue = 20
green = 21
purple = 22
light_purple = 23
royal = 24
dark_purple = 25
"#;
    let p: Prefs = toml::from_str(t).unwrap();
    assert_eq!(p.custom_themes.len(), 2);
    assert_eq!(p.custom_themes.get("a").expect("a").green, 11);
    assert_eq!(p.custom_themes.get("b").expect("b").royal, 24);
}
