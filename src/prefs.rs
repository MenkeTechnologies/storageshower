use serde::{Deserialize, Serialize};
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
}

fn default_true() -> bool { true }

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
    let path = prefs_path();
    if !path.exists() {
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
