use clap::Parser;

use crate::prefs::Prefs;
use crate::types::*;

/// Cyberpunk disk usage TUI
#[derive(Parser, Debug)]
#[command(
    name = "storageshower",
    version,
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Cli {
    /// Sort mode for disk entries
    #[arg(short = 's', long = "sort", value_name = "MODE")]
    pub sort_mode: Option<SortMode>,

    /// Reverse sort order
    #[arg(short = 'R', long = "reverse", overrides_with = "no_reverse")]
    pub sort_rev: bool,

    /// Do not reverse sort order
    #[arg(long = "no-reverse", overrides_with = "sort_rev", hide = true)]
    pub no_reverse: bool,

    /// Show only local disks (HDD/SSD)
    #[arg(short = 'l', long = "local-only", overrides_with = "no_local")]
    pub show_local: bool,

    /// Show all disks (not just local)
    #[arg(long = "no-local", overrides_with = "show_local", hide = true)]
    pub no_local: bool,

    /// Data refresh interval in seconds
    #[arg(short = 'r', long = "refresh", value_name = "SECS")]
    pub refresh_rate: Option<u64>,

    /// Bar visualization style
    #[arg(short = 'b', long = "bar-style", value_name = "STYLE")]
    pub bar_style: Option<BarStyle>,

    /// Color palette
    #[arg(long = "color", value_name = "PALETTE")]
    pub color_mode: Option<ColorMode>,

    /// Warning threshold percentage
    #[arg(short = 'w', long = "warn", value_name = "PCT")]
    pub thresh_warn: Option<u8>,

    /// Critical threshold percentage
    #[arg(short = 'C', long = "crit", value_name = "PCT")]
    pub thresh_crit: Option<u8>,

    /// Show usage bars
    #[arg(long = "bars", overrides_with = "no_bars", hide = true)]
    pub bars: bool,

    /// Hide usage bars
    #[arg(long = "no-bars", overrides_with = "bars")]
    pub no_bars: bool,

    /// Show border chrome
    #[arg(long = "border", overrides_with = "no_border", hide = true)]
    pub border: bool,

    /// Hide border chrome
    #[arg(long = "no-border", overrides_with = "border")]
    pub no_border: bool,

    /// Show column headers
    #[arg(long = "header", overrides_with = "no_header", hide = true)]
    pub header: bool,

    /// Hide column headers
    #[arg(long = "no-header", overrides_with = "header")]
    pub no_header: bool,

    /// Compact mount names
    #[arg(short = 'k', long = "compact", overrides_with = "no_compact")]
    pub compact: bool,

    /// Do not compact mount names
    #[arg(long = "no-compact", overrides_with = "compact", hide = true)]
    pub no_compact: bool,

    /// Show used/total size display
    #[arg(long = "used", overrides_with = "no_used", hide = true)]
    pub used: bool,

    /// Hide used/total size display
    #[arg(long = "no-used", overrides_with = "used")]
    pub no_used: bool,

    /// Show full mount paths
    #[arg(short = 'f', long = "full-mount", overrides_with = "no_full_mount")]
    pub full_mount: bool,

    /// Do not show full mount paths
    #[arg(long = "no-full-mount", overrides_with = "full_mount", hide = true)]
    pub no_full_mount: bool,

    /// Show hover tooltips on title/footer bars
    #[arg(long = "tooltips", overrides_with = "no_tooltips", hide = true)]
    pub tooltips: bool,

    /// Hide hover tooltips (right-click still works)
    #[arg(long = "no-tooltips", overrides_with = "tooltips")]
    pub no_tooltips: bool,

    /// Show virtual filesystems
    #[arg(long = "virtual", overrides_with = "no_virtual", hide = true)]
    pub show_virtual: bool,

    /// Hide virtual filesystems
    #[arg(long = "no-virtual", overrides_with = "show_virtual")]
    pub no_virtual: bool,

    /// Unit display mode
    #[arg(short = 'u', long = "units", value_name = "MODE")]
    pub unit_mode: Option<UnitMode>,

    /// Mount column width (0 = auto)
    #[arg(long = "col-mount", value_name = "WIDTH")]
    pub col_mount_w: Option<u16>,

    /// Bar-end column width (0 = auto)
    #[arg(long = "col-bar-end", value_name = "WIDTH")]
    pub col_bar_end_w: Option<u16>,

    /// Percentage column width (0 = auto)
    #[arg(long = "col-pct", value_name = "WIDTH")]
    pub col_pct_w: Option<u16>,

    /// Config file path
    #[arg(short = 'c', long = "config", value_name = "PATH")]
    pub config: Option<String>,

    /// Display this transmission
    #[arg(short = 'h', long = "help")]
    pub help: bool,

    /// Display version information
    #[arg(short = 'V', long = "version")]
    pub version: bool,

    /// List all builtin color schemes
    #[arg(long = "list-colors")]
    pub list_colors: bool,

    /// Activate a custom theme by name (defined in config file)
    #[arg(long = "theme", value_name = "NAME")]
    pub theme: Option<String>,

    /// Export the current or named theme as TOML
    #[arg(long = "export-theme")]
    pub export_theme: bool,

    /// RECLAIM_MAP: estimate reclaimable (compressible) space per subtree during
    /// drill-down, via bounded-prefix sampling. Adds a reclaim overlay + sort.
    #[cfg(feature = "reclaim")]
    #[arg(long = "reclaim")]
    pub reclaim: bool,
}

// ANSI color constants
const RST: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const B_CYAN: &str = "\x1b[1;36m";
const B_MAGENTA: &str = "\x1b[1;35m";
const B_GREEN: &str = "\x1b[1;32m";
const B_YELLOW: &str = "\x1b[1;33m";

pub fn print_help() {
    println!("{}", help_text());
}

/// The full `--help` transmission as a string.
///
/// Split out from [`print_help`] so tests can assert the text stays in step
/// with the actual flag set and key handlers instead of drifting silently.
pub fn help_text() -> String {
    let ver = env!("CARGO_PKG_VERSION");
    // Status box, padded at runtime so its right border never drifts as
    // VERSION grows. BOX_W tracks the banner's display width (STORAGE row).
    const BOX_W: usize = 58;
    let status = format!(" STATUS: ONLINE  // SIGNAL: ████████░░ // v{ver}");
    let space = " ".repeat(BOX_W.saturating_sub(status.chars().count()));
    let rule = "─".repeat(BOX_W);
    #[allow(unused_mut)]
    let mut out = format!(
        "
{CYAN}  ███████╗████████╗ ██████╗ ██████╗  █████╗  ██████╗ ███████╗{RST}
{CYAN}  ██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██╔══██╗██╔════╝ ██╔════╝{RST}
{MAGENTA}  ███████╗   ██║   ██║   ██║██████╔╝███████║██║  ███╗█████╗  {RST}
{MAGENTA}  ╚════██║   ██║   ██║   ██║██╔══██╗██╔══██║██║   ██║██╔══╝  {RST}
{RED}  ███████║   ██║   ╚██████╔╝██║  ██║██║  ██║╚██████╔╝███████╗{RST}
{RED}  ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝{RST}
{YELLOW}        ███████╗██╗  ██╗ ██████╗ ██╗    ██╗███████╗██████╗ {RST}
{YELLOW}        ██╔════╝██║  ██║██╔═══██╗██║    ██║██╔════╝██╔══██╗{RST}
{YELLOW}        ███████╗███████║██║   ██║██║ █╗ ██║█████╗  ██████╔╝{RST}
{YELLOW}        ╚════██║██╔══██║██║   ██║██║███╗██║██╔══╝  ██╔══██╗{RST}
{YELLOW}        ███████║██║  ██║╚██████╔╝╚███╔███╔╝███████╗██║  ██║{RST}
{YELLOW}        ╚══════╝╚═╝  ╚═╝ ╚═════╝  ╚══╝╚══╝ ╚══════╝╚═╝  ╚═╝{RST}

 {CYAN}┌{rule}┐{RST}
 {CYAN}│{RST}{status}{space}{CYAN}│{RST}
 {CYAN}└{rule}┘{RST}

{B_MAGENTA}  >> NETRUNNER DISK MONITOR v{ver} <<{RST}
{B_MAGENTA}  [ jack in to your chrome and monitor the datastream ]{RST}

{B_YELLOW}  USAGE:{RST} storageshower [OPTIONS]

{B_CYAN}  ── SORTING ───────────────────────────────────────{RST}
  -s, --sort MODE          \x1b[32m//\x1b[0m sort disk entries {B_MAGENTA}(name, pct, size){RST}
  -R, --reverse            \x1b[32m//\x1b[0m reverse sort order
  -l, --local-only         \x1b[32m//\x1b[0m show only local disks {B_MAGENTA}(HDD/SSD){RST}
      --no-virtual         \x1b[32m//\x1b[0m hide virtual filesystems {B_MAGENTA}(tmpfs, devfs, etc.){RST}

{B_CYAN}  ── DISPLAY ───────────────────────────────────────{RST}
  -b, --bar-style STYLE    \x1b[32m//\x1b[0m bar visualization {B_MAGENTA}(gradient, solid, thin, ascii){RST}
      --color PALETTE      \x1b[32m//\x1b[0m color palette {B_MAGENTA}(default, green, blue, purple, ...){RST}
      --list-colors        \x1b[32m//\x1b[0m list all builtin color schemes
      --export-theme       \x1b[32m//\x1b[0m export current palette as TOML
      --theme NAME         \x1b[32m//\x1b[0m activate a custom theme by name
  -u, --units MODE         \x1b[32m//\x1b[0m unit display {B_MAGENTA}(human, gib, mib, bytes){RST}
  -k, --compact            \x1b[32m//\x1b[0m compact mount names
  -f, --full-mount         \x1b[32m//\x1b[0m show full mount paths
      --no-bars            \x1b[32m//\x1b[0m hide usage bars
      --no-border          \x1b[32m//\x1b[0m hide border chrome
      --no-header          \x1b[32m//\x1b[0m hide column headers
      --no-used            \x1b[32m//\x1b[0m hide used/total size display
      --no-tooltips        \x1b[32m//\x1b[0m hide hover tooltips {B_MAGENTA}(right-click still works){RST}

{B_CYAN}  ── THRESHOLDS ────────────────────────────────────{RST}
  -w, --warn PCT           \x1b[32m//\x1b[0m warning threshold {B_MAGENTA}(default: 70%){RST}
  -C, --crit PCT           \x1b[32m//\x1b[0m critical threshold {B_MAGENTA}(default: 90%){RST}

{B_CYAN}  ── COLUMNS ───────────────────────────────────────{RST}
      --col-mount WIDTH    \x1b[32m//\x1b[0m mount column width {B_MAGENTA}(0 = auto){RST}
      --col-bar-end WIDTH  \x1b[32m//\x1b[0m bar-end column width {B_MAGENTA}(0 = auto){RST}
      --col-pct WIDTH      \x1b[32m//\x1b[0m percentage column width {B_MAGENTA}(0 = auto){RST}

{B_CYAN}  ── GENERAL ───────────────────────────────────────{RST}
  -r, --refresh SECS       \x1b[32m//\x1b[0m data refresh interval {B_MAGENTA}(default: 1s){RST}
  -c, --config PATH        \x1b[32m//\x1b[0m config file path {B_MAGENTA}(default: ~/.storageshower.conf){RST}
  -h, --help               \x1b[32m//\x1b[0m display this transmission
  -V, --version            \x1b[32m//\x1b[0m display version information

{B_CYAN}  ── KEYBINDS ──────────────────────────────────────{RST}
  q / Q                    \x1b[32m//\x1b[0m flatline {B_MAGENTA}(quit){RST}
  Esc                      \x1b[32m//\x1b[0m deselect the current disk
  j / k                    \x1b[32m//\x1b[0m scroll the datastream
  Enter                    \x1b[32m//\x1b[0m drill down into the selected mount
  n / u / s                \x1b[32m//\x1b[0m sort by name / usage % / size
  r                        \x1b[32m//\x1b[0m reverse sort polarity
  b                        \x1b[32m//\x1b[0m swap bar firmware
  c / C                    \x1b[32m//\x1b[0m theme chooser popup / theme editor
  i                        \x1b[32m//\x1b[0m cycle units {B_MAGENTA}(human, GiB, MiB, bytes){RST}
  d                        \x1b[32m//\x1b[0m toggle used/total
  m / w                    \x1b[32m//\x1b[0m compact mount names / full mount paths
  l / a                    \x1b[32m//\x1b[0m local disks only / all filesystems
  /                        \x1b[32m//\x1b[0m enter filter daemon
  p                        \x1b[32m//\x1b[0m pause data feed
  h / ?                    \x1b[32m//\x1b[0m open help overlay {B_MAGENTA}(full keybind matrix){RST}

{B_CYAN}  ── EXAMPLES ──────────────────────────────────────{RST}
  storageshower --color purple -b ascii \x1b[32m//\x1b[0m purple palette with ascii bars
  storageshower -s pct -R               \x1b[32m//\x1b[0m sort by usage%, reversed
  storageshower -l --no-virtual         \x1b[32m//\x1b[0m local physical disks only
  storageshower -u gib -w 60 -C 85      \x1b[32m//\x1b[0m GiB units, custom thresholds
  storageshower --config /tmp/ss.conf   \x1b[32m//\x1b[0m use alternate config

{B_CYAN}  ── SYSTEM ────────────────────────────────────────{RST}
  {B_MAGENTA}v{ver} {RST}\x1b[32m//\x1b[0m {B_YELLOW}(c) MenkeTechnologies{RST}
  {B_MAGENTA}Config synced to ~/.storageshower.conf — CLI flags override it.{RST}
  {B_YELLOW}>>> WAKE UP, SAMURAI. WE HAVE DISKS TO MONITOR. <<<{RST}
 {CYAN}░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░{RST}
"
    );
    #[cfg(feature = "reclaim")]
    out.push_str(
        &format!(
            "{B_CYAN}  ── RECLAIM_MAP ───────────────────────────────────{RST}
      --reclaim            \x1b[32m//\x1b[0m estimate reclaimable (compressible) space per subtree
                           \x1b[32m//\x1b[0m in drill-down; adds a reclaim overlay + sort
  {B_MAGENTA}drill key{RST} c            \x1b[32m//\x1b[0m toggle the reclaim overlay / sort in drill-down
"
        ),
    );
    out
}

pub fn print_version() {
    println!(
        "{B_CYAN}storageshower{RST} {B_MAGENTA}v{ver}{RST}",
        ver = env!("CARGO_PKG_VERSION"),
    );
}

pub fn print_colors() {
    print!("{}", colors_text());
}

/// The `--list-colors` table as a string.
///
/// Split out from [`print_colors`] so tests can assert every printed flag is
/// a value `--color` actually accepts.
pub fn colors_text() -> String {
    use crate::ui::palette;
    use ratatui::style::Color;
    use std::fmt::Write;

    fn idx(c: Color) -> u8 {
        match c {
            Color::Indexed(n) => n,
            _ => 0,
        }
    }

    // Column widths are measured from the data so the table stays aligned as
    // palettes are added or renamed, instead of drifting past a fixed pad.
    let flag_w = ColorMode::ALL
        .iter()
        .map(|&m| m.flag().chars().count())
        .max()
        .unwrap_or(0);
    let name_w = ColorMode::ALL
        .iter()
        .map(|&m| m.name().chars().count())
        .max()
        .unwrap_or(0);

    let mut out = format!("\n{B_CYAN}  ── BUILTIN COLOR SCHEMES ────────────────────────{RST}\n\n");
    for &mode in ColorMode::ALL {
        let (a, b, c, d, e, f) = palette(mode);
        let swatch: String = [a, b, c, d, e, f]
            .iter()
            .map(|&col| format!("\x1b[48;5;{}m   {RST}", idx(col)))
            .collect();
        let _ = writeln!(
            out,
            "  {B_GREEN}{flag:<flag_w$}{RST} {B_MAGENTA}{name:<name_w$}{RST} {swatch}",
            flag = mode.flag(),
            name = mode.name(),
        );
    }
    let _ = writeln!(
        out,
        "\n  {B_YELLOW}Usage:{RST}   storageshower {B_GREEN}--color{RST} {B_MAGENTA}<flag>{RST}"
    );
    let _ = writeln!(
        out,
        "  {B_YELLOW}Chooser:{RST} press {B_GREEN}c{RST} in the TUI for the live theme picker\n"
    );
    out
}

pub fn print_export_theme(prefs: &Prefs) {
    use crate::ui::{palette, palette_for_prefs};
    use ratatui::style::Color;

    fn idx(c: Color) -> u8 {
        match c {
            Color::Indexed(n) => n,
            _ => 0,
        }
    }

    let (name, colors) = if let Some(ref theme_name) = prefs.active_theme {
        if let Some(theme) = prefs.custom_themes.get(theme_name) {
            (
                theme_name.clone(),
                [
                    theme.blue,
                    theme.green,
                    theme.purple,
                    theme.light_purple,
                    theme.royal,
                    theme.dark_purple,
                ],
            )
        } else {
            let (a, b, c, d, e, f) = palette(prefs.color_mode);
            (
                prefs.color_mode.name().to_string(),
                [idx(a), idx(b), idx(c), idx(d), idx(e), idx(f)],
            )
        }
    } else {
        let (a, b, c, d, e, f) = palette_for_prefs(prefs);
        (
            prefs.color_mode.name().to_string(),
            [idx(a), idx(b), idx(c), idx(d), idx(e), idx(f)],
        )
    };

    let safe_name: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();

    println!("# {} — exported from storageshower", name);
    println!("[custom_themes.{}]", safe_name);
    println!("blue         = {}", colors[0]);
    println!("green        = {}", colors[1]);
    println!("purple       = {}", colors[2]);
    println!("light_purple = {}", colors[3]);
    println!("royal        = {}", colors[4]);
    println!("dark_purple  = {}", colors[5]);
    println!();
    println!("# Paste into ~/.storageshower.conf and set:");
    println!("# active_theme = \"{}\"", safe_name);
}

impl Cli {
    /// Apply CLI overrides on top of loaded prefs. CLI flags take priority.
    pub fn apply_to(&self, prefs: &mut Prefs) {
        #[cfg(feature = "reclaim")]
        if self.reclaim {
            prefs.reclaim = true;
        }
        if let Some(v) = self.sort_mode {
            prefs.sort_mode = v;
        }
        if let Some(v) = self.refresh_rate {
            prefs.refresh_rate = v;
        }
        if let Some(v) = self.bar_style {
            prefs.bar_style = v;
        }
        if let Some(v) = self.color_mode {
            prefs.color_mode = v;
        }
        if let Some(v) = self.thresh_warn {
            prefs.thresh_warn = v;
        }
        if let Some(v) = self.thresh_crit {
            prefs.thresh_crit = v;
        }
        if let Some(v) = self.unit_mode {
            prefs.unit_mode = v;
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
        // Boolean pairs: --flag / --no-flag (last one wins via clap overrides_with)
        if self.sort_rev {
            prefs.sort_rev = true;
        }
        if self.no_reverse {
            prefs.sort_rev = false;
        }
        if self.show_local {
            prefs.show_local = true;
        }
        if self.no_local {
            prefs.show_local = false;
        }
        if self.compact {
            prefs.compact = true;
        }
        if self.no_compact {
            prefs.compact = false;
        }
        if self.full_mount {
            prefs.full_mount = true;
        }
        if self.no_full_mount {
            prefs.full_mount = false;
        }
        if self.bars {
            prefs.show_bars = true;
        }
        if self.no_bars {
            prefs.show_bars = false;
        }
        if self.border {
            prefs.show_border = true;
        }
        if self.no_border {
            prefs.show_border = false;
        }
        if self.header {
            prefs.show_header = true;
        }
        if self.no_header {
            prefs.show_header = false;
        }
        if self.used {
            prefs.show_used = true;
        }
        if self.no_used {
            prefs.show_used = false;
        }
        if self.tooltips {
            prefs.show_tooltips = true;
        }
        if self.no_tooltips {
            prefs.show_tooltips = false;
        }
        if self.show_virtual {
            prefs.show_all = true;
        }
        if self.no_virtual {
            prefs.show_all = false;
        }
        if let Some(ref name) = self.theme {
            prefs.active_theme = Some(name.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Parser, ValueEnum};

    fn default_cli() -> Cli {
        Cli::parse_from(["storageshower"])
    }

    #[test]
    fn default_cli_no_overrides() {
        let cli = default_cli();
        assert!(cli.sort_mode.is_none());
        assert!(!cli.sort_rev);
        assert!(!cli.show_local);
        assert!(cli.refresh_rate.is_none());
        assert!(cli.bar_style.is_none());
        assert!(cli.color_mode.is_none());
        assert!(cli.thresh_warn.is_none());
        assert!(cli.thresh_crit.is_none());
        assert!(!cli.no_bars);
        assert!(!cli.no_border);
        assert!(!cli.no_header);
        assert!(!cli.compact);
        assert!(!cli.no_used);
        assert!(!cli.full_mount);
        assert!(!cli.no_virtual);
        assert!(cli.unit_mode.is_none());
        assert!(cli.col_mount_w.is_none());
        assert!(cli.col_bar_end_w.is_none());
        assert!(cli.col_pct_w.is_none());
        assert!(cli.config.is_none());
        assert!(!cli.help);
        assert!(!cli.version);
        assert!(!cli.list_colors);
    }

    #[test]
    fn list_colors_flag() {
        let cli = Cli::parse_from(["storageshower", "--list-colors"]);
        assert!(cli.list_colors);
    }

    #[test]
    fn apply_sort_mode() {
        let cli = Cli::parse_from(["storageshower", "-s", "pct"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.sort_mode, SortMode::Pct);
    }

    #[test]
    fn apply_sort_mode_size() {
        let cli = Cli::parse_from(["storageshower", "--sort", "size"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.sort_mode, SortMode::Size);
    }

    #[test]
    fn apply_reverse() {
        let cli = Cli::parse_from(["storageshower", "-R"]);
        let mut prefs = Prefs::default();
        assert!(!prefs.sort_rev);
        cli.apply_to(&mut prefs);
        assert!(prefs.sort_rev);
    }

    #[test]
    fn apply_local_only() {
        let cli = Cli::parse_from(["storageshower", "-l"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert!(prefs.show_local);
    }

    #[test]
    fn apply_refresh_rate() {
        let cli = Cli::parse_from(["storageshower", "-r", "5"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.refresh_rate, 5);
    }

    #[test]
    fn apply_bar_style() {
        let cli = Cli::parse_from(["storageshower", "-b", "ascii"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.bar_style, BarStyle::Ascii);
    }

    #[test]
    fn apply_color_mode() {
        let cli = Cli::parse_from(["storageshower", "--color", "purple"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.color_mode, ColorMode::Purple);
    }

    #[test]
    fn apply_thresholds() {
        let cli = Cli::parse_from(["storageshower", "-w", "60", "-C", "85"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.thresh_warn, 60);
        assert_eq!(prefs.thresh_crit, 85);
    }

    #[test]
    fn apply_no_flags() {
        let cli = Cli::parse_from([
            "storageshower",
            "--no-bars",
            "--no-border",
            "--no-header",
            "--no-used",
            "--no-virtual",
        ]);
        let mut prefs = Prefs::default();
        assert!(prefs.show_bars);
        assert!(prefs.show_border);
        assert!(prefs.show_header);
        assert!(prefs.show_used);
        assert!(prefs.show_all);
        cli.apply_to(&mut prefs);
        assert!(!prefs.show_bars);
        assert!(!prefs.show_border);
        assert!(!prefs.show_header);
        assert!(!prefs.show_used);
        assert!(!prefs.show_all);
    }

    #[test]
    fn apply_compact_and_full_mount() {
        let cli = Cli::parse_from(["storageshower", "-k", "-f"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert!(prefs.compact);
        assert!(prefs.full_mount);
    }

    #[test]
    fn apply_unit_mode_gib() {
        let cli = Cli::parse_from(["storageshower", "-u", "gib"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.unit_mode, UnitMode::GiB);
    }

    #[test]
    fn apply_unit_mode_mib() {
        let cli = Cli::parse_from(["storageshower", "--units", "mib"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.unit_mode, UnitMode::MiB);
    }

    #[test]
    fn apply_unit_mode_bytes() {
        let cli = Cli::parse_from(["storageshower", "-u", "bytes"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.unit_mode, UnitMode::Bytes);
    }

    #[test]
    fn apply_column_widths() {
        let cli = Cli::parse_from([
            "storageshower",
            "--col-mount",
            "25",
            "--col-bar-end",
            "30",
            "--col-pct",
            "8",
        ]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.col_mount_w, 25);
        assert_eq!(prefs.col_bar_end_w, 30);
        assert_eq!(prefs.col_pct_w, 8);
    }

    #[test]
    fn apply_config_path() {
        let cli = Cli::parse_from(["storageshower", "--config", "/tmp/test.conf"]);
        assert_eq!(cli.config.as_deref(), Some("/tmp/test.conf"));
    }

    #[test]
    fn help_flag() {
        let cli = Cli::parse_from(["storageshower", "-h"]);
        assert!(cli.help);
    }

    #[test]
    fn version_flag() {
        let cli = Cli::parse_from(["storageshower", "-V"]);
        assert!(cli.version);
    }

    #[test]
    fn no_override_preserves_prefs() {
        let cli = default_cli();
        let mut prefs = Prefs::default();
        prefs.sort_mode = SortMode::Size;
        prefs.bar_style = BarStyle::Thin;
        prefs.color_mode = ColorMode::Blue;
        prefs.refresh_rate = 10;
        cli.apply_to(&mut prefs);
        // None of these should change
        assert_eq!(prefs.sort_mode, SortMode::Size);
        assert_eq!(prefs.bar_style, BarStyle::Thin);
        assert_eq!(prefs.color_mode, ColorMode::Blue);
        assert_eq!(prefs.refresh_rate, 10);
    }

    #[test]
    fn all_bar_styles_parse() {
        for style in ["gradient", "solid", "thin", "ascii"] {
            let cli = Cli::parse_from(["storageshower", "-b", style]);
            assert!(cli.bar_style.is_some());
        }
    }

    #[test]
    fn all_color_modes_parse() {
        for color in [
            "default", "green", "blue", "purple", "amber", "cyan", "red", "sakura", "matrix",
            "sunset",
        ] {
            let cli = Cli::parse_from(["storageshower", "--color", color]);
            assert!(cli.color_mode.is_some());
        }
    }

    #[test]
    fn all_sort_modes_parse() {
        for mode in ["name", "pct", "size"] {
            let cli = Cli::parse_from(["storageshower", "-s", mode]);
            assert!(cli.sort_mode.is_some());
        }
    }

    #[test]
    fn all_unit_modes_parse() {
        for mode in ["human", "gib", "mib", "bytes"] {
            let cli = Cli::parse_from(["storageshower", "-u", mode]);
            assert!(cli.unit_mode.is_some());
        }
    }

    #[test]
    fn combined_flags() {
        let cli = Cli::parse_from([
            "storageshower",
            "-s",
            "pct",
            "-R",
            "-l",
            "-b",
            "thin",
            "--color",
            "green",
            "-u",
            "gib",
            "-k",
            "-f",
            "-w",
            "50",
            "-C",
            "80",
            "-r",
            "2",
            "--no-bars",
            "--no-border",
        ]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.sort_mode, SortMode::Pct);
        assert!(prefs.sort_rev);
        assert!(prefs.show_local);
        assert_eq!(prefs.bar_style, BarStyle::Thin);
        assert_eq!(prefs.color_mode, ColorMode::Green);
        assert_eq!(prefs.unit_mode, UnitMode::GiB);
        assert!(prefs.compact);
        assert!(prefs.full_mount);
        assert_eq!(prefs.thresh_warn, 50);
        assert_eq!(prefs.thresh_crit, 80);
        assert_eq!(prefs.refresh_rate, 2);
        assert!(!prefs.show_bars);
        assert!(!prefs.show_border);
    }

    #[test]
    fn invalid_sort_mode_errors() {
        let result = Cli::try_parse_from(["storageshower", "-s", "invalid"]);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_bar_style_errors() {
        let result = Cli::try_parse_from(["storageshower", "-b", "nope"]);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_color_mode_errors() {
        let result = Cli::try_parse_from(["storageshower", "--color", "rainbow"]);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_unit_mode_errors() {
        let result = Cli::try_parse_from(["storageshower", "-u", "petabytes"]);
        assert!(result.is_err());
    }

    // ── Counter-flags override config values ──────────────

    #[test]
    fn no_reverse_overrides_config() {
        let cli = Cli::parse_from(["storageshower", "--no-reverse"]);
        let mut prefs = Prefs::default();
        prefs.sort_rev = true; // config says reversed
        cli.apply_to(&mut prefs);
        assert!(!prefs.sort_rev); // CLI overrides
    }

    #[test]
    fn no_local_overrides_config() {
        let cli = Cli::parse_from(["storageshower", "--no-local"]);
        let mut prefs = Prefs::default();
        prefs.show_local = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.show_local);
    }

    #[test]
    fn no_compact_overrides_config() {
        let cli = Cli::parse_from(["storageshower", "--no-compact"]);
        let mut prefs = Prefs::default();
        prefs.compact = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.compact);
    }

    #[test]
    fn no_full_mount_overrides_config() {
        let cli = Cli::parse_from(["storageshower", "--no-full-mount"]);
        let mut prefs = Prefs::default();
        prefs.full_mount = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.full_mount);
    }

    #[test]
    fn bars_overrides_config_no_bars() {
        let cli = Cli::parse_from(["storageshower", "--bars"]);
        let mut prefs = Prefs::default();
        prefs.show_bars = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_bars);
    }

    #[test]
    fn border_overrides_config_no_border() {
        let cli = Cli::parse_from(["storageshower", "--border"]);
        let mut prefs = Prefs::default();
        prefs.show_border = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_border);
    }

    #[test]
    fn header_overrides_config_no_header() {
        let cli = Cli::parse_from(["storageshower", "--header"]);
        let mut prefs = Prefs::default();
        prefs.show_header = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_header);
    }

    #[test]
    fn used_overrides_config_no_used() {
        let cli = Cli::parse_from(["storageshower", "--used"]);
        let mut prefs = Prefs::default();
        prefs.show_used = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_used);
    }

    #[test]
    fn virtual_overrides_config_no_virtual() {
        let cli = Cli::parse_from(["storageshower", "--virtual"]);
        let mut prefs = Prefs::default();
        prefs.show_all = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_all);
    }

    #[test]
    fn export_theme_flag_parses() {
        let cli = Cli::parse_from(["storageshower", "--export-theme"]);
        assert!(cli.export_theme);
    }

    #[test]
    fn apply_theme_sets_active_theme() {
        let cli = Cli::parse_from(["storageshower", "--theme", "my_dark_theme"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.active_theme.as_deref(), Some("my_dark_theme"));
    }

    #[test]
    fn apply_tooltips_enables() {
        let cli = Cli::parse_from(["storageshower", "--tooltips"]);
        let mut prefs = Prefs::default();
        prefs.show_tooltips = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_tooltips);
    }

    #[test]
    fn apply_no_tooltips_disables() {
        let cli = Cli::parse_from(["storageshower", "--no-tooltips"]);
        let mut prefs = Prefs::default();
        assert!(prefs.show_tooltips);
        cli.apply_to(&mut prefs);
        assert!(!prefs.show_tooltips);
    }

    #[test]
    fn short_config_flag() {
        let cli = Cli::parse_from(["storageshower", "-c", "/path/to/x.conf"]);
        assert_eq!(cli.config.as_deref(), Some("/path/to/x.conf"));
    }

    #[test]
    fn short_sort_name() {
        let cli = Cli::parse_from(["storageshower", "-s", "name"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.sort_mode, SortMode::Name);
    }

    #[test]
    fn no_reverse_flag() {
        let cli = Cli::parse_from(["storageshower", "--no-reverse"]);
        let mut prefs = Prefs::default();
        prefs.sort_rev = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.sort_rev);
    }

    #[test]
    fn no_local_flag() {
        let cli = Cli::parse_from(["storageshower", "--no-local"]);
        let mut prefs = Prefs::default();
        prefs.show_local = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.show_local);
    }

    #[test]
    fn refresh_short_flag() {
        let cli = Cli::parse_from(["storageshower", "-r", "12"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.refresh_rate, 12);
    }

    #[test]
    fn warn_crit_short_flags() {
        let cli = Cli::parse_from(["storageshower", "-w", "55", "-C", "92"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.thresh_warn, 55);
        assert_eq!(prefs.thresh_crit, 92);
    }

    #[test]
    fn compact_short_flag() {
        let cli = Cli::parse_from(["storageshower", "-k"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert!(prefs.compact);
    }

    #[test]
    fn full_mount_short_flag() {
        let cli = Cli::parse_from(["storageshower", "-f"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert!(prefs.full_mount);
    }

    #[test]
    fn units_short_flag_mib() {
        let cli = Cli::parse_from(["storageshower", "-u", "mib"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.unit_mode, UnitMode::MiB);
    }

    #[test]
    fn list_colors_long_flag() {
        let cli = Cli::parse_from(["storageshower", "--list-colors"]);
        assert!(cli.list_colors);
    }

    #[test]
    fn export_theme_with_theme_name() {
        let cli = Cli::parse_from(["storageshower", "--export-theme", "--theme", "custom1"]);
        assert!(cli.export_theme);
        assert_eq!(cli.theme.as_deref(), Some("custom1"));
    }

    #[test]
    fn parse_multiple_display_negations() {
        let cli = Cli::parse_from([
            "storageshower",
            "--no-bars",
            "--no-border",
            "--no-header",
            "--no-used",
            "--no-tooltips",
            "--no-virtual",
        ]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert!(!prefs.show_bars);
        assert!(!prefs.show_border);
        assert!(!prefs.show_header);
        assert!(!prefs.show_used);
        assert!(!prefs.show_tooltips);
        assert!(!prefs.show_all);
    }

    #[test]
    fn parse_positive_display_overrides() {
        let cli = Cli::parse_from([
            "storageshower",
            "--bars",
            "--border",
            "--header",
            "--used",
            "--tooltips",
            "--virtual",
        ]);
        let mut prefs = Prefs::default();
        prefs.show_bars = false;
        prefs.show_border = false;
        prefs.show_header = false;
        prefs.show_used = false;
        prefs.show_tooltips = false;
        prefs.show_all = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_bars);
        assert!(prefs.show_border);
        assert!(prefs.show_header);
        assert!(prefs.show_used);
        assert!(prefs.show_tooltips);
        assert!(prefs.show_all);
    }

    #[test]
    fn invalid_refresh_nonnumeric_errors() {
        let r = Cli::try_parse_from(["storageshower", "-r", "nope"]);
        assert!(r.is_err());
    }

    #[test]
    fn invalid_thresh_warn_nonnumeric_errors() {
        let r = Cli::try_parse_from(["storageshower", "-w", "xx"]);
        assert!(r.is_err());
    }

    #[test]
    fn invalid_crit_nonnumeric_errors() {
        let r = Cli::try_parse_from(["storageshower", "-C", "??"]);
        assert!(r.is_err());
    }

    #[test]
    fn refresh_rate_zero_allowed() {
        let cli = Cli::parse_from(["storageshower", "-r", "0"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.refresh_rate, 0);
    }

    #[test]
    fn col_widths_zero_explicit() {
        let cli = Cli::parse_from([
            "storageshower",
            "--col-mount",
            "0",
            "--col-bar-end",
            "0",
            "--col-pct",
            "0",
        ]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.col_mount_w, 0);
        assert_eq!(prefs.col_bar_end_w, 0);
        assert_eq!(prefs.col_pct_w, 0);
    }

    #[test]
    fn long_sort_flag_pct() {
        let cli = Cli::parse_from(["storageshower", "--sort", "pct"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.sort_mode, SortMode::Pct);
    }

    #[test]
    fn long_local_only_flag() {
        let cli = Cli::parse_from(["storageshower", "--local-only"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert!(prefs.show_local);
    }

    #[test]
    fn long_units_human() {
        let cli = Cli::parse_from(["storageshower", "--units", "human"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.unit_mode, UnitMode::Human);
    }

    #[test]
    fn long_no_bars_combo() {
        let cli = Cli::parse_from(["storageshower", "--no-bars", "--no-header"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert!(!prefs.show_bars);
        assert!(!prefs.show_header);
    }

    #[test]
    fn long_reverse_flag_sets_sort_rev() {
        let cli = Cli::parse_from(["storageshower", "--reverse"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert!(prefs.sort_rev);
    }

    #[test]
    fn long_no_compact_flag() {
        let cli = Cli::parse_from(["storageshower", "--no-compact"]);
        let mut prefs = Prefs::default();
        prefs.compact = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.compact);
    }

    #[test]
    fn long_no_full_mount_flag() {
        let cli = Cli::parse_from(["storageshower", "--no-full-mount"]);
        let mut prefs = Prefs::default();
        prefs.full_mount = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.full_mount);
    }

    #[test]
    fn long_refresh_flag() {
        let cli = Cli::parse_from(["storageshower", "--refresh", "3"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.refresh_rate, 3);
    }

    #[test]
    fn long_warn_and_crit_flags() {
        let cli = Cli::parse_from(["storageshower", "--warn", "61", "--crit", "91"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.thresh_warn, 61);
        assert_eq!(prefs.thresh_crit, 91);
    }

    #[test]
    fn long_bar_style_and_color_flags() {
        let cli = Cli::parse_from(["storageshower", "--bar-style", "thin", "--color", "amber"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.bar_style, BarStyle::Thin);
        assert_eq!(prefs.color_mode, ColorMode::Amber);
    }

    #[test]
    fn long_sort_size_flag() {
        let cli = Cli::parse_from(["storageshower", "--sort", "size"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.sort_mode, SortMode::Size);
    }

    #[test]
    fn compact_and_full_mount_long_flags() {
        let cli = Cli::parse_from(["storageshower", "--compact", "--full-mount"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert!(prefs.compact);
        assert!(prefs.full_mount);
    }

    #[test]
    fn bars_border_header_used_long_positives() {
        let cli = Cli::parse_from(["storageshower", "--bars", "--border", "--header", "--used"]);
        let mut prefs = Prefs::default();
        prefs.show_bars = false;
        prefs.show_border = false;
        prefs.show_header = false;
        prefs.show_used = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_bars);
        assert!(prefs.show_border);
        assert!(prefs.show_header);
        assert!(prefs.show_used);
    }

    #[test]
    fn try_parse_fails_unknown_binary_name_still_storageshower() {
        let cli = Cli::try_parse_from(["prog", "-h"]);
        assert!(cli.is_ok());
        assert!(cli.unwrap().help);
    }

    #[test]
    fn long_sort_name_flag() {
        let cli = Cli::parse_from(["storageshower", "--sort", "name"]);
        let mut prefs = Prefs::default();
        prefs.sort_mode = SortMode::Pct;
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.sort_mode, SortMode::Name);
    }

    #[test]
    fn long_bar_style_solid() {
        let cli = Cli::parse_from(["storageshower", "--bar-style", "solid"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.bar_style, BarStyle::Solid);
    }

    #[test]
    fn parse_color_neon_noir_kebab() {
        let cli = Cli::parse_from(["storageshower", "--color", "neon-noir"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.color_mode, ColorMode::NeonNoir);
    }

    #[test]
    fn parse_color_blade_runner_kebab() {
        let cli = Cli::parse_from(["storageshower", "--color", "blade-runner"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.color_mode, ColorMode::BladeRunner);
    }

    #[test]
    fn parse_color_zaibatsu() {
        let cli = Cli::parse_from(["storageshower", "--color", "zaibatsu"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.color_mode, ColorMode::Zaibatsu);
    }

    #[test]
    fn no_reverse_long_clears_sort_rev() {
        let cli = Cli::parse_from(["storageshower", "--no-reverse"]);
        let mut prefs = Prefs::default();
        prefs.sort_rev = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.sort_rev);
    }

    #[test]
    fn short_bar_style_gradient() {
        let cli = Cli::parse_from(["storageshower", "-b", "gradient"]);
        let mut prefs = Prefs::default();
        prefs.bar_style = BarStyle::Ascii;
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.bar_style, BarStyle::Gradient);
    }

    #[test]
    fn parse_color_cyber_frost_kebab() {
        let cli = Cli::parse_from(["storageshower", "--color", "cyber-frost"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.color_mode, ColorMode::CyberFrost);
    }

    #[test]
    fn parse_color_plasma_core_kebab() {
        let cli = Cli::parse_from(["storageshower", "--color", "plasma-core"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.color_mode, ColorMode::PlasmaCore);
    }

    #[test]
    fn parse_color_night_city_kebab() {
        let cli = Cli::parse_from(["storageshower", "--color", "night-city"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.color_mode, ColorMode::NightCity);
    }

    #[test]
    fn parse_color_holo_shift_kebab() {
        let cli = Cli::parse_from(["storageshower", "--color", "holo-shift"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.color_mode, ColorMode::HoloShift);
    }

    #[test]
    fn parse_color_bio_hazard_kebab() {
        let cli = Cli::parse_from(["storageshower", "--color", "bio-hazard"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.color_mode, ColorMode::BioHazard);
    }

    #[test]
    fn apply_refresh_rate_zero() {
        let cli = Cli::parse_from(["storageshower", "-r", "0"]);
        let mut prefs = Prefs::default();
        prefs.refresh_rate = 9;
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.refresh_rate, 0);
    }

    #[test]
    fn parse_theme_name_with_hyphen() {
        let cli = Cli::parse_from(["storageshower", "--theme", "neon-pink"]);
        let mut prefs = Prefs::default();
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.active_theme.as_deref(), Some("neon-pink"));
    }

    #[test]
    fn parse_color_kebab_void_walker_quantum_flux_laser_grid() {
        let cases = [
            ("void-walker", ColorMode::VoidWalker),
            ("quantum-flux", ColorMode::QuantumFlux),
            ("laser-grid", ColorMode::LaserGrid),
            ("deep-net", ColorMode::DeepNet),
            ("steel-nerve", ColorMode::SteelNerve),
            ("dark-signal", ColorMode::DarkSignal),
            ("glitch-pop", ColorMode::GlitchPop),
            ("toxic-waste", ColorMode::ToxicWaste),
            ("chrome-heart", ColorMode::ChromeHeart),
            ("megacorp", ColorMode::Megacorp),
            ("overlock", ColorMode::Overlock),
            ("darkwave", ColorMode::Darkwave),
        ];
        for (flag, expected) in cases {
            let cli = Cli::parse_from(["storageshower", "--color", flag]);
            let mut prefs = Prefs::default();
            cli.apply_to(&mut prefs);
            assert_eq!(prefs.color_mode, expected, "flag={flag}");
        }
    }

    #[test]
    fn short_refresh_flag_overrides_prefs() {
        let cli = Cli::parse_from(["storageshower", "-r", "7"]);
        let mut prefs = Prefs::default();
        prefs.refresh_rate = 1;
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.refresh_rate, 7);
    }

    #[test]
    fn apply_col_pct_isolated_override() {
        let cli = Cli::parse_from(["storageshower", "--col-pct", "11"]);
        let mut prefs = Prefs::default();
        prefs.col_pct_w = 3;
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.col_pct_w, 11);
    }

    #[test]
    fn apply_col_mount_isolated_override() {
        let cli = Cli::parse_from(["storageshower", "--col-mount", "19"]);
        let mut prefs = Prefs::default();
        prefs.col_mount_w = 8;
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.col_mount_w, 19);
    }

    #[test]
    fn apply_col_bar_end_isolated_override() {
        let cli = Cli::parse_from(["storageshower", "--col-bar-end", "42"]);
        let mut prefs = Prefs::default();
        prefs.col_bar_end_w = 6;
        cli.apply_to(&mut prefs);
        assert_eq!(prefs.col_bar_end_w, 42);
    }

    #[test]
    fn no_bars_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--bars", "--no-bars"]);
        let mut prefs = Prefs::default();
        prefs.show_bars = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.show_bars);
    }

    #[test]
    fn virtual_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--no-virtual", "--virtual"]);
        let mut prefs = Prefs::default();
        prefs.show_all = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_all);
    }

    #[test]
    fn no_border_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--border", "--no-border"]);
        let mut prefs = Prefs::default();
        prefs.show_border = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.show_border);
    }

    #[test]
    fn header_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--no-header", "--header"]);
        let mut prefs = Prefs::default();
        prefs.show_header = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_header);
    }

    #[test]
    fn used_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--no-used", "--used"]);
        let mut prefs = Prefs::default();
        prefs.show_used = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_used);
    }

    #[test]
    fn compact_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--no-compact", "--compact"]);
        let mut prefs = Prefs::default();
        prefs.compact = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.compact);
    }

    #[test]
    fn full_mount_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--no-full-mount", "--full-mount"]);
        let mut prefs = Prefs::default();
        prefs.full_mount = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.full_mount);
    }

    #[test]
    fn tooltips_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--no-tooltips", "--tooltips"]);
        let mut prefs = Prefs::default();
        prefs.show_tooltips = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_tooltips);
    }

    #[test]
    fn no_reverse_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--reverse", "--no-reverse"]);
        let mut prefs = Prefs::default();
        prefs.sort_rev = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.sort_rev);
    }

    #[test]
    fn local_only_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--no-local", "--local-only"]);
        let mut prefs = Prefs::default();
        prefs.show_local = false;
        cli.apply_to(&mut prefs);
        assert!(prefs.show_local);
    }

    #[test]
    fn no_local_wins_when_last_in_argv() {
        let cli = Cli::parse_from(["storageshower", "--local-only", "--no-local"]);
        let mut prefs = Prefs::default();
        prefs.show_local = true;
        cli.apply_to(&mut prefs);
        assert!(!prefs.show_local);
    }

    #[test]
    fn list_colors_with_config_path_parse() {
        let cli = Cli::parse_from(["storageshower", "--list-colors", "--config", "/tmp/ss.conf"]);
        assert!(cli.list_colors);
        assert_eq!(cli.config.as_deref(), Some("/tmp/ss.conf"));
    }

    #[test]
    fn export_theme_with_color_flag_parse() {
        let cli = Cli::parse_from(["storageshower", "--export-theme", "--color", "purple"]);
        assert!(cli.export_theme);
        assert_eq!(cli.color_mode, Some(ColorMode::Purple));
    }

    #[test]
    fn version_flag_does_not_set_other_flags() {
        let cli = Cli::parse_from(["storageshower", "-V"]);
        assert!(cli.version);
        assert!(!cli.help);
        assert!(!cli.list_colors);
        assert!(!cli.export_theme);
    }

    /// Every `ColorMode` must round-trip through clap's `--color` spelling.
    #[test]
    fn every_color_mode_parses_via_clap_flag() {
        for &mode in ColorMode::ALL {
            let pv = mode
                .to_possible_value()
                .unwrap_or_else(|| panic!("no PossibleValue for {mode:?}"));
            let name = pv.get_name();
            let cli = Cli::try_parse_from(["storageshower", "--color", name])
                .unwrap_or_else(|e| panic!("parse --color {name:?} ({mode:?}): {e}"));
            assert_eq!(cli.color_mode, Some(mode), "flag {name}");
            let mut prefs = Prefs::default();
            prefs.color_mode = ColorMode::Green;
            cli.apply_to(&mut prefs);
            assert_eq!(prefs.color_mode, mode);
        }
    }

    /// BarStyle and SortMode ValueEnum names are accepted by the CLI.
    #[test]
    fn every_bar_style_and_sort_mode_parse_via_clap() {
        for &style in BarStyle::value_variants() {
            let pv = style.to_possible_value().expect("BarStyle possible value");
            let name = pv.get_name();
            let cli = Cli::try_parse_from(["storageshower", "-b", name]).unwrap_or_else(|e| {
                panic!("parse -b {name:?} ({style:?}): {e}");
            });
            assert_eq!(cli.bar_style, Some(style));
        }
        for &sort in SortMode::value_variants() {
            let pv = sort.to_possible_value().expect("SortMode possible value");
            let name = pv.get_name();
            let cli = Cli::try_parse_from(["storageshower", "-s", name])
                .unwrap_or_else(|e| panic!("parse -s {name:?} ({sort:?}): {e}"));
            assert_eq!(cli.sort_mode, Some(sort));
        }
        for &unit in UnitMode::value_variants() {
            let pv = unit.to_possible_value().expect("UnitMode possible value");
            let name = pv.get_name();
            let cli = Cli::try_parse_from(["storageshower", "-u", name])
                .unwrap_or_else(|e| panic!("parse -u {name:?} ({unit:?}): {e}"));
            assert_eq!(cli.unit_mode, Some(unit));
        }
    }

    fn strip_ansi(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c != '\x1b' {
                out.push(c);
                continue;
            }
            // CSI sequence: ESC '[' params/intermediates, then a final byte in
            // @..~. The '[' must be consumed first — it is itself in that range.
            if chars.peek() == Some(&'[') {
                chars.next();
                for c in chars.by_ref() {
                    if ('@'..='~').contains(&c) {
                        break;
                    }
                }
            }
        }
        out
    }

    /// Every palette `--list-colors` advertises must round-trip through
    /// `--color`. The table used to print the `Debug` spelling
    /// (`neonnoir`), which clap rejects — 20 of 30 listed flags were
    /// unusable copy-paste.
    #[test]
    fn every_listed_palette_flag_is_accepted_by_color() {
        let listing = strip_ansi(&colors_text());
        for &mode in ColorMode::ALL {
            let flag = mode.flag();
            assert!(
                listing.contains(&flag),
                "--list-colors never prints the flag {flag:?} for {mode:?}",
            );
            let cli = Cli::try_parse_from(["storageshower", "--color", &flag])
                .unwrap_or_else(|e| panic!("--color {flag:?} ({mode:?}) rejected: {e}"));
            assert_eq!(cli.color_mode, Some(mode), "--color {flag:?} parsed wrong");
        }
    }

    /// The usage hint under the palette table must name `--color`. It used to
    /// say `-c <flag>`, which is `--config` — that command silently kept the
    /// default palette and treated the palette name as a config path.
    #[test]
    fn list_colors_usage_hint_names_color_not_config() {
        let listing = strip_ansi(&colors_text());
        assert!(
            listing.contains("storageshower --color <flag>"),
            "usage hint missing --color: {listing}",
        );
        assert!(
            !listing.contains("storageshower -c <flag>"),
            "usage hint still points at -c, which is --config",
        );
        let stray = Cli::try_parse_from(["storageshower", "-c", "purple"])
            .expect("-c takes a path, so this parses");
        assert!(
            stray.color_mode.is_none(),
            "-c must not set the palette; it sets the config path",
        );
    }

    /// `--help` is hand-written, so it can silently omit a flag that was added
    /// to the parser. Every user-visible long flag must appear in it.
    #[test]
    fn help_text_documents_every_visible_long_flag() {
        use clap::CommandFactory;
        let help = strip_ansi(&help_text());
        let cmd = Cli::command();
        let mut missing = Vec::new();
        for arg in cmd.get_arguments() {
            if arg.is_hide_set() {
                continue;
            }
            if let Some(long) = arg.get_long()
                && !help.contains(&format!("--{long}"))
            {
                missing.push(format!("--{long}"));
            }
        }
        assert!(missing.is_empty(), "--help omits: {missing:?}");
    }
}
