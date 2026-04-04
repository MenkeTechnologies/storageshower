//! TOML `refresh_rate` for `Prefs`.

#![allow(clippy::field_reassign_with_default)]

use storageshower::prefs::Prefs;

fn prefs_toml(refresh_rate: u64) -> String {
    format!(
        r#"
sort_mode = "Name"
sort_rev = false
show_local = false
refresh_rate = {refresh_rate}
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
"#
    )
}

#[test]
fn deserialize_refresh_one() {
    let p: Prefs = toml::from_str(&prefs_toml(1)).unwrap();
    assert_eq!(p.refresh_rate, 1);
}

#[test]
fn deserialize_refresh_sixty() {
    let p: Prefs = toml::from_str(&prefs_toml(60)).unwrap();
    assert_eq!(p.refresh_rate, 60);
}

#[test]
fn roundtrip_refresh_large() {
    let p: Prefs = toml::from_str(&prefs_toml(3600)).unwrap();
    assert_eq!(p.refresh_rate, 3600);
    let s = toml::to_string_pretty(&p).unwrap();
    let q: Prefs = toml::from_str(&s).unwrap();
    assert_eq!(q.refresh_rate, 3600);
}
