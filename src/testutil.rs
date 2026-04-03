use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use std::sync::{Arc, Mutex};
use sysinfo::DiskKind;

use crate::app::App;
use crate::prefs::Prefs;
use crate::types::{DiskEntry, SysStats};

pub fn make_key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

pub fn make_ctrl_key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

pub fn test_disks() -> Vec<DiskEntry> {
    vec![
        DiskEntry {
            mount: "/".into(),
            used: 50_000_000_000,
            total: 100_000_000_000,
            pct: 50.0,
            kind: DiskKind::SSD,
            fs: "apfs".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        },
        DiskEntry {
            mount: "/home".into(),
            used: 80_000_000_000,
            total: 200_000_000_000,
            pct: 40.0,
            kind: DiskKind::SSD,
            fs: "ext4".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        },
        DiskEntry {
            mount: "/data".into(),
            used: 900_000_000_000,
            total: 1_000_000_000_000,
            pct: 90.0,
            kind: DiskKind::HDD,
            fs: "xfs".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        },
        DiskEntry {
            mount: "/tmp".into(),
            used: 100_000,
            total: 500_000_000,
            pct: 0.02,
            kind: DiskKind::Unknown(-1),
            fs: "tmpfs".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        },
    ]
}

pub fn test_app() -> App {
    let stats = SysStats::default();
    let disks = test_disks();
    let shared = Arc::new(Mutex::new((stats.clone(), disks.clone())));
    let mut app = App::new_default(shared);
    app.disks = disks;
    app.stats = stats;
    app.prefs = Prefs::default();
    app.test_mode = true;
    app.update_sorted();
    app
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disks_count_and_ordering() {
        let d = test_disks();
        assert_eq!(d.len(), 4);
        assert_eq!(d[0].mount, "/");
        assert_eq!(d[1].mount, "/home");
        assert_eq!(d[2].mount, "/data");
        assert_eq!(d[3].mount, "/tmp");
    }

    #[test]
    fn test_disks_percentages_and_fs() {
        let d = test_disks();
        assert!((d[0].pct - 50.0).abs() < f64::EPSILON);
        assert_eq!(d[0].fs, "apfs");
        assert!((d[2].pct - 90.0).abs() < f64::EPSILON);
        assert_eq!(d[2].fs, "xfs");
    }

    #[test]
    fn test_app_test_mode_and_sorted() {
        let app = test_app();
        assert!(app.test_mode);
        assert!(!app.disks.is_empty());
    }

    #[test]
    fn make_key_codes() {
        assert_eq!(make_key(KeyCode::Char('q')).code, KeyCode::Char('q'));
        assert_eq!(make_key(KeyCode::Esc).code, KeyCode::Esc);
    }

    #[test]
    fn make_ctrl_key_has_control_modifier() {
        let e = make_ctrl_key(KeyCode::Char('c'));
        assert_eq!(e.code, KeyCode::Char('c'));
        assert!(e.modifiers.contains(KeyModifiers::CONTROL));
    }
}
