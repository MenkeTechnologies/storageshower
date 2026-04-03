//! `format_bytes` across `UnitMode` values (public `helpers` API).

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn gib_half_terabyte() {
    assert_eq!(format_bytes(512 * 1_073_741_824, UnitMode::GiB), "512.0G");
}

#[test]
fn mib_exactly_1024_mib() {
    assert_eq!(format_bytes(1024 * 1_048_576, UnitMode::MiB), "1024.0M");
}

#[test]
fn bytes_mode_large_number() {
    let s = format_bytes(9_999_999_999, UnitMode::Bytes);
    assert_eq!(s, "9999999999B");
}

#[test]
fn human_one_byte() {
    assert_eq!(format_bytes(1, UnitMode::Human), "1B");
}

#[test]
fn gib_small_fraction() {
    let s = format_bytes(100_000, UnitMode::GiB);
    assert!(s.contains('G'), "got {s}");
}

#[test]
fn mib_small_fraction() {
    let s = format_bytes(50_000, UnitMode::MiB);
    assert!(s.contains('M'), "got {s}");
}

#[test]
fn all_modes_same_zero() {
    let z = 0u64;
    assert_eq!(format_bytes(z, UnitMode::Human), "0B");
    assert_eq!(format_bytes(z, UnitMode::Bytes), "0B");
    assert_eq!(format_bytes(z, UnitMode::GiB), "0.0G");
    assert_eq!(format_bytes(z, UnitMode::MiB), "0.0M");
}
