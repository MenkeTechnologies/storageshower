use std::time::Duration;
use std::sync::{Arc, Mutex};
use sysinfo::{DiskKind, System};

use crate::types::*;

// ─── Time helpers ──────────────────────────────────────────────────────────

pub fn chrono_now() -> (String, String) {
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (y, mo, d, h, mi, s) = epoch_to_local(epoch as i64);
    (
        format!("{:04}.{:02}.{:02}", y, mo, d),
        format!("{:02}:{:02}:{:02}", h, mi, s),
    )
}

#[cfg(unix)]
pub fn epoch_to_local(epoch: i64) -> (i32, u32, u32, u32, u32, u32) {
    unsafe {
        let mut tm: libc::tm = std::mem::zeroed();
        let t = epoch as libc::time_t;
        libc::localtime_r(&t, &mut tm);
        (
            tm.tm_year as i32 + 1900,
            tm.tm_mon as u32 + 1,
            tm.tm_mday as u32,
            tm.tm_hour as u32,
            tm.tm_min as u32,
            tm.tm_sec as u32,
        )
    }
}

#[cfg(not(unix))]
pub fn epoch_to_local(epoch: i64) -> (i32, u32, u32, u32, u32, u32) {
    let secs_per_day = 86400i64;
    let mut days = epoch / secs_per_day;
    let day_secs = (epoch % secs_per_day) as u32;
    let hh = day_secs / 3600;
    let mm = (day_secs % 3600) / 60;
    let ss = day_secs % 60;
    let mut y = 1970i32;
    loop {
        let dy: i64 = if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 { 366 } else { 365 };
        if days < dy { break; }
        days -= dy;
        y += 1;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let mdays: [i64; 12] = [31, if leap {29} else {28}, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut mo = 1u32;
    for i in 0..12 {
        if days < mdays[i] { mo = i as u32 + 1; break; }
        days -= mdays[i];
    }
    (y, mo, days as u32 + 1, hh, mm, ss)
}

// ─── System info helpers ───────────────────────────────────────────────────

pub fn get_username() -> Option<String> {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .ok()
}

pub fn get_local_ip() -> String {
    std::net::UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| {
            s.connect("8.8.8.8:80")?;
            s.local_addr()
        })
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}

#[cfg(unix)]
pub fn get_tty() -> Option<String> {
    unsafe {
        let name = libc::ttyname(0);
        if name.is_null() {
            None
        } else {
            Some(std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned())
        }
    }
}

#[cfg(not(unix))]
pub fn get_tty() -> Option<String> {
    None
}

pub fn get_battery() -> Option<u8> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("pmset")
            .args(["-g", "batt"])
            .output()
            .ok()
            .and_then(|o| {
                let s = String::from_utf8_lossy(&o.stdout);
                for word in s.split_whitespace() {
                    if word.ends_with("%;") || word.ends_with('%') {
                        let num: String = word.chars().take_while(|c| c.is_ascii_digit()).collect();
                        if let Ok(v) = num.parse::<u8>() {
                            return Some(v);
                        }
                    }
                }
                None
            })
    }
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .ok()
            .and_then(|s| s.trim().parse().ok())
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

// ─── Disk collection ───────────────────────────────────────────────────────

pub fn collect_disk_entries() -> Vec<DiskEntry> {
    collect_all_mounts()
}

#[cfg(target_os = "macos")]
fn collect_all_mounts() -> Vec<DiskEntry> {
    use std::ffi::CStr;
    unsafe {
        let mut mntbuf: *mut libc::statfs = std::ptr::null_mut();
        let count = libc::getmntinfo(&mut mntbuf, libc::MNT_NOWAIT);
        if count <= 0 || mntbuf.is_null() {
            return Vec::new();
        }
        let entries = std::slice::from_raw_parts(mntbuf, count as usize);
        entries
            .iter()
            .map(|fs| {
                let mount = CStr::from_ptr(fs.f_mntonname.as_ptr())
                    .to_string_lossy()
                    .to_string();
                let fstype = CStr::from_ptr(fs.f_fstypename.as_ptr())
                    .to_string_lossy()
                    .to_string();
                let total = fs.f_blocks * fs.f_bsize as u64;
                let avail = fs.f_bavail * fs.f_bsize as u64;
                let used = total.saturating_sub(avail);
                let pct = if total > 0 {
                    (used as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                let kind = if fstype == "apfs" || fstype == "hfs" {
                    DiskKind::SSD
                } else {
                    DiskKind::Unknown(-1)
                };
                DiskEntry { mount, used, total, pct, kind, fs: fstype }
            })
            .collect()
    }
}

#[cfg(target_os = "linux")]
fn collect_all_mounts() -> Vec<DiskEntry> {
    let content = match std::fs::read_to_string("/proc/mounts") {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                return None;
            }
            let mount = parts[1].to_string();
            let fstype = parts[2].to_string();
            let mut stat: std::mem::MaybeUninit<libc::statvfs> = std::mem::MaybeUninit::uninit();
            let c_mount = std::ffi::CString::new(mount.as_str()).ok()?;
            let (total, used, pct) = unsafe {
                if libc::statvfs(c_mount.as_ptr(), stat.as_mut_ptr()) == 0 {
                    let s = stat.assume_init();
                    let total = s.f_blocks * s.f_frsize;
                    let avail = s.f_bavail * s.f_frsize;
                    let used = total.saturating_sub(avail);
                    let pct = if total > 0 {
                        (used as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };
                    (total, used, pct)
                } else {
                    (0, 0, 0.0)
                }
            };
            Some(DiskEntry {
                mount,
                used,
                total,
                pct,
                kind: DiskKind::Unknown(-1),
                fs: fstype,
            })
        })
        .collect()
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn collect_all_mounts() -> Vec<DiskEntry> {
    use sysinfo::Disks;
    let disks = Disks::new_with_refreshed_list();
    disks
        .list()
        .iter()
        .map(|d| {
            let total = d.total_space();
            let avail = d.available_space();
            let used = total.saturating_sub(avail);
            let pct = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            DiskEntry {
                mount: d.mount_point().to_string_lossy().to_string(),
                used,
                total,
                pct,
                kind: d.kind(),
                fs: d.file_system().to_string_lossy().to_string(),
            }
        })
        .collect()
}

// ─── Stats collection ──────────────────────────────────────────────────────

pub fn collect_sys_stats(sys: &System) -> SysStats {
    let load = System::load_average();
    SysStats {
        hostname: System::host_name().unwrap_or_default(),
        load_avg: (load.one, load.five, load.fifteen),
        mem_used: sys.used_memory(),
        mem_total: sys.total_memory(),
        cpu_count: sys.cpus().len(),
        process_count: sys.processes().len(),
        swap_used: sys.used_swap(),
        swap_total: sys.total_swap(),
        kernel: System::kernel_version().unwrap_or_default(),
        arch: System::cpu_arch().unwrap_or_default(),
        uptime: System::uptime(),
        os_name: System::name().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
    }
}

pub fn spawn_bg_collector(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>) {
    std::thread::spawn(move || {
        let mut sys = System::new_all();
        loop {
            std::thread::sleep(Duration::from_secs(3));
            sys.refresh_all();
            let stats = collect_sys_stats(&sys);
            let entries = collect_disk_entries();
            {
                let mut lock = shared.lock().unwrap();
                *lock = (stats, entries);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrono_now_returns_valid_format() {
        let (date, time) = chrono_now();
        // date: "YYYY.MM.DD"
        assert_eq!(date.len(), 10);
        assert_eq!(date.chars().nth(4), Some('.'));
        assert_eq!(date.chars().nth(7), Some('.'));
        // time: "HH:MM:SS"
        assert_eq!(time.len(), 8);
        assert_eq!(time.chars().nth(2), Some(':'));
        assert_eq!(time.chars().nth(5), Some(':'));
    }

    #[test]
    fn chrono_now_year_reasonable() {
        let (date, _) = chrono_now();
        let year: i32 = date[..4].parse().unwrap();
        assert!(year >= 2024 && year <= 2100);
    }

    #[test]
    fn get_username_returns_something() {
        // On CI/dev machines USER is almost always set
        let user = get_username();
        // Just check it doesn't panic; may be None in unusual environments
        if let Some(u) = user {
            assert!(!u.is_empty());
        }
    }

    #[test]
    fn get_local_ip_returns_valid_ip() {
        let ip = get_local_ip();
        assert!(!ip.is_empty());
        // Should be parseable as an IP or fallback
        assert!(ip.contains('.') || ip == "127.0.0.1");
    }

    #[test]
    fn collect_disk_entries_returns_something() {
        let disks = collect_disk_entries();
        // On any real system there should be at least one disk
        assert!(!disks.is_empty(), "Expected at least one disk entry");
    }

    #[test]
    fn collect_disk_entries_have_mount_points() {
        let disks = collect_disk_entries();
        for d in &disks {
            assert!(!d.mount.is_empty(), "Disk mount should not be empty");
        }
    }

    #[test]
    fn collect_disk_entries_pct_in_range() {
        let disks = collect_disk_entries();
        for d in &disks {
            assert!(d.pct >= 0.0 && d.pct <= 100.0,
                "Disk {} pct {} out of range", d.mount, d.pct);
        }
    }

    #[test]
    fn collect_sys_stats_returns_valid() {
        let sys = System::new_all();
        let stats = collect_sys_stats(&sys);
        assert!(stats.mem_total > 0, "mem_total should be > 0");
        assert!(stats.cpu_count > 0, "cpu_count should be > 0");
    }

    #[test]
    fn collect_sys_stats_hostname_not_empty() {
        let sys = System::new_all();
        let stats = collect_sys_stats(&sys);
        assert!(!stats.hostname.is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn get_tty_does_not_panic() {
        // May fail in CI but should not panic
        let _ = get_tty();
    }

    #[test]
    fn get_battery_does_not_panic() {
        let _ = get_battery();
    }
}
