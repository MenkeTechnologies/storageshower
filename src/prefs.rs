use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Prefs {
    pub sort_mode: SortMode,
    pub sort_rev: bool,
    pub show_local: bool,
    pub refresh_rate: u64,
    pub bar_style: BarStyle,
    pub color_mode: ColorMode,
    pub thresh_warn: u8,
    pub thresh_crit: u8,
    pub show_bars: bool,
    pub show_border: bool,
    pub show_header: bool,
    pub compact: bool,
    pub show_used: bool,
    pub full_mount: bool,
    #[serde(default = "default_true")]
    pub show_all: bool,
    #[serde(default)]
    pub unit_mode: UnitMode,
    #[serde(default)]
    pub col_mount_w: u16,
    #[serde(default)]
    pub col_bar_end_w: u16,
    #[serde(default)]
    pub col_pct_w: u16,
    #[serde(default)]
    pub custom_themes: HashMap<String, ThemeColors>,
    #[serde(default)]
    pub active_theme: Option<String>,
    #[serde(default = "default_true")]
    pub show_tooltips: bool,
    #[serde(default)]
    pub bookmarks: Vec<String>,
}

fn default_true() -> bool {
    true
}

impl Default for Prefs {
    fn default() -> Self {
        Self {
            sort_mode: SortMode::Name,
            sort_rev: false,
            show_local: false,
            refresh_rate: 1,
            bar_style: BarStyle::Gradient,
            color_mode: ColorMode::Default,
            thresh_warn: 70,
            thresh_crit: 90,
            show_bars: true,
            show_border: true,
            show_header: true,
            compact: false,
            show_used: true,
            full_mount: false,
            show_all: true,
            unit_mode: UnitMode::Human,
            col_mount_w: 0,
            col_bar_end_w: 0,
            col_pct_w: 0,
            custom_themes: HashMap::new(),
            active_theme: None,
            show_tooltips: true,
            bookmarks: Vec::new(),
        }
    }
}

fn prefs_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".storageshower.conf")
}

const DEFAULT_CONF: &str = include_str!("../storageshower.default.conf");

pub fn load_prefs() -> Prefs {
    load_prefs_from(None)
}

pub fn load_prefs_from(custom_path: Option<&str>) -> Prefs {
    let path = match custom_path {
        Some(p) => PathBuf::from(p),
        None => prefs_path(),
    };
    if !path.exists() && custom_path.is_none() {
        let _ = std::fs::write(&path, DEFAULT_CONF);
    }
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_prefs(p: &Prefs) {
    if let Ok(s) = toml::to_string_pretty(p) {
        let _ = std::fs::write(prefs_path(), s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_prefs_values() {
        let p = Prefs::default();
        assert_eq!(p.sort_mode, SortMode::Name);
        assert!(!p.sort_rev);
        assert!(!p.show_local);
        assert_eq!(p.refresh_rate, 1);
        assert_eq!(p.bar_style, BarStyle::Gradient);
        assert_eq!(p.color_mode, ColorMode::Default);
        assert_eq!(p.thresh_warn, 70);
        assert_eq!(p.thresh_crit, 90);
        assert!(p.show_bars);
        assert!(p.show_border);
        assert!(p.show_header);
        assert!(!p.compact);
        assert!(p.show_used);
        assert!(!p.full_mount);
        assert!(p.show_all);
        assert_eq!(p.unit_mode, UnitMode::Human);
        assert_eq!(p.col_mount_w, 0);
        assert_eq!(p.col_bar_end_w, 0);
        assert_eq!(p.col_pct_w, 0);
    }

    #[test]
    fn prefs_roundtrip_toml() {
        let mut p = Prefs::default();
        p.sort_mode = SortMode::Size;
        p.sort_rev = true;
        p.bar_style = BarStyle::Ascii;
        p.color_mode = ColorMode::Purple;
        p.thresh_warn = 60;
        p.thresh_crit = 85;
        p.unit_mode = UnitMode::GiB;
        p.col_mount_w = 25;
        p.col_bar_end_w = 30;
        p.col_pct_w = 7;
        p.refresh_rate = 5;

        let serialized = toml::to_string_pretty(&p).unwrap();
        let deserialized: Prefs = toml::from_str(&serialized).unwrap();

        assert_eq!(deserialized.sort_mode, SortMode::Size);
        assert!(deserialized.sort_rev);
        assert_eq!(deserialized.bar_style, BarStyle::Ascii);
        assert_eq!(deserialized.color_mode, ColorMode::Purple);
        assert_eq!(deserialized.thresh_warn, 60);
        assert_eq!(deserialized.thresh_crit, 85);
        assert_eq!(deserialized.unit_mode, UnitMode::GiB);
        assert_eq!(deserialized.col_mount_w, 25);
        assert_eq!(deserialized.col_bar_end_w, 30);
        assert_eq!(deserialized.col_pct_w, 7);
        assert_eq!(deserialized.refresh_rate, 5);
    }

    #[test]
    fn prefs_deserialize_missing_fields_uses_defaults() {
        let toml_str = r#"
            sort_mode = "Pct"
            sort_rev = false
            show_local = false
            refresh_rate = 2
            bar_style = "Solid"
            color_mode = "Green"
            thresh_warn = 50
            thresh_crit = 80
            show_bars = true
            show_border = false
            show_header = true
            compact = true
            show_used = false
            full_mount = true
        "#;
        let p: Prefs = toml::from_str(toml_str).unwrap();
        // Fields present:
        assert_eq!(p.sort_mode, SortMode::Pct);
        assert_eq!(p.bar_style, BarStyle::Solid);
        assert!(p.compact);
        assert!(!p.show_border);
        // Defaults for missing serde(default) fields:
        assert!(p.show_all); // default_true
        assert_eq!(p.unit_mode, UnitMode::Human);
        assert_eq!(p.col_mount_w, 0);
        assert_eq!(p.col_bar_end_w, 0);
        assert_eq!(p.col_pct_w, 0);
    }

    #[test]
    fn default_conf_parses() {
        let p: Result<Prefs, _> = toml::from_str(DEFAULT_CONF);
        assert!(p.is_ok(), "Default config should parse: {:?}", p.err());
    }

    #[test]
    fn load_prefs_from_file_reads_values() {
        use crate::types::SortMode;

        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("storageshower-test.conf");
        let mut expected = Prefs::default();
        expected.sort_mode = SortMode::Pct;
        expected.refresh_rate = 9;
        let contents = toml::to_string_pretty(&expected).expect("serialize prefs");
        std::fs::write(&path, contents).expect("write temp prefs");
        let loaded = load_prefs_from(Some(path.to_str().expect("utf8 path")));
        assert_eq!(loaded.sort_mode, SortMode::Pct);
        assert_eq!(loaded.refresh_rate, 9);
    }

    #[test]
    fn load_prefs_from_empty_file_yields_defaults() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("empty.conf");
        std::fs::write(&path, "").expect("write empty");
        let p = load_prefs_from(Some(path.to_str().expect("utf8 path")));
        assert_eq!(p.sort_mode, crate::types::SortMode::Name);
        assert_eq!(p.refresh_rate, 1);
    }

    #[test]
    fn load_prefs_from_garbage_toml_yields_defaults() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("bad.conf");
        std::fs::write(&path, "this is not [[valid]] toml {{{").expect("write");
        let p = load_prefs_from(Some(path.to_str().expect("utf8 path")));
        assert_eq!(p.sort_mode, crate::types::SortMode::Name);
    }

    #[test]
    fn prefs_toml_roundtrip_active_theme_and_bookmarks() {
        let mut p = Prefs::default();
        p.active_theme = Some("neon".into());
        p.bookmarks = vec!["/home".into(), "/data".into()];
        let s = toml::to_string_pretty(&p).unwrap();
        let q: Prefs = toml::from_str(&s).unwrap();
        assert_eq!(q.active_theme.as_deref(), Some("neon"));
        assert_eq!(q.bookmarks, vec!["/home", "/data"]);
    }
}
