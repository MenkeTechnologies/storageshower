//! `Cli::apply_to` for `--sort size` with `--col-pct`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::SortMode;

#[test]
fn sort_size_col_pct() {
    let cli = Cli::parse_from(["storageshower", "--sort", "size", "--col-pct", "16"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.sort_mode, SortMode::Size);
    assert_eq!(p.col_pct_w, 16);
}

#[test]
fn short_sort_size_col_pct_preserves_other_widths() {
    let cli = Cli::parse_from(["storageshower", "-s", "size", "--col-pct", "5"]);
    let mut p = Prefs {
        col_mount_w: 22,
        col_bar_end_w: 33,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert_eq!(p.col_pct_w, 5);
    assert_eq!(p.col_mount_w, 22);
    assert_eq!(p.col_bar_end_w, 33);
}
