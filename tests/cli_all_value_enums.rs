//! Ensure every `clap::ValueEnum` variant round-trips through `Cli` flag strings.
//! Catches renamed flags (e.g. `neon-noir`) drifting from `ColorMode`.

use clap::Parser;
use clap::ValueEnum;
use storageshower::cli::Cli;
use storageshower::types::{BarStyle, ColorMode, SortMode, UnitMode};

fn parse(args: &[&str]) -> Cli {
    let mut v = vec!["storageshower"];
    v.extend_from_slice(args);
    Cli::try_parse_from(v).unwrap_or_else(|e| panic!("parse {args:?}: {e}"))
}

#[test]
fn all_sort_modes_parse() {
    for &mode in SortMode::value_variants() {
        let pv = mode
            .to_possible_value()
            .expect("sort variant has clap name");
        let name = pv.get_name();
        let c = parse(&["--sort", name]);
        assert_eq!(c.sort_mode, Some(mode), "--sort {name}");
    }
}

#[test]
fn all_bar_styles_parse() {
    for &style in BarStyle::value_variants() {
        let pv = style
            .to_possible_value()
            .expect("bar style variant has clap name");
        let name = pv.get_name();
        let c = parse(&["--bar-style", name]);
        assert_eq!(c.bar_style, Some(style), "--bar-style {name}");
    }
}

#[test]
fn all_unit_modes_parse() {
    for &mode in UnitMode::value_variants() {
        let pv = mode
            .to_possible_value()
            .expect("unit variant has clap name");
        let name = pv.get_name();
        let c = parse(&["--units", name]);
        assert_eq!(c.unit_mode, Some(mode), "--units {name}");
    }
}

#[test]
fn all_color_modes_parse() {
    for &mode in ColorMode::value_variants() {
        let pv = mode
            .to_possible_value()
            .expect("color variant has clap name");
        let name = pv.get_name();
        let c = parse(&["--color", name]);
        assert_eq!(c.color_mode, Some(mode), "--color {name}");
    }
}

#[test]
fn color_mode_all_matches_value_variants_len() {
    assert_eq!(
        ColorMode::ALL.len(),
        ColorMode::value_variants().len(),
        "ColorMode::ALL should list every ValueEnum variant"
    );
}
