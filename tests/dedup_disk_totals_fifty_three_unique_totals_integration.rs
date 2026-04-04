//! `dedup_disk_totals` with fifty-three mounts and fifty-three distinct `total` values.

use storageshower::system::dedup_disk_totals;
use storageshower::types::DiskEntry;
use sysinfo::DiskKind;

fn disk(mount: &str, used: u64, total: u64) -> DiskEntry {
    DiskEntry {
        mount: mount.into(),
        used,
        total,
        pct: 0.0,
        kind: DiskKind::SSD,
        fs: "ext4".into(),
        latency_ms: None,
        io_read_rate: None,
        io_write_rate: None,
        smart_status: None,
    }
}

#[test]
fn fifty_three_distinct_totals_sum_all() {
    let d = vec![
        disk("/a", 1, 10),
        disk("/b", 2, 20),
        disk("/c", 3, 30),
        disk("/d", 4, 40),
        disk("/e", 5, 50),
        disk("/f", 6, 60),
        disk("/g", 7, 70),
        disk("/h", 8, 80),
        disk("/i", 9, 90),
        disk("/j", 10, 100),
        disk("/k", 11, 110),
        disk("/l", 12, 120),
        disk("/m", 13, 130),
        disk("/n", 14, 140),
        disk("/o", 15, 150),
        disk("/p", 16, 160),
        disk("/q", 17, 170),
        disk("/r", 18, 180),
        disk("/s", 19, 190),
        disk("/t", 20, 200),
        disk("/u", 21, 210),
        disk("/v", 22, 220),
        disk("/w", 23, 230),
        disk("/x", 24, 240),
        disk("/y", 25, 250),
        disk("/z", 26, 260),
        disk("/aa", 27, 270),
        disk("/ab", 28, 280),
        disk("/ac", 29, 290),
        disk("/ad", 30, 300),
        disk("/ae", 31, 310),
        disk("/af", 32, 320),
        disk("/ag", 33, 330),
        disk("/ah", 34, 340),
        disk("/ai", 35, 350),
        disk("/aj", 36, 360),
        disk("/ak", 37, 370),
        disk("/al", 38, 380),
        disk("/am", 39, 390),
        disk("/an", 40, 400),
        disk("/ao", 41, 410),
        disk("/ap", 42, 420),
        disk("/aq", 43, 430),
        disk("/ar", 44, 440),
        disk("/as", 45, 450),
        disk("/at", 46, 460),
        disk("/au", 47, 470),
        disk("/av", 48, 480),
        disk("/aw", 49, 490),
        disk("/ax", 50, 500),
        disk("/ay", 51, 510),
        disk("/az", 52, 520),
        disk("/ba", 53, 530),
    ];
    assert_eq!(dedup_disk_totals(&d), (14310, 1431));
}
