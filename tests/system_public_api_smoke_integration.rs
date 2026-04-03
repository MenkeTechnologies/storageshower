//! Smoke tests for `storageshower::system` public helpers from an external test crate.

use storageshower::system::{
    chrono_now, collect_disk_entries, collect_sys_stats, dedup_disk_totals, get_local_ip,
    get_username, is_network_fs, scan_directory,
};
use sysinfo::DiskKind;
use sysinfo::System;

#[test]
fn chrono_now_date_has_dots_and_time_has_colons() {
    let (date, time) = chrono_now();
    assert_eq!(date.len(), 10);
    assert_eq!(date.chars().filter(|c| *c == '.').count(), 2);
    assert_eq!(time.len(), 8);
    assert_eq!(time.chars().filter(|c| *c == ':').count(), 2);
}

#[test]
fn chrono_now_year_is_sane() {
    let (date, _) = chrono_now();
    let y: i32 = date[..4].parse().expect("year");
    assert!((2020..=2120).contains(&y));
}

#[test]
fn get_local_ip_non_empty() {
    let ip = get_local_ip();
    assert!(!ip.is_empty());
}

#[test]
fn get_username_does_not_panic() {
    let _ = get_username();
}

#[test]
fn collect_sys_stats_positive_memory_and_cpu() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let s = collect_sys_stats(&sys);
    assert!(s.mem_total > 0, "mem_total");
    assert!(s.cpu_count > 0, "cpu_count");
}

#[test]
fn collect_sys_stats_hostname_not_empty() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let s = collect_sys_stats(&sys);
    assert!(!s.hostname.is_empty());
}

#[test]
fn scan_directory_nonexistent_returns_empty() {
    let p = "/no/such/path/storageshower/scan/xyz";
    assert!(scan_directory(p).is_empty());
}

#[test]
fn is_network_fs_rejects_tmpfs() {
    assert!(!is_network_fs("tmpfs"));
}

#[test]
fn dedup_disk_totals_public_api_empty() {
    assert_eq!(dedup_disk_totals(&[]), (0, 0));
}

#[test]
fn collect_disk_entries_each_has_nonempty_mount() {
    let disks = collect_disk_entries();
    for d in &disks {
        assert!(!d.mount.is_empty(), "empty mount");
    }
}

#[test]
fn collect_disk_entries_pct_in_0_100() {
    let disks = collect_disk_entries();
    for d in &disks {
        assert!(
            d.pct >= 0.0 && d.pct <= 100.0,
            "pct {} for {}",
            d.pct,
            d.mount
        );
    }
}

#[test]
fn disk_entry_fields_sensible_for_first_mount_if_any() {
    let disks = collect_disk_entries();
    let Some(d) = disks.first() else {
        return;
    };
    assert!(!d.fs.is_empty());
    assert_eq!(d.mount.chars().next(), Some('/'));
    // DiskKind is always a valid enum variant
    let _ = matches!(d.kind, DiskKind::SSD | DiskKind::HDD | DiskKind::Unknown(_));
}
