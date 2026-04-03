//! Multi-argument `Cli::try_parse_from` cases (typical user invocations).

use clap::Parser;
use storageshower::cli::Cli;
use storageshower::types::{BarStyle, ColorMode, SortMode, UnitMode};

fn p(args: &[&str]) -> Cli {
    let mut v = vec!["storageshower"];
    v.extend_from_slice(args);
    Cli::try_parse_from(v).unwrap_or_else(|e| panic!("parse {:?}: {e}", args))
}

#[test]
fn combo_local_compact_full_mount() {
    let c = p(&["-l", "-k", "-f"]);
    assert!(c.show_local);
    assert!(c.compact);
    assert!(c.full_mount);
}

#[test]
fn combo_chrome_minimal() {
    let c = p(&["--no-bars", "--no-header", "--no-border", "--no-used"]);
    assert!(c.no_bars);
    assert!(c.no_header);
    assert!(c.no_border);
    assert!(c.no_used);
}

#[test]
fn combo_sort_size_refresh_color() {
    let c = p(&["--sort", "size", "--refresh", "5", "--color", "cyan"]);
    assert_eq!(c.sort_mode, Some(SortMode::Size));
    assert_eq!(c.refresh_rate, Some(5));
    assert_eq!(c.color_mode, Some(ColorMode::Cyan));
}

#[test]
fn combo_sort_pct_reverse_bar_thin() {
    let c = p(&["--sort", "pct", "--reverse", "--bar-style", "thin"]);
    assert_eq!(c.sort_mode, Some(SortMode::Pct));
    assert!(c.sort_rev);
    assert_eq!(c.bar_style, Some(BarStyle::Thin));
}

#[test]
fn combo_warn_crit_refresh() {
    let c = p(&["--warn", "60", "--crit", "85", "--refresh", "30"]);
    assert_eq!(c.thresh_warn, Some(60));
    assert_eq!(c.thresh_crit, Some(85));
    assert_eq!(c.refresh_rate, Some(30));
}

#[test]
fn combo_short_sort_units_refresh() {
    let c = p(&["-s", "name", "-u", "mib", "-r", "2"]);
    assert_eq!(c.sort_mode, Some(SortMode::Name));
    assert_eq!(c.unit_mode, Some(UnitMode::MiB));
    assert_eq!(c.refresh_rate, Some(2));
}

#[test]
fn combo_virtual_no_tooltips() {
    let c = p(&["--virtual", "--no-tooltips"]);
    assert!(c.show_virtual);
    assert!(c.no_tooltips);
}

#[test]
fn combo_no_virtual_with_local() {
    let c = p(&["--no-virtual", "-l"]);
    assert!(c.no_virtual);
    assert!(c.show_local);
}

#[test]
fn combo_config_theme_export() {
    let c = p(&[
        "--config",
        "/tmp/cfg.toml",
        "--theme",
        "slot-a",
        "--export-theme",
    ]);
    assert_eq!(c.config.as_deref(), Some("/tmp/cfg.toml"));
    assert_eq!(c.theme.as_deref(), Some("slot-a"));
    assert!(c.export_theme);
}

#[test]
fn combo_list_colors_with_config_ignored_by_parse() {
    let c = p(&["--config", "/dev/null", "--list-colors"]);
    assert!(c.list_colors);
}

#[test]
fn combo_col_widths_with_sort() {
    let c = p(&[
        "--sort",
        "pct",
        "--col-mount",
        "20",
        "--col-bar-end",
        "28",
        "--col-pct",
        "7",
    ]);
    assert_eq!(c.sort_mode, Some(SortMode::Pct));
    assert_eq!(c.col_mount_w, Some(20));
    assert_eq!(c.col_bar_end_w, Some(28));
    assert_eq!(c.col_pct_w, Some(7));
}

#[test]
fn combo_zaibatsu_gib() {
    let c = p(&["--color", "zaibatsu", "--units", "gib"]);
    assert_eq!(c.color_mode, Some(ColorMode::Zaibatsu));
    assert_eq!(c.unit_mode, Some(UnitMode::GiB));
}

#[test]
fn combo_blade_runner_ascii() {
    let c = p(&["--color", "blade-runner", "--bar-style", "ascii"]);
    assert_eq!(c.color_mode, Some(ColorMode::BladeRunner));
    assert_eq!(c.bar_style, Some(BarStyle::Ascii));
}

#[test]
fn combo_megacorp_solid() {
    let c = p(&["--color", "megacorp", "--bar-style", "solid"]);
    assert_eq!(c.color_mode, Some(ColorMode::Megacorp));
    assert_eq!(c.bar_style, Some(BarStyle::Solid));
}

#[test]
fn combo_no_compact_no_full_mount() {
    let c = p(&["--no-compact", "--no-full-mount"]);
    assert!(c.no_compact);
    assert!(c.no_full_mount);
}

#[test]
fn combo_border_and_header_explicit() {
    let c = p(&["--border", "--header"]);
    assert!(c.border);
    assert!(c.header);
}

#[test]
fn combo_bars_and_used_explicit() {
    let c = p(&["--bars", "--used"]);
    assert!(c.bars);
    assert!(c.used);
}

#[test]
fn combo_tooltips_on() {
    let c = p(&["--tooltips"]);
    assert!(c.tooltips);
}

#[test]
fn combo_reverse_no_reverse_is_false() {
    let c = p(&["--reverse", "--no-reverse", "--sort", "size"]);
    assert!(!c.sort_rev);
    assert_eq!(c.sort_mode, Some(SortMode::Size));
}

#[test]
fn combo_local_no_local() {
    let c = p(&["--local-only", "--no-local"]);
    assert!(!c.show_local);
}

#[test]
fn combo_bytes_units_blade_runner() {
    let c = p(&["--units", "bytes", "--color", "blade-runner"]);
    assert_eq!(c.unit_mode, Some(UnitMode::Bytes));
    assert_eq!(c.color_mode, Some(ColorMode::BladeRunner));
}

#[test]
fn combo_refresh_1_warn_70_crit_90() {
    let c = p(&["--refresh", "1", "--warn", "70", "--crit", "90"]);
    assert_eq!(c.refresh_rate, Some(1));
    assert_eq!(c.thresh_warn, Some(70));
    assert_eq!(c.thresh_crit, Some(90));
}

#[test]
fn combo_sort_name_short_long_equivalent_values() {
    let a = p(&["-s", "name"]);
    let b = p(&["--sort", "name"]);
    assert_eq!(a.sort_mode, b.sort_mode);
}

#[test]
fn combo_three_width_flags_only() {
    let c = p(&["--col-pct", "12", "--col-mount", "40", "--col-bar-end", "0"]);
    assert_eq!(c.col_pct_w, Some(12));
    assert_eq!(c.col_mount_w, Some(40));
    assert_eq!(c.col_bar_end_w, Some(0));
}

#[test]
fn combo_holo_shift_quantum_flux_names() {
    let c = p(&["--color", "holo-shift"]);
    assert_eq!(c.color_mode, Some(ColorMode::HoloShift));
    let d = p(&["--color", "quantum-flux"]);
    assert_eq!(d.color_mode, Some(ColorMode::QuantumFlux));
}

#[test]
fn combo_night_city_deep_net() {
    let c = p(&["--color", "night-city", "--sort", "pct"]);
    assert_eq!(c.color_mode, Some(ColorMode::NightCity));
    assert_eq!(c.sort_mode, Some(SortMode::Pct));
}

#[test]
fn combo_laser_grid_toxic_waste() {
    let c = p(&["--color", "laser-grid"]);
    assert_eq!(c.color_mode, Some(ColorMode::LaserGrid));
    let d = p(&["--color", "toxic-waste"]);
    assert_eq!(d.color_mode, Some(ColorMode::ToxicWaste));
}

#[test]
fn combo_darkwave_overlock() {
    let c = p(&["--color", "darkwave", "-R"]);
    assert_eq!(c.color_mode, Some(ColorMode::Darkwave));
    assert!(c.sort_rev);
    let d = p(&["--color", "overlock"]);
    assert_eq!(d.color_mode, Some(ColorMode::Overlock));
}

#[test]
fn combo_matrix_sunset() {
    let c = p(&["--color", "matrix", "--bar-style", "gradient"]);
    assert_eq!(c.color_mode, Some(ColorMode::Matrix));
    assert_eq!(c.bar_style, Some(BarStyle::Gradient));
}

#[test]
fn combo_sakura_red() {
    let c = p(&["--color", "sakura", "--warn", "55"]);
    assert_eq!(c.color_mode, Some(ColorMode::Sakura));
    assert_eq!(c.thresh_warn, Some(55));
}

#[test]
fn combo_config_with_many_flags() {
    let c = p(&[
        "-c",
        "/home/user/.storageshower.conf",
        "-s",
        "size",
        "-u",
        "human",
        "--no-bars",
    ]);
    assert_eq!(c.config.as_deref(), Some("/home/user/.storageshower.conf"));
    assert_eq!(c.sort_mode, Some(SortMode::Size));
    assert_eq!(c.unit_mode, Some(UnitMode::Human));
    assert!(c.no_bars);
}

#[test]
fn combo_theme_only() {
    let c = p(&["--theme", ""]);
    assert_eq!(c.theme.as_deref(), Some(""));
}
