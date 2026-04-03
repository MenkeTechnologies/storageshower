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
    Amber,
    Cyan,
    Red,
    Sakura,
    Matrix,
    Sunset,
    #[value(name = "neon-noir")]
    NeonNoir,
    #[value(name = "chrome-heart")]
    ChromeHeart,
    #[value(name = "blade-runner")]
    BladeRunner,
    #[value(name = "void-walker")]
    VoidWalker,
    #[value(name = "toxic-waste")]
    ToxicWaste,
    #[value(name = "cyber-frost")]
    CyberFrost,
    #[value(name = "plasma-core")]
    PlasmaCore,
    #[value(name = "steel-nerve")]
    SteelNerve,
    #[value(name = "dark-signal")]
    DarkSignal,
    #[value(name = "glitch-pop")]
    GlitchPop,
    #[value(name = "holo-shift")]
    HoloShift,
    #[value(name = "night-city")]
    NightCity,
    #[value(name = "deep-net")]
    DeepNet,
    #[value(name = "laser-grid")]
    LaserGrid,
    #[value(name = "quantum-flux")]
    QuantumFlux,
    #[value(name = "bio-hazard")]
    BioHazard,
    Darkwave,
    Overlock,
    Megacorp,
    Zaibatsu,
}

impl ColorMode {
    pub const ALL: &'static [ColorMode] = &[
        ColorMode::Default,
        ColorMode::Green,
        ColorMode::Blue,
        ColorMode::Purple,
        ColorMode::Amber,
        ColorMode::Cyan,
        ColorMode::Red,
        ColorMode::Sakura,
        ColorMode::Matrix,
        ColorMode::Sunset,
        ColorMode::NeonNoir,
        ColorMode::ChromeHeart,
        ColorMode::BladeRunner,
        ColorMode::VoidWalker,
        ColorMode::ToxicWaste,
        ColorMode::CyberFrost,
        ColorMode::PlasmaCore,
        ColorMode::SteelNerve,
        ColorMode::DarkSignal,
        ColorMode::GlitchPop,
        ColorMode::HoloShift,
        ColorMode::NightCity,
        ColorMode::DeepNet,
        ColorMode::LaserGrid,
        ColorMode::QuantumFlux,
        ColorMode::BioHazard,
        ColorMode::Darkwave,
        ColorMode::Overlock,
        ColorMode::Megacorp,
        ColorMode::Zaibatsu,
    ];

    pub fn name(self) -> &'static str {
        match self {
            ColorMode::Default => "Neon Sprawl",
            ColorMode::Green => "Acid Rain",
            ColorMode::Blue => "Ice Breaker",
            ColorMode::Purple => "Synth Wave",
            ColorMode::Amber => "Rust Belt",
            ColorMode::Cyan => "Ghost Wire",
            ColorMode::Red => "Red Sector",
            ColorMode::Sakura => "Sakura Den",
            ColorMode::Matrix => "Data Stream",
            ColorMode::Sunset => "Solar Flare",
            ColorMode::NeonNoir => "Neon Noir",
            ColorMode::ChromeHeart => "Chrome Heart",
            ColorMode::BladeRunner => "Blade Runner",
            ColorMode::VoidWalker => "Void Walker",
            ColorMode::ToxicWaste => "Toxic Waste",
            ColorMode::CyberFrost => "Cyber Frost",
            ColorMode::PlasmaCore => "Plasma Core",
            ColorMode::SteelNerve => "Steel Nerve",
            ColorMode::DarkSignal => "Dark Signal",
            ColorMode::GlitchPop => "Glitch Pop",
            ColorMode::HoloShift => "Holo Shift",
            ColorMode::NightCity => "Night City",
            ColorMode::DeepNet => "Deep Net",
            ColorMode::LaserGrid => "Laser Grid",
            ColorMode::QuantumFlux => "Quantum Flux",
            ColorMode::BioHazard => "Bio Hazard",
            ColorMode::Darkwave => "Darkwave",
            ColorMode::Overlock => "Overlock",
            ColorMode::Megacorp => "Megacorp",
            ColorMode::Zaibatsu => "Zaibatsu",
        }
    }

    pub fn next(self) -> ColorMode {
        let all = ColorMode::ALL;
        let idx = all.iter().position(|&m| m == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeColors {
    pub blue: u8,
    pub green: u8,
    pub purple: u8,
    pub light_purple: u8,
    pub royal: u8,
    pub dark_purple: u8,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmartHealth {
    Verified,
    Failing,
    Unknown,
}

#[derive(Clone)]
pub struct DiskEntry {
    pub mount: String,
    pub used: u64,
    pub total: u64,
    pub pct: f64,
    pub kind: DiskKind,
    pub fs: String,
    pub latency_ms: Option<f64>,
    pub io_read_rate: Option<f64>,
    pub io_write_rate: Option<f64>,
    pub smart_status: Option<SmartHealth>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrillSortMode {
    Size,
    Name,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverZone {
    None,
    TitleBar,
    FooterBar,
    DiskRow(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Disks,
    DrillDown,
}

#[derive(Clone)]
pub struct DirEntry {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
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

    #[test]
    fn disk_entry_clone() {
        let d = DiskEntry {
            mount: "/mnt".into(),
            used: 100,
            total: 200,
            pct: 50.0,
            kind: DiskKind::SSD,
            fs: "ext4".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        };
        let c = d.clone();
        assert_eq!(c.mount, "/mnt");
        assert_eq!(c.used, 100);
        assert_eq!(c.total, 200);
        assert!((c.pct - 50.0).abs() < f64::EPSILON);
        assert_eq!(c.fs, "ext4");
    }

    #[test]
    fn sys_stats_clone() {
        let s = SysStats {
            hostname: "test".into(),
            load_avg: (1.0, 2.0, 3.0),
            mem_used: 100,
            mem_total: 200,
            cpu_count: 4,
            process_count: 50,
            swap_used: 10,
            swap_total: 20,
            kernel: "6.0".into(),
            arch: "x86_64".into(),
            uptime: 3600,
            os_name: "Linux".into(),
            os_version: "6.0".into(),
        };
        let c = s.clone();
        assert_eq!(c.hostname, "test");
        assert_eq!(c.cpu_count, 4);
        assert_eq!(c.uptime, 3600);
        assert_eq!(c.kernel, "6.0");
    }

    #[test]
    fn unit_mode_all_variants_debug() {
        for mode in [
            UnitMode::Human,
            UnitMode::GiB,
            UnitMode::MiB,
            UnitMode::Bytes,
        ] {
            let s = format!("{:?}", mode);
            assert!(!s.is_empty());
        }
    }

    #[test]
    fn sort_mode_all_variants_debug() {
        for mode in [SortMode::Name, SortMode::Pct, SortMode::Size] {
            let s = format!("{:?}", mode);
            assert!(!s.is_empty());
        }
    }

    #[test]
    fn bar_style_all_variants_debug() {
        for style in [
            BarStyle::Gradient,
            BarStyle::Solid,
            BarStyle::Thin,
            BarStyle::Ascii,
        ] {
            let s = format!("{:?}", style);
            assert!(!s.is_empty());
        }
    }

    #[test]
    fn color_mode_all_variants_debug() {
        for mode in ColorMode::ALL {
            let s = format!("{:?}", mode);
            assert!(!s.is_empty());
        }
    }

    #[test]
    fn sys_stats_default_non_zero_mem() {
        let s = SysStats::default();
        assert_eq!(s.mem_total, 1); // prevents div-by-zero
    }

    #[test]
    fn unit_mode_serialize_deserialize() {
        for mode in [
            UnitMode::Human,
            UnitMode::GiB,
            UnitMode::MiB,
            UnitMode::Bytes,
        ] {
            let s = serde_json::to_string(&mode).unwrap();
            let d: UnitMode = serde_json::from_str(&s).unwrap();
            assert_eq!(d, mode);
        }
    }

    #[test]
    fn sort_mode_serialize_deserialize() {
        for mode in [SortMode::Name, SortMode::Pct, SortMode::Size] {
            let s = serde_json::to_string(&mode).unwrap();
            let d: SortMode = serde_json::from_str(&s).unwrap();
            assert_eq!(d, mode);
        }
    }

    #[test]
    fn bar_style_serialize_deserialize() {
        for style in [
            BarStyle::Gradient,
            BarStyle::Solid,
            BarStyle::Thin,
            BarStyle::Ascii,
        ] {
            let s = serde_json::to_string(&style).unwrap();
            let d: BarStyle = serde_json::from_str(&s).unwrap();
            assert_eq!(d, style);
        }
    }

    #[test]
    fn color_mode_serialize_deserialize() {
        for &mode in ColorMode::ALL {
            let s = serde_json::to_string(&mode).unwrap();
            let d: ColorMode = serde_json::from_str(&s).unwrap();
            assert_eq!(d, mode);
        }
    }

    #[test]
    fn color_mode_next_advances_along_all() {
        let first = ColorMode::ALL[0];
        assert_eq!(first, ColorMode::Default);
        assert_eq!(first.next(), ColorMode::ALL[1]);
    }

    #[test]
    fn color_mode_next_wraps_last_to_first() {
        let last = *ColorMode::ALL.last().expect("ColorMode::ALL non-empty");
        assert_eq!(last.next(), ColorMode::ALL[0]);
    }

    #[test]
    fn color_mode_name_nonempty_for_all() {
        for &mode in ColorMode::ALL {
            let n = mode.name();
            assert!(!n.is_empty(), "{mode:?}");
        }
    }

    #[test]
    fn drill_sort_mode_equality() {
        assert_eq!(DrillSortMode::Size, DrillSortMode::Size);
        assert_ne!(DrillSortMode::Size, DrillSortMode::Name);
    }

    #[test]
    fn hover_zone_equality() {
        assert_eq!(HoverZone::None, HoverZone::None);
        assert_eq!(HoverZone::DiskRow(3), HoverZone::DiskRow(3));
        assert_ne!(HoverZone::DiskRow(0), HoverZone::DiskRow(1));
        assert_ne!(HoverZone::TitleBar, HoverZone::FooterBar);
    }

    #[test]
    fn view_mode_equality() {
        assert_eq!(ViewMode::Disks, ViewMode::Disks);
        assert_ne!(ViewMode::Disks, ViewMode::DrillDown);
    }

    #[test]
    fn smart_health_exhaustive_eq() {
        assert_eq!(SmartHealth::Verified, SmartHealth::Verified);
        assert_ne!(SmartHealth::Verified, SmartHealth::Failing);
        assert_ne!(SmartHealth::Failing, SmartHealth::Unknown);
        assert_ne!(SmartHealth::Unknown, SmartHealth::Verified);
    }

    #[test]
    fn dir_entry_clone() {
        let d = DirEntry {
            path: "/a/b".into(),
            name: "b".into(),
            size: 99,
            is_dir: true,
        };
        let c = d.clone();
        assert_eq!(c.path, d.path);
        assert_eq!(c.size, 99);
        assert!(c.is_dir);
    }

    #[test]
    fn theme_colors_serde_json_roundtrip() {
        let t = ThemeColors {
            blue: 10,
            green: 20,
            purple: 30,
            light_purple: 40,
            royal: 50,
            dark_purple: 60,
        };
        let s = serde_json::to_string(&t).unwrap();
        let u: ThemeColors = serde_json::from_str(&s).unwrap();
        assert_eq!(u.blue, t.blue);
        assert_eq!(u.green, t.green);
        assert_eq!(u.purple, t.purple);
        assert_eq!(u.light_purple, t.light_purple);
        assert_eq!(u.royal, t.royal);
        assert_eq!(u.dark_purple, t.dark_purple);
    }

    #[test]
    fn disk_entry_smart_status_roundtrip_clone() {
        let d = DiskEntry {
            mount: "/".into(),
            used: 1,
            total: 2,
            pct: 50.0,
            kind: DiskKind::SSD,
            fs: "ext4".into(),
            latency_ms: Some(1.5),
            io_read_rate: Some(100.0),
            io_write_rate: Some(200.0),
            smart_status: Some(SmartHealth::Failing),
        };
        let c = d.clone();
        assert_eq!(c.smart_status, Some(SmartHealth::Failing));
        assert_eq!(c.io_read_rate, Some(100.0));
    }

    #[test]
    fn drag_target_is_copy() {
        let a = DragTarget::MountSep;
        let _b = a;
        let _c = a;
    }
}
