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

pub fn format_latency(ms: f64) -> String {
    if ms < 1.0 {
        "<1ms".into()
    } else if ms < 1000.0 {
        format!("{:.0}ms", ms)
    } else {
        format!("{:.1}s", ms / 1000.0)
    }
}

pub fn format_rate(bytes_per_sec: f64) -> String {
    if bytes_per_sec < 1.0 {
        "0B/s".into()
    } else if bytes_per_sec < 1024.0 {
        format!("{:.0}B/s", bytes_per_sec)
    } else if bytes_per_sec < 1_048_576.0 {
        format!("{:.1}K/s", bytes_per_sec / 1024.0)
    } else if bytes_per_sec < 1_073_741_824.0 {
        format!("{:.1}M/s", bytes_per_sec / 1_048_576.0)
    } else {
        format!("{:.1}G/s", bytes_per_sec / 1_073_741_824.0)
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

    // ── format_bytes edge cases ─────────────────────────────

    #[test]
    fn format_bytes_zero_all_modes() {
        assert_eq!(format_bytes(0, UnitMode::Human), "0B");
        assert_eq!(format_bytes(0, UnitMode::Bytes), "0B");
        assert_eq!(format_bytes(0, UnitMode::GiB), "0.0G");
        assert_eq!(format_bytes(0, UnitMode::MiB), "0.0M");
    }

    #[test]
    fn format_bytes_one() {
        assert_eq!(format_bytes(1, UnitMode::Human), "1B");
        assert_eq!(format_bytes(1, UnitMode::Bytes), "1B");
    }

    #[test]
    fn format_bytes_boundary_kilo_minus_one() {
        assert_eq!(format_bytes(1023, UnitMode::Human), "1023B");
    }

    #[test]
    fn format_bytes_boundary_kilo() {
        assert_eq!(format_bytes(1024, UnitMode::Human), "1.0K");
    }

    #[test]
    fn format_bytes_boundary_mega_minus_one() {
        assert_eq!(format_bytes(1_048_575, UnitMode::Human), "1024.0K");
    }

    #[test]
    fn format_bytes_boundary_mega() {
        assert_eq!(format_bytes(1_048_576, UnitMode::Human), "1.0M");
    }

    #[test]
    fn format_bytes_boundary_giga_minus_one() {
        assert_eq!(format_bytes(1_073_741_823, UnitMode::Human), "1024.0M");
    }

    #[test]
    fn format_bytes_boundary_giga() {
        assert_eq!(format_bytes(1_073_741_824, UnitMode::Human), "1.0G");
    }

    #[test]
    fn format_bytes_boundary_tera_minus_one() {
        assert_eq!(format_bytes(1_099_511_627_775, UnitMode::Human), "1024.0G");
    }

    #[test]
    fn format_bytes_boundary_tera() {
        assert_eq!(format_bytes(1_099_511_627_776, UnitMode::Human), "1.0T");
    }

    #[test]
    fn format_bytes_max_u64() {
        // Should not panic
        let _ = format_bytes(u64::MAX, UnitMode::Human);
        let _ = format_bytes(u64::MAX, UnitMode::Bytes);
        let _ = format_bytes(u64::MAX, UnitMode::GiB);
        let _ = format_bytes(u64::MAX, UnitMode::MiB);
    }

    // ── format_uptime edge cases ────────────────────────────

    #[test]
    fn format_uptime_large_value() {
        // 10 years
        let secs = 10 * 365 * 86400;
        let r = format_uptime(secs);
        assert!(r.starts_with("3650d"));
    }

    #[test]
    fn format_uptime_59_seconds() {
        assert_eq!(format_uptime(59), "0m");
    }

    #[test]
    fn format_uptime_60_seconds() {
        assert_eq!(format_uptime(60), "1m");
    }

    #[test]
    fn format_uptime_23h59m() {
        assert_eq!(format_uptime(86399), "23h59m");
    }

    // ── truncate_mount edge cases ───────────────────────────

    #[test]
    fn truncate_mount_unicode() {
        let r = truncate_mount("/日本語/パス", 8);
        assert!(r.chars().count() <= 8);
    }

    #[test]
    fn truncate_mount_width_equals_string() {
        let s = "/mnt/data";
        let r = truncate_mount(s, s.len());
        assert_eq!(r, s);
    }

    #[test]
    fn truncate_mount_width_one_more_than_string() {
        let s = "/mnt";
        let r = truncate_mount(s, s.len() + 1);
        assert!(r.starts_with(s));
        assert_eq!(r.chars().count(), s.len() + 1);
    }

    // ── format_latency ──────────────────────────────────────

    #[test]
    fn format_latency_sub_millisecond() {
        assert_eq!(format_latency(0.3), "<1ms");
        assert_eq!(format_latency(0.0), "<1ms");
    }

    #[test]
    fn format_latency_milliseconds() {
        assert_eq!(format_latency(1.0), "1ms");
        assert_eq!(format_latency(12.4), "12ms");
        assert_eq!(format_latency(150.0), "150ms");
        assert_eq!(format_latency(999.9), "1000ms");
    }

    #[test]
    fn format_latency_seconds() {
        assert_eq!(format_latency(1000.0), "1.0s");
        assert_eq!(format_latency(2500.0), "2.5s");
    }

    // ── format_rate ─────────────────────────────────────

    #[test]
    fn format_rate_zero() {
        assert_eq!(format_rate(0.0), "0B/s");
    }

    #[test]
    fn format_rate_bytes() {
        assert_eq!(format_rate(512.0), "512B/s");
    }

    #[test]
    fn format_rate_kilobytes() {
        assert_eq!(format_rate(1024.0), "1.0K/s");
        assert_eq!(format_rate(10240.0), "10.0K/s");
    }

    #[test]
    fn format_rate_megabytes() {
        assert_eq!(format_rate(1_048_576.0), "1.0M/s");
        assert_eq!(format_rate(52_428_800.0), "50.0M/s");
    }

    #[test]
    fn format_rate_gigabytes() {
        assert_eq!(format_rate(1_073_741_824.0), "1.0G/s");
    }

    #[test]
    fn format_rate_sub_one_is_zero_band() {
        assert_eq!(format_rate(0.5), "0B/s");
        assert_eq!(format_rate(0.99), "0B/s");
    }

    #[test]
    fn format_rate_just_below_kilo_boundary() {
        assert_eq!(format_rate(1023.9), "1024B/s");
    }

    #[test]
    fn format_rate_just_below_mebi_boundary() {
        let x = 1_048_576.0 - 1.0;
        let s = format_rate(x);
        assert!(s.ends_with("K/s"), "got {s}");
    }

    #[test]
    fn format_rate_just_below_gibi_boundary() {
        let x = 1_073_741_824.0 - 1.0;
        let s = format_rate(x);
        assert!(s.ends_with("M/s"), "got {s}");
    }

    #[test]
    fn format_rate_very_large() {
        let s = format_rate(50.0 * 1_073_741_824.0);
        assert_eq!(s, "50.0G/s");
    }

    #[test]
    fn format_latency_boundary_ms_vs_s() {
        assert_eq!(format_latency(999.0), "999ms");
        assert_eq!(format_latency(1000.0), "1.0s");
    }

    #[test]
    fn format_latency_fractional_ms_rounds() {
        assert_eq!(format_latency(42.7), "43ms");
    }

    #[test]
    fn truncate_mount_width_zero_nonempty() {
        let r = truncate_mount("/ab", 0);
        assert_eq!(r, "\u{2026}");
    }

    #[test]
    fn truncate_mount_width_one() {
        let r = truncate_mount("/ab", 1);
        assert_eq!(r.chars().count(), 1);
    }

    #[test]
    fn format_bytes_human_one_below_tera() {
        assert_eq!(format_bytes(1_099_511_627_775, UnitMode::Human), "1024.0G");
    }

    #[test]
    fn format_uptime_day_with_nonzero_hours_minutes() {
        assert_eq!(format_uptime(86400 + 3661), "1d1h1m");
    }

    #[test]
    fn format_uptime_hour_only_no_days() {
        assert_eq!(format_uptime(7200), "2h0m");
    }

    #[test]
    fn format_rate_negative_falls_through_zero_band() {
        assert_eq!(format_rate(-50.0), "0B/s");
    }

    #[test]
    fn format_bytes_human_exactly_one_byte_below_kilo() {
        assert_eq!(format_bytes(1023, UnitMode::Human), "1023B");
    }

    #[test]
    fn format_bytes_gib_mode_fraction() {
        assert_eq!(format_bytes(536_870_912, UnitMode::GiB), "0.5G");
    }

    #[test]
    fn format_bytes_mib_exact_one() {
        assert_eq!(format_bytes(1_048_576, UnitMode::MiB), "1.0M");
    }

    #[test]
    fn format_latency_exactly_one_ms() {
        assert_eq!(format_latency(1.0), "1ms");
    }

    #[test]
    fn format_bytes_bytes_mode_three_digits() {
        assert_eq!(format_bytes(999, UnitMode::Bytes), "999B");
    }

    #[test]
    fn format_uptime_only_minutes_no_hours() {
        assert_eq!(format_uptime(3599), "59m");
    }

    #[test]
    fn format_bytes_bytes_mode_one_mebibyte() {
        assert_eq!(format_bytes(1_048_576, UnitMode::Bytes), "1048576B");
    }

    #[test]
    fn format_latency_ten_seconds() {
        assert_eq!(format_latency(10_000.0), "10.0s");
    }
}
