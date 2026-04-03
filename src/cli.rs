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
    println!(
        r#"
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

{B_CYAN}  >> NETRUNNER DISK MONITOR v{ver} << {RST}
{B_MAGENTA}  [ jack in to your chrome and monitor the datastream ]{RST}

{B_YELLOW}  USAGE:{RST} storageshower [OPTIONS]

{B_CYAN}  ── SORTING ───────────────────────────────────────{RST}
{B_GREEN}   -s, --sort MODE       {RST}sort disk entries {B_MAGENTA}(name, pct, size){RST}
{B_GREEN}   -R, --reverse         {RST}reverse sort order
{B_GREEN}   -l, --local-only      {RST}show only local disks {B_MAGENTA}(HDD/SSD){RST}
{B_GREEN}       --no-virtual      {RST}hide virtual filesystems {B_MAGENTA}(tmpfs, devfs, etc.){RST}

{B_CYAN}  ── DISPLAY ───────────────────────────────────────{RST}
{B_GREEN}   -b, --bar-style STYLE {RST}bar visualization {B_MAGENTA}(gradient, solid, thin, ascii){RST}
{B_GREEN}       --color PALETTE   {RST}color palette {B_MAGENTA}(default, green, blue, purple, ...){RST}
{B_GREEN}       --list-colors      {RST}list all builtin color schemes
{B_GREEN}       --export-theme     {RST}export current palette as TOML
{B_GREEN}       --theme NAME       {RST}activate a custom theme by name
{B_GREEN}   -u, --units MODE      {RST}unit display {B_MAGENTA}(human, gib, mib, bytes){RST}
{B_GREEN}   -k, --compact         {RST}compact mount names
{B_GREEN}   -f, --full-mount      {RST}show full mount paths
{B_GREEN}       --no-bars         {RST}hide usage bars
{B_GREEN}       --no-border       {RST}hide border chrome
{B_GREEN}       --no-header       {RST}hide column headers
{B_GREEN}       --no-used         {RST}hide used/total size display

{B_CYAN}  ── THRESHOLDS ────────────────────────────────────{RST}
{B_GREEN}   -w, --warn PCT        {RST}warning threshold {B_MAGENTA}(default: 70%){RST}
{B_GREEN}   -C, --crit PCT        {RST}critical threshold {B_MAGENTA}(default: 90%){RST}

{B_CYAN}  ── COLUMNS ───────────────────────────────────────{RST}
{B_GREEN}       --col-mount WIDTH  {RST}mount column width {B_MAGENTA}(0 = auto){RST}
{B_GREEN}       --col-bar-end WIDTH{RST} bar-end column width {B_MAGENTA}(0 = auto){RST}
{B_GREEN}       --col-pct WIDTH    {RST}percentage column width {B_MAGENTA}(0 = auto){RST}

{B_CYAN}  ── SYSTEM ────────────────────────────────────────{RST}
{B_GREEN}   -r, --refresh SECS    {RST}data refresh interval {B_MAGENTA}(default: 1s){RST}
{B_GREEN}   -c, --config PATH     {RST}config file path {B_MAGENTA}(default: ~/.storageshower.conf){RST}
{B_GREEN}   -h, --help            {RST}display this transmission
{B_GREEN}   -V, --version         {RST}display version information

{B_CYAN}  ── KEYBINDS ──────────────────────────────────────{RST}
{B_GREEN}   q, Esc                {RST}flatline {B_MAGENTA}(quit){RST}
{B_GREEN}   j / k                 {RST}scroll the datastream
{B_GREEN}   s                     {RST}cycle sort ICE
{B_GREEN}   r                     {RST}reverse sort polarity
{B_GREEN}   b                     {RST}swap bar firmware
{B_GREEN}   c                     {RST}shift chroma palette
{B_GREEN}   u                     {RST}toggle used/total
{B_GREEN}   a                     {RST}toggle all/local netlinks
{B_GREEN}   m                     {RST}toggle full mount path
{B_GREEN}   /                     {RST}enter filter daemon
{B_GREEN}   p                     {RST}pause data feed
{B_GREEN}   ?                     {RST}open help overlay

{B_CYAN}  ── EXAMPLES ──────────────────────────────────────{RST}
{B_GREEN}   storageshower --color purple -b ascii{RST}purple palette with ascii bars
{B_GREEN}   storageshower -s pct -R            {RST}sort by usage%, reversed
{B_GREEN}   storageshower -l --no-virtual      {RST}local physical disks only
{B_GREEN}   storageshower -u gib -w 60 -C 85  {RST}GiB units, custom thresholds
{B_GREEN}   storageshower --config /tmp/ss.conf{RST} use alternate config

{B_CYAN}  ── INFO ──────────────────────────────────────────{RST}
{B_MAGENTA}  v{ver} {RST}// {B_YELLOW}cyberpunk disk usage TUI{RST}
  Config synced to: ~/.storageshower.conf
  CLI flags override config file. Every --flag has a --no-flag inverse.
{B_MAGENTA}  Wake up, samurai. We have disks to monitor.{RST}
"#,
        ver = env!("CARGO_PKG_VERSION"),
    );
}

pub fn print_version() {
    println!(
        "{B_CYAN}storageshower{RST} {B_MAGENTA}v{ver}{RST}",
        ver = env!("CARGO_PKG_VERSION"),
    );
}

pub fn print_colors() {
    use crate::ui::palette;
    use ratatui::style::Color;

    fn idx(c: Color) -> u8 {
        match c {
            Color::Indexed(n) => n,
            _ => 0,
        }
    }

    println!("\n{B_CYAN}  ── BUILTIN COLOR SCHEMES ────────────────────────{RST}\n");
    for &mode in ColorMode::ALL {
        let (a, b, c, d, e, f) = palette(mode);
        let swatch: String = [a, b, c, d, e, f]
            .iter()
            .map(|&col| format!("\x1b[48;5;{}m   {RST}", idx(col)))
            .collect();
        println!(
            "  {B_GREEN}{flag:<10}{RST} {B_MAGENTA}{name:<14}{RST} {swatch}",
            flag = format!("{:?}", mode).to_lowercase(),
            name = mode.name(),
        );
    }
    println!("\n  {B_YELLOW}Usage:{RST} storageshower {B_GREEN}-c{RST} {B_MAGENTA}<flag>{RST}");
    println!("  {B_YELLOW}Cycle:{RST} press {B_GREEN}c{RST} in the TUI\n");
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
    use clap::Parser;

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
}
