//! `Cli::apply_to` for `-w`, `-C`, and `--no-bars`.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;

#[test]
fn warn_crit_no_bars() {
    let cli = Cli::parse_from(["storageshower", "-w", "52", "-C", "89", "--no-bars"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.thresh_warn, 52);
    assert_eq!(p.thresh_crit, 89);
    assert!(!p.show_bars);
}

#[test]
fn no_bars_only_preserves_warn_crit() {
    let cli = Cli::parse_from(["storageshower", "--no-bars"]);
    let mut p = Prefs {
        thresh_warn: 63,
        thresh_crit: 91,
        show_bars: true,
        ..Default::default()
    };
    cli.apply_to(&mut p);
    assert!(!p.show_bars);
    assert_eq!(p.thresh_warn, 63);
    assert_eq!(p.thresh_crit, 91);
}
