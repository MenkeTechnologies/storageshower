//! `Cli::apply_to` for hyphenated `ColorMode` CLI names.

use clap::Parser;

use storageshower::cli::Cli;
use storageshower::prefs::Prefs;
use storageshower::types::ColorMode;

#[test]
fn neon_noir() {
    let cli = Cli::parse_from(["storageshower", "--color", "neon-noir"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::NeonNoir);
}

#[test]
fn blade_runner() {
    let cli = Cli::parse_from(["storageshower", "--color", "blade-runner"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::BladeRunner);
}

#[test]
fn laser_grid() {
    let cli = Cli::parse_from(["storageshower", "--color", "laser-grid"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::LaserGrid);
}

#[test]
fn quantum_flux() {
    let cli = Cli::parse_from(["storageshower", "--color", "quantum-flux"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::QuantumFlux);
}

#[test]
fn deep_net() {
    let cli = Cli::parse_from(["storageshower", "--color", "deep-net"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::DeepNet);
}

#[test]
fn holo_shift() {
    let cli = Cli::parse_from(["storageshower", "--color", "holo-shift"]);
    let mut p = Prefs::default();
    cli.apply_to(&mut p);
    assert_eq!(p.color_mode, ColorMode::HoloShift);
}
