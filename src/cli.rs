use clap::Parser;

use crate::prefs::Prefs;
use crate::types::*;

const CYBERPUNK_BANNER: &str = r#"
 ╔═══════════════════════════════════════════════════════════════════╗
 ║  ███████╗████████╗ ██████╗ ██████╗  █████╗  ██████╗ ███████╗    ║
 ║  ██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██╔══██╗██╔════╝ ██╔════╝    ║
 ║  ███████╗   ██║   ██║   ██║██████╔╝███████║██║  ███╗█████╗      ║
 ║  ╚════██║   ██║   ██║   ██║██╔══██╗██╔══██║██║   ██║██╔══╝      ║
 ║  ███████║   ██║   ╚██████╔╝██║  ██║██║  ██║╚██████╔╝███████╗    ║
 ║  ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝    ║
 ║        ███████╗██╗  ██╗ ██████╗ ██╗    ██╗███████╗██████╗        ║
 ║        ██╔════╝██║  ██║██╔═══██╗██║    ██║██╔════╝██╔══██╗       ║
 ║        ███████╗███████║██║   ██║██║ █╗ ██║█████╗  ██████╔╝       ║
 ║        ╚════██║██╔══██║██║   ██║██║███╗██║██╔══╝  ██╔══██╗       ║
 ║        ███████║██║  ██║╚██████╔╝╚███╔███╔╝███████╗██║  ██║       ║
 ║        ╚══════╝╚═╝  ╚═╝ ╚═════╝  ╚══╝╚══╝ ╚══════╝╚═╝  ╚═╝       ║
 ╠═══════════════════════════════════════════════════════════════════╣
 ║  [NETRUNNER DISK MONITOR v0.1.0]     .: JACK IN TO YOUR CHROME :.║
 ╚═══════════════════════════════════════════════════════════════════╝"#;

const CYBERPUNK_AFTER_HELP: &str = r#"
 ┌─────────────────────────────────────────────────────────────────┐
 │  ░▒▓ INTERACTIVE KEYBINDS ▓▒░                                  │
 │                                                                 │
 │  [q/Esc] Flatline (quit)    [j/k] Scroll the datastream        │
 │  [s] Cycle sort ICE          [r] Reverse sort polarity          │
 │  [b] Swap bar firmware       [c] Shift chroma palette           │
 │  [u] Toggle used/total       [a] Toggle all/local netlinks      │
 │  [m] Toggle full mount path  [/] Enter filter daemon            │
 │  [p] Pause data feed         [?] Open help overlay              │
 │                                                                 │
 │  Config synced to: ~/.storageshower.conf                        │
 │                                                                 │
 │  .: WAKE UP SAMURAI, WE HAVE DISKS TO MONITOR :.               │
 └─────────────────────────────────────────────────────────────────┘"#;

/// Cyberpunk disk usage TUI — jack in to your chrome and monitor the datastream
#[derive(Parser, Debug)]
#[command(
    name = "storageshower",
    version,
    about = "Cyberpunk disk usage TUI — jack in to your chrome and monitor the datastream",
    before_help = CYBERPUNK_BANNER,
    after_help = CYBERPUNK_AFTER_HELP,
    styles = cyberpunk_styles(),
)]
pub struct Cli {
    /// Sort mode for disk entries [name, pct, size]
    #[arg(short = 's', long = "sort", value_name = "MODE")]
    pub sort_mode: Option<CliSortMode>,

    /// Reverse sort order
    #[arg(short = 'R', long = "reverse")]
    pub sort_rev: Option<bool>,

    /// Show only local disks (HDD/SSD), filter out network/virtual
    #[arg(short = 'l', long = "local-only")]
    pub show_local: Option<bool>,

    /// Data refresh interval in seconds [1, 2, 5, 10]
    #[arg(short = 'r', long = "refresh", value_name = "SECS")]
    pub refresh_rate: Option<u64>,

    /// Bar visualization style [gradient, solid, thin, ascii]
    #[arg(short = 'b', long = "bar-style", value_name = "STYLE")]
    pub bar_style: Option<CliBarStyle>,

    /// Color palette [default, green, blue, purple]
    #[arg(short = 'c', long = "color", value_name = "PALETTE")]
    pub color_mode: Option<CliColorMode>,

    /// Warning threshold percentage (disk usage)
    #[arg(short = 'w', long = "warn", value_name = "PCT")]
    pub thresh_warn: Option<u8>,

    /// Critical threshold percentage (disk usage)
    #[arg(short = 'C', long = "crit", value_name = "PCT")]
    pub thresh_crit: Option<u8>,

    /// Hide usage bars
    #[arg(long = "no-bars")]
    pub no_bars: bool,

    /// Hide border chrome
    #[arg(long = "no-border")]
    pub no_border: bool,

    /// Hide column headers
    #[arg(long = "no-header")]
    pub no_header: bool,

    /// Compact mount names
    #[arg(short = 'k', long = "compact")]
    pub compact: Option<bool>,

    /// Hide used/total size display
    #[arg(long = "no-used")]
    pub no_used: bool,

    /// Show full mount paths
    #[arg(short = 'f', long = "full-mount")]
    pub full_mount: Option<bool>,

    /// Hide virtual filesystems (tmpfs, devfs, etc.)
    #[arg(long = "no-virtual")]
    pub no_virtual: bool,

    /// Unit display mode [human, gib, mib, bytes]
    #[arg(short = 'u', long = "units", value_name = "MODE")]
    pub unit_mode: Option<CliUnitMode>,

    /// Mount column width (0 = auto)
    #[arg(long = "col-mount", value_name = "WIDTH")]
    pub col_mount_w: Option<u16>,

    /// Bar-end column width (0 = auto)
    #[arg(long = "col-bar-end", value_name = "WIDTH")]
    pub col_bar_end_w: Option<u16>,

    /// Percentage column width (0 = auto)
    #[arg(long = "col-pct", value_name = "WIDTH")]
    pub col_pct_w: Option<u16>,

    /// Config file path (default: ~/.storageshower.conf)
    #[arg(long = "config", value_name = "PATH")]
    pub config: Option<String>,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum CliSortMode {
    Name,
    Pct,
    Size,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum CliBarStyle {
    Gradient,
    Solid,
    Thin,
    Ascii,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum CliColorMode {
    Default,
    Green,
    Blue,
    Purple,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum CliUnitMode {
    Human,
    Gib,
    Mib,
    Bytes,
}

fn cyberpunk_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(
            clap::builder::styling::AnsiColor::Magenta
                .on_default()
                .bold(),
        )
        .usage(
            clap::builder::styling::AnsiColor::Cyan
                .on_default()
                .bold(),
        )
        .literal(
            clap::builder::styling::AnsiColor::Green
                .on_default()
                .bold(),
        )
        .placeholder(
            clap::builder::styling::AnsiColor::Yellow
                .on_default(),
        )
        .valid(
            clap::builder::styling::AnsiColor::Cyan
                .on_default()
                .bold(),
        )
        .invalid(
            clap::builder::styling::AnsiColor::Red
                .on_default()
                .bold(),
        )
        .error(
            clap::builder::styling::AnsiColor::Red
                .on_default()
                .bold(),
        )
}

impl Cli {
    /// Apply CLI overrides on top of loaded prefs. CLI flags take priority.
    pub fn apply_to(&self, prefs: &mut Prefs) {
        if let Some(m) = self.sort_mode {
            prefs.sort_mode = match m {
                CliSortMode::Name => SortMode::Name,
                CliSortMode::Pct => SortMode::Pct,
                CliSortMode::Size => SortMode::Size,
            };
        }
        if let Some(v) = self.sort_rev {
            prefs.sort_rev = v;
        }
        if let Some(v) = self.show_local {
            prefs.show_local = v;
        }
        if let Some(v) = self.refresh_rate {
            prefs.refresh_rate = v;
        }
        if let Some(s) = self.bar_style {
            prefs.bar_style = match s {
                CliBarStyle::Gradient => BarStyle::Gradient,
                CliBarStyle::Solid => BarStyle::Solid,
                CliBarStyle::Thin => BarStyle::Thin,
                CliBarStyle::Ascii => BarStyle::Ascii,
            };
        }
        if let Some(c) = self.color_mode {
            prefs.color_mode = match c {
                CliColorMode::Default => ColorMode::Default,
                CliColorMode::Green => ColorMode::Green,
                CliColorMode::Blue => ColorMode::Blue,
                CliColorMode::Purple => ColorMode::Purple,
            };
        }
        if let Some(v) = self.thresh_warn {
            prefs.thresh_warn = v;
        }
        if let Some(v) = self.thresh_crit {
            prefs.thresh_crit = v;
        }
        if self.no_bars {
            prefs.show_bars = false;
        }
        if self.no_border {
            prefs.show_border = false;
        }
        if self.no_header {
            prefs.show_header = false;
        }
        if let Some(v) = self.compact {
            prefs.compact = v;
        }
        if self.no_used {
            prefs.show_used = false;
        }
        if let Some(v) = self.full_mount {
            prefs.full_mount = v;
        }
        if self.no_virtual {
            prefs.show_all = false;
        }
        if let Some(u) = self.unit_mode {
            prefs.unit_mode = match u {
                CliUnitMode::Human => UnitMode::Human,
                CliUnitMode::Gib => UnitMode::GiB,
                CliUnitMode::Mib => UnitMode::MiB,
                CliUnitMode::Bytes => UnitMode::Bytes,
            };
        }
        if let Some(v) = self.col_mount_w {
            prefs.col_mount_w = v;
        }
        if let Some(v) = self.col_bar_end_w {
            prefs.col_bar_end_w = v;
        }
        if let Some(v) = self.col_pct_w {
            prefs.col_pct_w = v;
        }
    }
}
