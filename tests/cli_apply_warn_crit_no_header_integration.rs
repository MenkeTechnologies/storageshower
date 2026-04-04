//! `Cli::apply_to` for `-w`, `-C`, and `--no-header`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_no_header() {
    let cli = Cli::parse_from(["storageshower", "-w", "51", "-C", "88", "--no-header"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 51);
    assert_eq!(p.thresh_crit, 88);
    assert!(!p.show_header);
}

#[test]
fn no_header_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--no-header"]);
    let mut p = Prefs {
        thresh_warn: 59,
        thresh_crit: 93,
        show_header: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_header);
    assert_eq!(p.thresh_warn, 59);
    assert_eq!(p.thresh_crit, 93);
}
