//! Parse-only coverage for `--list-colors`, `--export-theme`, `--theme`, and `--help`/`--version` combinations.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::types::ColorMode;

fn parse(args: &[&str]) -> Cli {
    let mut v = vec!["storageshower"];
    v.extend_from_slice(args);
    Cli::try_parse_from(v).unwrap_or_else(|e| panic!("parse {:?}: {e}", args))
}

#[test]
fn list_colors_flag_sets_true() {
    let c = parse(&["--list-colors"]);
    assert!(c.list_colors);
}

#[test]
fn export_theme_flag_sets_true() {
    let c = parse(&["--export-theme"]);
    assert!(c.export_theme);
}

#[test]
fn theme_name_parsed() {
    let c = parse(&["--theme", "my-custom-slot"]);
    assert_eq!(c.theme.as_deref(), Some("my-custom-slot"));
}

#[test]
fn export_theme_with_theme_name() {
    let c = parse(&["--export-theme", "--theme", "saved"]);
    assert!(c.export_theme);
    assert_eq!(c.theme.as_deref(), Some("saved"));
}

#[test]
fn list_colors_with_config_path() {
    let c = parse(&["--list-colors", "--config", "/tmp/ss.conf"]);
    assert!(c.list_colors);
    assert_eq!(c.config.as_deref(), Some("/tmp/ss.conf"));
}

#[test]
fn help_short_stops_parse() {
    let c = parse(&["-h"]);
    assert!(c.help);
}

#[test]
fn help_long_stops_parse() {
    let c = parse(&["--help"]);
    assert!(c.help);
}

#[test]
fn version_short() {
    let c = parse(&["-V"]);
    assert!(c.version);
}

#[test]
fn version_long() {
    let c = parse(&["--version"]);
    assert!(c.version);
}

#[test]
fn export_theme_with_color_palette() {
    let c = parse(&["--export-theme", "--color", "zaibatsu"]);
    assert!(c.export_theme);
    assert_eq!(c.color_mode, Some(ColorMode::Zaibatsu));
}

#[test]
fn list_colors_with_sort_ignored_for_parse() {
    let c = parse(&["--list-colors", "--sort", "size"]);
    assert!(c.list_colors);
    assert_eq!(c.sort_mode, Some(storageshower::types::SortMode::Size));
}

#[test]
fn theme_with_color_and_config() {
    let c = parse(&[
        "--theme",
        "neon",
        "--color",
        "cyan",
        "-c",
        "/path/conf.toml",
    ]);
    assert_eq!(c.theme.as_deref(), Some("neon"));
    assert_eq!(c.color_mode, Some(ColorMode::Cyan));
    assert_eq!(c.config.as_deref(), Some("/path/conf.toml"));
}

#[test]
fn export_theme_false_when_absent() {
    let c = parse(&["--sort", "name"]);
    assert!(!c.export_theme);
}

#[test]
fn list_colors_false_when_absent() {
    let c = parse(&["--refresh", "3"]);
    assert!(!c.list_colors);
}
