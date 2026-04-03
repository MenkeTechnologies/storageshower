//! `DiskEntry` and related `types` field behavior at the crate boundary.

use storageshower::types::{DiskEntry, SmartHealth};
use sysinfo::DiskKind;

#[test]
fn disk_entry_clone_independent_pct() {
    let a = DiskEntry {
        mount: "/a".into(),
        used: 10,
        total: 100,
        pct: 10.0,
        kind: DiskKind::SSD,
        fs: "apfs".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    };
    let mut b = a.clone();
    b.pct = 50.0;
    assert!((a.pct - 10.0).abs() < f64::EPSILON);
    assert!((b.pct - 50.0).abs() < f64::EPSILON);
}

#[test]
fn disk_entry_io_rates_roundtrip_clone() {
    let d = DiskEntry {
        mount: "/".into(),
        used: 1,
        total: 2,
        pct: 50.0,
        kind: DiskKind::HDD,
        fs: "ext4".into(),
        latency_ms: Some(1.5),
        io_read_rate: Some(1_048_576.0),
        io_write_rate: Some(512.0),
        smart_status: Some(SmartHealth::Unknown),
    };
    let c = d.clone();
    assert_eq!(c.io_read_rate, Some(1_048_576.0));
    assert_eq!(c.smart_status, Some(SmartHealth::Unknown));
}

#[test]
fn smart_health_exhaustive_copy() {
    let a = SmartHealth::Verified;
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn disk_kind_ssd_hdd_ne() {
    assert_ne!(DiskKind::SSD, DiskKind::HDD);
}

#[test]
fn disk_entry_mount_fs_preserved() {
    let d = DiskEntry {
        mount: "/srv".into(),
        used: 0,
        total: 1,
        pct: 0.0,
        kind: DiskKind::Unknown(0),
        fs: "zfs".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    };
    assert_eq!(d.fs, "zfs");
    assert_eq!(d.mount, "/srv");
}

#[test]
fn disk_entry_zero_used_total() {
    let d = DiskEntry {
        mount: "/empty".into(),
        used: 0,
        total: 0,
        pct: 0.0,
        kind: DiskKind::SSD,
        fs: "tmpfs".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    };
    assert_eq!(d.used, 0);
    assert_eq!(d.total, 0);
}
