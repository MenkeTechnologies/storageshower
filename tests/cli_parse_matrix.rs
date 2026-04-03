//! `Cli::try_parse_from` coverage: fast, no subprocess, complements `cli_binary.rs`.
//!
//! Help/version use custom clap flags (`disable_help_flag`), so successful parse sets
//! `cli.help` / `cli.version` instead of `ErrorKind::DisplayHelp`.

use clap::Parser;
use storageshower::cli::Cli;
use storageshower::types::{BarStyle, ColorMode, SortMode, UnitMode};

fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    let mut v = vec!["storageshower"];
    v.extend_from_slice(args);
    Cli::try_parse_from(v)
}

#[test]
fn minimal_invocation() {
    let c = parse(&[]).unwrap();
    assert!(!c.help);
    assert!(!c.version);
}

#[test]
fn help_long_sets_flag() {
    let c = parse(&["--help"]).unwrap();
    assert!(c.help);
    assert!(!c.version);
}

#[test]
fn help_short_sets_flag() {
    let c = parse(&["-h"]).unwrap();
    assert!(c.help);
}

#[test]
fn version_long_sets_flag() {
    let c = parse(&["--version"]).unwrap();
    assert!(c.version);
    assert!(!c.help);
}

#[test]
fn version_short_sets_flag() {
    let c = parse(&["-V"]).unwrap();
    assert!(c.version);
}

#[test]
fn sort_name() {
    let c = parse(&["--sort", "name"]).unwrap();
    assert_eq!(c.sort_mode, Some(SortMode::Name));
}

#[test]
fn sort_pct_and_reverse() {
    let c = parse(&["--sort", "pct", "--reverse"]).unwrap();
    assert_eq!(c.sort_mode, Some(SortMode::Pct));
    assert!(c.sort_rev);
}

#[test]
fn no_reverse_overrides_reverse() {
    let c = parse(&["-R", "--no-reverse"]).unwrap();
    assert!(!c.sort_rev);
}

#[test]
fn bar_style_solid() {
    let c = parse(&["--bar-style", "solid"]).unwrap();
    assert_eq!(c.bar_style, Some(BarStyle::Solid));
}

#[test]
fn color_mode_matrix() {
    let c = parse(&["--color", "matrix"]).unwrap();
    assert_eq!(c.color_mode, Some(ColorMode::Matrix));
}

#[test]
fn units_mib() {
    let c = parse(&["--units", "mib"]).unwrap();
    assert_eq!(c.unit_mode, Some(UnitMode::MiB));
}

#[test]
fn units_gib() {
    let c = parse(&["--units", "gib"]).unwrap();
    assert_eq!(c.unit_mode, Some(UnitMode::GiB));
}

#[test]
fn warn_and_crit() {
    let c = parse(&["--warn", "80", "--crit", "95"]).unwrap();
    assert_eq!(c.thresh_warn, Some(80));
    assert_eq!(c.thresh_crit, Some(95));
}

#[test]
fn refresh_rate() {
    let c = parse(&["--refresh", "10"]).unwrap();
    assert_eq!(c.refresh_rate, Some(10));
}

#[test]
fn local_only_short() {
    let c = parse(&["-l"]).unwrap();
    assert!(c.show_local);
}

#[test]
fn compact_short() {
    let c = parse(&["-k"]).unwrap();
    assert!(c.compact);
}

#[test]
fn no_bars_long() {
    let c = parse(&["--no-bars"]).unwrap();
    assert!(c.no_bars);
}

#[test]
fn no_header() {
    let c = parse(&["--no-header"]).unwrap();
    assert!(c.no_header);
}

#[test]
fn no_border() {
    let c = parse(&["--no-border"]).unwrap();
    assert!(c.no_border);
}

#[test]
fn no_used() {
    let c = parse(&["--no-used"]).unwrap();
    assert!(c.no_used);
}

#[test]
fn full_mount_short() {
    let c = parse(&["-f"]).unwrap();
    assert!(c.full_mount);
}

#[test]
fn no_full_mount() {
    let c = parse(&["--no-full-mount"]).unwrap();
    assert!(c.no_full_mount);
}

#[test]
fn no_tooltips() {
    let c = parse(&["--no-tooltips"]).unwrap();
    assert!(c.no_tooltips);
}

#[test]
fn show_virtual() {
    let c = parse(&["--virtual"]).unwrap();
    assert!(c.show_virtual);
}

#[test]
fn no_virtual() {
    let c = parse(&["--no-virtual"]).unwrap();
    assert!(c.no_virtual);
}

#[test]
fn col_widths() {
    let c = parse(&["--col-mount", "24", "--col-bar-end", "30", "--col-pct", "8"]).unwrap();
    assert_eq!(c.col_mount_w, Some(24));
    assert_eq!(c.col_bar_end_w, Some(30));
    assert_eq!(c.col_pct_w, Some(8));
}

#[test]
fn config_path() {
    let c = parse(&["--config", "/tmp/x.conf"]).unwrap();
    assert_eq!(c.config.as_deref(), Some("/tmp/x.conf"));
}

#[test]
fn list_colors() {
    let c = parse(&["--list-colors"]).unwrap();
    assert!(c.list_colors);
}

#[test]
fn export_theme() {
    let c = parse(&["--export-theme"]).unwrap();
    assert!(c.export_theme);
}

#[test]
fn theme_name() {
    let c = parse(&["--theme", "mytheme"]).unwrap();
    assert_eq!(c.theme.as_deref(), Some("mytheme"));
}

#[test]
fn sort_size_with_units_bytes() {
    let c = parse(&["--sort", "size", "--units", "bytes"]).unwrap();
    assert_eq!(c.sort_mode, Some(SortMode::Size));
    assert_eq!(c.unit_mode, Some(UnitMode::Bytes));
}

#[test]
fn neon_noir_color_hyphenated() {
    let c = parse(&["--color", "neon-noir"]).unwrap();
    assert_eq!(c.color_mode, Some(ColorMode::NeonNoir));
}

#[test]
fn help_with_leading_sort_still_help() {
    let c = parse(&["--sort", "name", "--help"]).unwrap();
    assert!(c.help);
    assert_eq!(c.sort_mode, Some(SortMode::Name));
}

#[test]
fn version_with_leading_units_still_version() {
    let c = parse(&["--units", "gib", "-V"]).unwrap();
    assert!(c.version);
    assert_eq!(c.unit_mode, Some(UnitMode::GiB));
}

#[test]
fn unknown_flag_errors() {
    assert!(parse(&["--totally-unknown-flag"]).is_err());
}
