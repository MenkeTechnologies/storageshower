//! TOML roundtrips for `active_theme` and `custom_themes` keys together.
#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;
use storageshower::types::ThemeColors;

#[test]
fn toml_roundtrip_active_theme_only() {
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
active_theme = "neon-slot"
"#;
    let p: Prefs = toml::from_str(t).unwrap();
    assert_eq!(p.active_theme.as_deref(), Some("neon-slot"));
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.active_theme, p.active_theme);
}

#[test]
fn toml_custom_theme_key_with_dash() {
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

[custom_themes."my-theme"]
blue = 1
green = 2
purple = 3
light_purple = 4
royal = 5
dark_purple = 6
"#;
    let p: Prefs = toml::from_str(t).unwrap();
    let th = p.custom_themes.get("my-theme").expect("theme");
    assert_eq!(th.blue, 1);
}

#[test]
fn active_theme_points_to_custom_table() {
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
active_theme = "slot1"

[custom_themes.slot1]
blue = 10
green = 20
purple = 30
light_purple = 40
royal = 50
dark_purple = 60
"#;
    let p: Prefs = toml::from_str(t).unwrap();
    assert_eq!(p.active_theme.as_deref(), Some("slot1"));
    assert!(p.custom_themes.contains_key("slot1"));
}

#[test]
fn prefs_struct_roundtrip_two_named_themes() {
    let mut p = Prefs::default();
    p.active_theme = Some("b".into());
    p.custom_themes.insert(
        "a".into(),
        ThemeColors {
            blue: 1,
            green: 2,
            purple: 3,
            light_purple: 4,
            royal: 5,
            dark_purple: 6,
        },
    );
    p.custom_themes.insert(
        "b".into(),
        ThemeColors {
            blue: 9,
            green: 8,
            purple: 7,
            light_purple: 6,
            royal: 5,
            dark_purple: 4,
        },
    );
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.active_theme, p.active_theme);
    assert_eq!(q.custom_themes.len(), 2);
    assert_eq!(q.custom_themes.get("b").unwrap().blue, 9);
}
