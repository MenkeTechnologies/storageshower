use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use sysinfo::DiskKind;

pub const DARK_BG: Color = Color::Indexed(234);
pub const HELP_BG: Color = Color::Indexed(236);
pub const DIM_BORDER: Color = Color::Indexed(240);

#[derive(Clone, Copy)]
pub enum DragTarget {
    MountSep,
    BarEndSep,
    PctSep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
pub enum SortMode {
    Name,
    Pct,
    Size,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
pub enum BarStyle {
    Gradient,
    Solid,
    Thin,
    Ascii,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
pub enum ColorMode {
    Default,
    Green,
    Blue,
    Purple,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
pub enum UnitMode {
    #[default]
    Human,
    #[value(name = "gib")]
    GiB,
    #[value(name = "mib")]
    MiB,
    Bytes,
}

#[derive(Clone)]
pub struct DiskEntry {
    pub mount: String,
    pub used: u64,
    pub total: u64,
    pub pct: f64,
    pub kind: DiskKind,
    pub fs: String,
}

#[derive(Clone)]
pub struct SysStats {
    pub hostname: String,
    pub load_avg: (f64, f64, f64),
    pub mem_used: u64,
    pub mem_total: u64,
    pub cpu_count: usize,
    pub process_count: usize,
    pub swap_used: u64,
    pub swap_total: u64,
    pub kernel: String,
    pub arch: String,
    pub uptime: u64,
    pub os_name: String,
    pub os_version: String,
}

impl Default for SysStats {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            load_avg: (0.0, 0.0, 0.0),
            mem_used: 0,
            mem_total: 1,
            cpu_count: 0,
            process_count: 0,
            swap_used: 0,
            swap_total: 0,
            kernel: String::new(),
            arch: String::new(),
            uptime: 0,
            os_name: String::new(),
            os_version: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sys_stats_default() {
        let s = SysStats::default();
        assert!(s.hostname.is_empty());
        assert_eq!(s.mem_total, 1); // non-zero to avoid div-by-zero
        assert_eq!(s.uptime, 0);
    }

    #[test]
    fn unit_mode_default_is_human() {
        assert_eq!(UnitMode::default(), UnitMode::Human);
    }

    #[test]
    fn sort_mode_equality() {
        assert_eq!(SortMode::Name, SortMode::Name);
        assert_ne!(SortMode::Name, SortMode::Pct);
        assert_ne!(SortMode::Pct, SortMode::Size);
    }

    #[test]
    fn bar_style_equality() {
        assert_eq!(BarStyle::Gradient, BarStyle::Gradient);
        assert_ne!(BarStyle::Solid, BarStyle::Thin);
    }

    #[test]
    fn color_mode_equality() {
        assert_eq!(ColorMode::Default, ColorMode::Default);
        assert_ne!(ColorMode::Green, ColorMode::Blue);
    }
}
