//! `Cli` parses `--config` path without loading the file.

use clap::Parser;

use storageshower::cli::Cli;

#[test]
fn config_absolute_path() {
    let c = Cli::parse_from(["storageshower", "--config", "/tmp/storageshower-test.conf"]);
    assert_eq!(c.config.as_deref(), Some("/tmp/storageshower-test.conf"));
}

#[test]
fn config_relative_path() {
    let c = Cli::parse_from(["storageshower", "-c", "./my.conf"]);
    assert_eq!(c.config.as_deref(), Some("./my.conf"));
}
