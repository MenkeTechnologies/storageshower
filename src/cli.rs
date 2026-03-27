use clap::Parser;

use crate::prefs::Prefs;
use crate::types::*;

/// Cyberpunk disk usage TUI
#[derive(Parser, Debug)]
#[command(
    name = "storageshower",
    version,
    disable_help_flag = true,
    disable_version_flag = true,
)]
pub struct Cli {
    /// Sort mode for disk entries
    #[arg(short = 's', long = "sort", value_name = "MODE")]
    pub sort_mode: Option<CliSortMode>,

    /// Reverse sort order
    #[arg(short = 'R', long = "reverse")]
    pub sort_rev: bool,

    /// Show only local disks (HDD/SSD)
    #[arg(short = 'l', long = "local-only")]
    pub show_local: bool,

    /// Data refresh interval in seconds
    #[arg(short = 'r', long = "refresh", value_name = "SECS")]
    pub refresh_rate: Option<u64>,

    /// Bar visualization style
    #[arg(short = 'b', long = "bar-style", value_name = "STYLE")]
    pub bar_style: Option<CliBarStyle>,

    /// Color palette
    #[arg(short = 'c', long = "color", value_name = "PALETTE")]
    pub color_mode: Option<CliColorMode>,

    /// Warning threshold percentage
    #[arg(short = 'w', long = "warn", value_name = "PCT")]
    pub thresh_warn: Option<u8>,

    /// Critical threshold percentage
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
    pub compact: bool,

    /// Hide used/total size display
    #[arg(long = "no-used")]
    pub no_used: bool,

    /// Show full mount paths
    #[arg(short = 'f', long = "full-mount")]
    pub full_mount: bool,

    /// Hide virtual filesystems
    #[arg(long = "no-virtual")]
    pub no_virtual: bool,

    /// Unit display mode
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

    /// Config file path
    #[arg(long = "config", value_name = "PATH")]
    pub config: Option<String>,

    /// Display this transmission
    #[arg(short = 'h', long = "help")]
    pub help: bool,

    /// Display version information
    #[arg(short = 'V', long = "version")]
    pub version: bool,
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

// ANSI color helpers
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
{B_GREEN}   -c, --color PALETTE   {RST}color palette {B_MAGENTA}(default, green, blue, purple){RST}
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
{B_GREEN}       --config PATH     {RST}config file path {B_MAGENTA}(default: ~/.storageshower.conf){RST}
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
{B_GREEN}   storageshower -c purple -b ascii   {RST}purple palette with ascii bars
{B_GREEN}   storageshower -s pct -R            {RST}sort by usage%, reversed
{B_GREEN}   storageshower -l --no-virtual      {RST}local physical disks only
{B_GREEN}   storageshower -u gib -w 60 -C 85  {RST}GiB units, custom thresholds
{B_GREEN}   storageshower --config /tmp/ss.conf{RST} use alternate config

{B_CYAN}  ── INFO ──────────────────────────────────────────{RST}
{B_MAGENTA}  v{ver} {RST}// {B_YELLOW}cyberpunk disk usage TUI{RST}
  Config synced to: ~/.storageshower.conf
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
        if self.sort_rev {
            prefs.sort_rev = true;
        }
        if self.show_local {
            prefs.show_local = true;
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
        if self.compact {
            prefs.compact = true;
        }
        if self.no_used {
            prefs.show_used = false;
        }
        if self.full_mount {
            prefs.full_mount = true;
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
