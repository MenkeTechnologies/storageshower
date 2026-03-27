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
