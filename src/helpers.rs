use crate::types::UnitMode;

pub fn format_bytes(b: u64, mode: UnitMode) -> String {
    match mode {
        UnitMode::Bytes => format!("{b}B"),
        UnitMode::MiB => format!("{:.1}M", b as f64 / 1_048_576.0),
        UnitMode::GiB => format!("{:.1}G", b as f64 / 1_073_741_824.0),
        UnitMode::Human => {
            if b >= 1_099_511_627_776 {
                format!("{:.1}T", b as f64 / 1_099_511_627_776.0)
            } else if b >= 1_073_741_824 {
                format!("{:.1}G", b as f64 / 1_073_741_824.0)
            } else if b >= 1_048_576 {
                format!("{:.1}M", b as f64 / 1_048_576.0)
            } else if b >= 1024 {
                format!("{:.1}K", b as f64 / 1024.0)
            } else {
                format!("{b}B")
            }
        }
    }
}

pub fn format_uptime(secs: u64) -> String {
    let d = secs / 86400;
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    if d > 0 {
        format!("{d}d{h}h{m}m")
    } else if h > 0 {
        format!("{h}h{m}m")
    } else {
        format!("{m}m")
    }
}

pub fn truncate_mount(mount: &str, width: usize) -> String {
    if mount.chars().count() <= width {
        format!("{:<width$}", mount, width = width)
    } else {
        let s: String = mount.chars().take(width.saturating_sub(1)).collect();
        format!("{}\u{2026}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── format_bytes ───────────────────────────────────────

    #[test]
    fn format_bytes_zero() {
        assert_eq!(format_bytes(0, UnitMode::Human), "0B");
        assert_eq!(format_bytes(0, UnitMode::Bytes), "0B");
        assert_eq!(format_bytes(0, UnitMode::GiB), "0.0G");
        assert_eq!(format_bytes(0, UnitMode::MiB), "0.0M");
    }

    #[test]
    fn format_bytes_human_small() {
        assert_eq!(format_bytes(512, UnitMode::Human), "512B");
        assert_eq!(format_bytes(1023, UnitMode::Human), "1023B");
    }

    #[test]
    fn format_bytes_human_kilo() {
        assert_eq!(format_bytes(1024, UnitMode::Human), "1.0K");
        assert_eq!(format_bytes(1536, UnitMode::Human), "1.5K");
    }

    #[test]
    fn format_bytes_human_mega() {
        assert_eq!(format_bytes(1_048_576, UnitMode::Human), "1.0M");
        assert_eq!(format_bytes(10 * 1_048_576, UnitMode::Human), "10.0M");
    }

    #[test]
    fn format_bytes_human_giga() {
        assert_eq!(format_bytes(1_073_741_824, UnitMode::Human), "1.0G");
        assert_eq!(format_bytes(5 * 1_073_741_824, UnitMode::Human), "5.0G");
    }

    #[test]
    fn format_bytes_human_tera() {
        assert_eq!(format_bytes(1_099_511_627_776, UnitMode::Human), "1.0T");
        assert_eq!(format_bytes(2 * 1_099_511_627_776, UnitMode::Human), "2.0T");
    }

    #[test]
    fn format_bytes_explicit_modes() {
        let gb = 1_073_741_824u64;
        assert_eq!(format_bytes(gb, UnitMode::Bytes), "1073741824B");
        assert_eq!(format_bytes(gb, UnitMode::MiB), "1024.0M");
        assert_eq!(format_bytes(gb, UnitMode::GiB), "1.0G");
    }

    // ── format_uptime ──────────────────────────────────────

    #[test]
    fn format_uptime_minutes_only() {
        assert_eq!(format_uptime(0), "0m");
        assert_eq!(format_uptime(59), "0m");
        assert_eq!(format_uptime(60), "1m");
        assert_eq!(format_uptime(300), "5m");
    }

    #[test]
    fn format_uptime_hours() {
        assert_eq!(format_uptime(3600), "1h0m");
        assert_eq!(format_uptime(3660), "1h1m");
        assert_eq!(format_uptime(7200), "2h0m");
    }

    #[test]
    fn format_uptime_days() {
        assert_eq!(format_uptime(86400), "1d0h0m");
        assert_eq!(format_uptime(90000), "1d1h0m");
        assert_eq!(format_uptime(2 * 86400 + 3 * 3600 + 15 * 60), "2d3h15m");
    }

    // ── truncate_mount ─────────────────────────────────────

    #[test]
    fn truncate_mount_fits() {
        let r = truncate_mount("/mnt", 10);
        assert_eq!(r.len(), 10);
        assert!(r.starts_with("/mnt"));
    }

    #[test]
    fn truncate_mount_exact() {
        let r = truncate_mount("/mnt", 4);
        assert_eq!(r, "/mnt");
    }

    #[test]
    fn truncate_mount_too_long() {
        let r = truncate_mount("/very/long/mount/path", 10);
        assert_eq!(r.chars().count(), 10);
        assert!(r.ends_with('\u{2026}')); // ends with …
    }

    #[test]
    fn truncate_mount_empty() {
        let r = truncate_mount("", 5);
        assert_eq!(r.len(), 5);
    }
}
