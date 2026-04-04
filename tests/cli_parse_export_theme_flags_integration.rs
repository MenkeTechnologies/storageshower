//! `Cli` parses `--export-theme` with other flags (no `apply_to` side effects).

use clap::Parser;

use storageshower::cli::Cli;

#[test]
fn export_theme_with_list_colors() {
    let cli = Cli::parse_from(["storageshower", "--export-theme", "--list-colors"]);
    assert!(cli.export_theme);
    assert!(cli.list_colors);
}

#[test]
fn export_theme_sort_size() {
    let cli = Cli::parse_from(["storageshower", "--export-theme", "--sort", "size", "-V"]);
    assert!(cli.export_theme);
    assert!(cli.version);
}
