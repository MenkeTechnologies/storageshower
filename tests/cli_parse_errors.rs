//! `Cli::try_parse_from` should reject invalid enum values and unknown flags.

use clap::Parser;
use storageshower::cli::Cli;

fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    let mut v = vec!["storageshower"];
    v.extend_from_slice(args);
    Cli::try_parse_from(v)
}

#[test]
fn invalid_sort_mode() {
    assert!(parse(&["--sort", "nope"]).is_err());
}

#[test]
fn invalid_bar_style() {
    assert!(parse(&["--bar-style", "holographic"]).is_err());
}

#[test]
fn invalid_color_mode() {
    assert!(parse(&["--color", "not-a-palette"]).is_err());
}

#[test]
fn invalid_units() {
    assert!(parse(&["--units", "tb"]).is_err());
}

#[test]
fn invalid_warn_not_a_number() {
    assert!(parse(&["--warn", "lots"]).is_err());
}

#[test]
fn invalid_crit_not_a_number() {
    assert!(parse(&["--crit", "xyz"]).is_err());
}

#[test]
fn invalid_refresh_not_a_number() {
    assert!(parse(&["--refresh", "soon"]).is_err());
}

#[test]
fn invalid_col_mount_not_a_number() {
    assert!(parse(&["--col-mount", "wide"]).is_err());
}

#[test]
fn unknown_long_flag() {
    assert!(parse(&["--this-flag-does-not-exist"]).is_err());
}

#[test]
fn bare_positional_rejected() {
    assert!(parse(&["/mnt"]).is_err());
}

#[test]
fn empty_sort_value_rejected() {
    assert!(parse(&["--sort"]).is_err());
}

#[test]
fn valid_minimal_still_parses() {
    assert!(parse(&[]).is_ok());
}
