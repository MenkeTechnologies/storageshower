use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
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
#[allow(clippy::unnecessary_cast)]
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
        let dy: i64 = if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
            366
        } else {
            365
        };
        if days < dy {
            break;
        }
        days -= dy;
        y += 1;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let mdays: [i64; 12] = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut mo = 1u32;
    for i in 0..12 {
        if days < mdays[i] {
            mo = i as u32 + 1;
            break;
        }
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
            Some(
                std::ffi::CStr::from_ptr(name)
                    .to_string_lossy()
                    .into_owned(),
            )
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

// ─── Network filesystem helpers ───────────────────────────────────────────

pub fn is_network_fs(fs: &str) -> bool {
    matches!(
        fs,
        "nfs"
            | "nfs4"
            | "cifs"
            | "smbfs"
            | "afp"
            | "ncp"
            | "fuse.sshfs"
            | "fuse.rclone"
            | "fuse.s3fs"
            | "9p"
            | "afs"
    )
}

fn measure_mount_latency(mount: &str) -> Option<f64> {
    let mount = mount.to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let start = std::time::Instant::now();
        match std::fs::read_dir(&mount) {
            Ok(mut rd) => {
                let _ = rd.next();
                let _ = tx.send(Some(start.elapsed().as_secs_f64() * 1000.0));
            }
            Err(_) => {
                let _ = tx.send(None);
            }
        }
    });
    rx.recv_timeout(Duration::from_secs(2)).ok().flatten()
}

// ─── Directory scanning ───────────────────────────────────────────────────

fn dir_size(path: &std::path::Path) -> u64 {
    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    let mut total = 0u64;
    for entry in entries.flatten() {
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.is_dir() {
            total += dir_size(&entry.path());
        } else {
            total += meta.len();
        }
    }
    total
}

pub fn scan_directory(path: &str) -> Vec<DirEntry> {
    scan_directory_with_progress(path, None, None)
}

pub fn scan_directory_with_progress(
    path: &str,
    count: Option<Arc<Mutex<usize>>>,
    total: Option<Arc<Mutex<usize>>>,
) -> Vec<DirEntry> {
    let entries: Vec<_> = match std::fs::read_dir(path) {
        Ok(e) => e.flatten().collect(),
        Err(_) => return Vec::new(),
    };
    let entry_count = entries.len();
    if let Some(ref t) = total {
        *t.lock().unwrap() = entry_count;
    }
    let mut results: Vec<DirEntry> = entries
        .into_iter()
        .enumerate()
        .filter_map(|(i, entry)| {
            if let Some(ref c) = count {
                *c.lock().unwrap() = i + 1;
            }
            let meta = entry.metadata().ok()?;
            let name = entry.file_name().to_string_lossy().to_string();
            let full_path = entry.path().to_string_lossy().to_string();
            let is_dir = meta.is_dir();
            let size = if is_dir {
                dir_size(&entry.path())
            } else {
                meta.len()
            };
            Some(DirEntry {
                path: full_path,
                name,
                size,
                is_dir,
            })
        })
        .collect();
    results.sort_by(|a, b| b.size.cmp(&a.size));
    results
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
                let latency_ms = if is_network_fs(&fstype) {
                    measure_mount_latency(&mount)
                } else {
                    None
                };
                DiskEntry {
                    mount,
                    used,
                    total,
                    pct,
                    kind,
                    fs: fstype,
                    latency_ms,
                    io_read_rate: None,
                    io_write_rate: None,
                    smart_status: None,
                }
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
            let latency_ms = if is_network_fs(&fstype) {
                measure_mount_latency(&mount)
            } else {
                None
            };
            Some(DiskEntry {
                mount,
                used,
                total,
                pct,
                kind: DiskKind::Unknown(-1),
                fs: fstype,
                latency_ms,
                io_read_rate: None,
                io_write_rate: None,
                smart_status: None,
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
            let mount = d.mount_point().to_string_lossy().to_string();
            let fs = d.file_system().to_string_lossy().to_string();
            let latency_ms = if is_network_fs(&fs) {
                measure_mount_latency(&mount)
            } else {
                None
            };
            DiskEntry {
                mount,
                used,
                total,
                pct,
                kind: d.kind(),
                fs,
                latency_ms,
                io_read_rate: None,
                io_write_rate: None,
                smart_status: None,
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

// ─── Disk I/O collection ──────────────────────────────────────────────────

/// Per-device I/O counters: (bytes_read, bytes_written)
type IoSnapshot = HashMap<String, (u64, u64)>;

/// Per-mount I/O rates: (read_bytes_per_sec, write_bytes_per_sec)
type IoRates = HashMap<String, (f64, f64)>;

/// Map mount points to their underlying device names.
#[cfg(target_os = "macos")]
fn mount_to_device_map() -> HashMap<String, String> {
    use std::ffi::CStr;
    let mut map = HashMap::new();
    unsafe {
        let mut mntbuf: *mut libc::statfs = std::ptr::null_mut();
        let count = libc::getmntinfo(&mut mntbuf, libc::MNT_NOWAIT);
        if count > 0 && !mntbuf.is_null() {
            let entries = std::slice::from_raw_parts(mntbuf, count as usize);
            for fs in entries {
                let mount = CStr::from_ptr(fs.f_mntonname.as_ptr())
                    .to_string_lossy()
                    .to_string();
                let device = CStr::from_ptr(fs.f_mntfromname.as_ptr())
                    .to_string_lossy()
                    .to_string();
                // Extract base device: /dev/disk3s1 -> disk3
                if let Some(dev) = device.strip_prefix("/dev/") {
                    let base: String = dev
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric())
                        .collect();
                    if !base.is_empty() {
                        map.insert(mount, base);
                    }
                }
            }
        }
    }
    map
}

#[cfg(target_os = "linux")]
fn mount_to_device_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let device = parts[0];
                let mount = parts[1].to_string();
                // Extract base device: /dev/sda1 -> sda, /dev/nvme0n1p1 -> nvme0n1
                if let Some(dev) = device.strip_prefix("/dev/") {
                    let base = dev
                        .trim_end_matches(|c: char| c.is_ascii_digit())
                        .trim_end_matches('p');
                    if !base.is_empty() {
                        map.insert(mount, base.to_string());
                    }
                }
            }
        }
    }
    map
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn mount_to_device_map() -> HashMap<String, String> {
    HashMap::new()
}

/// Read current per-device I/O byte counters.
#[cfg(target_os = "macos")]
fn read_io_counters() -> IoSnapshot {
    use std::process::Command;
    let mut snap = HashMap::new();
    // Parse ioreg for IOBlockStorageDriver statistics
    let output = match Command::new("ioreg")
        .args(["-c", "IOBlockStorageDriver", "-r", "-d", "1"])
        .output()
    {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => return snap,
    };

    let mut current_device = String::new();
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.contains("\"BSD Name\"")
            && let Some(name) = trimmed.split('"').nth(3)
        {
            // Normalize to base device (disk0s1 -> disk0)
            current_device = name
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() && !c.is_ascii_digit())
                .chain(
                    name.chars()
                        .skip_while(|c| !c.is_ascii_digit())
                        .take_while(|c| c.is_ascii_digit()),
                )
                .collect();
        }
        if trimmed.contains("\"Bytes (Read)\"")
            && let Some(val) = extract_ioreg_number(trimmed, "Bytes (Read)")
        {
            let entry = snap.entry(current_device.clone()).or_insert((0, 0));
            entry.0 = val;
        }
        if trimmed.contains("\"Bytes (Write)\"")
            && let Some(val) = extract_ioreg_number(trimmed, "Bytes (Write)")
        {
            let entry = snap.entry(current_device.clone()).or_insert((0, 0));
            entry.1 = val;
        }
    }
    snap
}

#[cfg(target_os = "macos")]
fn extract_ioreg_number(line: &str, key: &str) -> Option<u64> {
    let pattern = format!("\"{}\"=", key);
    if let Some(pos) = line.find(&pattern) {
        let after = &line[pos + pattern.len()..];
        let num_str: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
        num_str.parse().ok()
    } else {
        None
    }
}

#[cfg(target_os = "linux")]
fn read_io_counters() -> IoSnapshot {
    let mut snap = HashMap::new();
    if let Ok(content) = std::fs::read_to_string("/proc/diskstats") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // Format: major minor name rd_ios rd_merges rd_sectors rd_ticks
            //         wr_ios wr_merges wr_sectors wr_ticks ...
            if parts.len() >= 10 {
                let name = parts[2].to_string();
                let rd_sectors: u64 = parts[5].parse().unwrap_or(0);
                let wr_sectors: u64 = parts[9].parse().unwrap_or(0);
                // Sectors are typically 512 bytes
                snap.insert(name, (rd_sectors * 512, wr_sectors * 512));
            }
        }
    }
    snap
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn read_io_counters() -> IoSnapshot {
    HashMap::new()
}

fn compute_io_rates(
    prev: &IoSnapshot,
    curr: &IoSnapshot,
    elapsed_secs: f64,
    mount_dev: &HashMap<String, String>,
) -> IoRates {
    let mut rates = HashMap::new();
    if elapsed_secs <= 0.0 {
        return rates;
    }
    for (mount, device) in mount_dev {
        if let (Some(prev_io), Some(curr_io)) = (prev.get(device), curr.get(device)) {
            let rd = curr_io.0.saturating_sub(prev_io.0) as f64 / elapsed_secs;
            let wr = curr_io.1.saturating_sub(prev_io.1) as f64 / elapsed_secs;
            rates.insert(mount.clone(), (rd, wr));
        }
    }
    rates
}

fn apply_io_rates(entries: &mut [DiskEntry], rates: &IoRates) {
    for entry in entries.iter_mut() {
        if let Some(&(rd, wr)) = rates.get(&entry.mount) {
            entry.io_read_rate = Some(rd);
            entry.io_write_rate = Some(wr);
        }
    }
}

// ─── SMART health ─────────────────────────────────────────────────────────

type SmartMap = HashMap<String, SmartHealth>;

#[cfg(target_os = "macos")]
fn collect_smart_status(mount_dev: &HashMap<String, String>) -> SmartMap {
    use std::process::Command;
    let mut result = SmartMap::new();
    let mut checked: HashMap<String, SmartHealth> = HashMap::new();
    for (mount, base_dev) in mount_dev {
        if let Some(&status) = checked.get(base_dev) {
            result.insert(mount.clone(), status);
            continue;
        }
        let status = Command::new("diskutil")
            .args(["info", base_dev])
            .output()
            .ok()
            .and_then(|o| {
                let out = String::from_utf8_lossy(&o.stdout);
                for line in out.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("SMART Status:") {
                        let val = trimmed.trim_start_matches("SMART Status:").trim();
                        return match val {
                            "Verified" => Some(SmartHealth::Verified),
                            "Failing" => Some(SmartHealth::Failing),
                            _ => Some(SmartHealth::Unknown),
                        };
                    }
                }
                None
            })
            .unwrap_or(SmartHealth::Unknown);
        checked.insert(base_dev.clone(), status);
        result.insert(mount.clone(), status);
    }
    result
}

#[cfg(target_os = "linux")]
fn collect_smart_status(mount_dev: &HashMap<String, String>) -> SmartMap {
    let mut result = SmartMap::new();
    let mut checked: HashMap<String, SmartHealth> = HashMap::new();
    for (mount, base_dev) in mount_dev {
        if let Some(&status) = checked.get(base_dev) {
            result.insert(mount.clone(), status);
            continue;
        }
        // Try /sys/block/<dev>/device/state first (no root needed)
        let state_path = format!("/sys/block/{}/device/state", base_dev);
        let status = std::fs::read_to_string(&state_path)
            .ok()
            .map(|s| match s.trim() {
                "running" => SmartHealth::Verified,
                "offline" | "dead" | "blocked" => SmartHealth::Failing,
                _ => SmartHealth::Unknown,
            })
            .unwrap_or(SmartHealth::Unknown);
        checked.insert(base_dev.clone(), status);
        result.insert(mount.clone(), status);
    }
    result
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn collect_smart_status(_mount_dev: &HashMap<String, String>) -> SmartMap {
    SmartMap::new()
}

fn apply_smart_status(entries: &mut [DiskEntry], smart: &SmartMap) {
    for entry in entries.iter_mut() {
        if let Some(&status) = smart.get(&entry.mount) {
            entry.smart_status = Some(status);
        }
    }
}

pub fn spawn_bg_collector(shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>>) {
    std::thread::spawn(move || {
        let mut sys = System::new_all();
        let mut prev_io = read_io_counters();
        let mut prev_time = Instant::now();
        loop {
            std::thread::sleep(Duration::from_secs(3));
            sys.refresh_all();
            let stats = collect_sys_stats(&sys);
            let mut entries = collect_disk_entries();

            let curr_io = read_io_counters();
            let now = Instant::now();
            let elapsed = now.duration_since(prev_time).as_secs_f64();
            let mount_dev = mount_to_device_map();
            let rates = compute_io_rates(&prev_io, &curr_io, elapsed, &mount_dev);
            apply_io_rates(&mut entries, &rates);
            let smart = collect_smart_status(&mount_dev);
            apply_smart_status(&mut entries, &smart);
            prev_io = curr_io;
            prev_time = now;

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
        assert!((2024..=2100).contains(&year));
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
            assert!(
                d.pct >= 0.0 && d.pct <= 100.0,
                "Disk {} pct {} out of range",
                d.mount,
                d.pct
            );
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

    #[test]
    fn is_network_fs_detects_known_types() {
        assert!(is_network_fs("nfs"));
        assert!(is_network_fs("nfs4"));
        assert!(is_network_fs("cifs"));
        assert!(is_network_fs("smbfs"));
        assert!(is_network_fs("fuse.sshfs"));
        assert!(!is_network_fs("apfs"));
        assert!(!is_network_fs("ext4"));
        assert!(!is_network_fs("xfs"));
    }

    #[test]
    fn compute_io_rates_basic() {
        let mut prev = IoSnapshot::new();
        prev.insert("disk0".into(), (1000, 2000));
        let mut curr = IoSnapshot::new();
        curr.insert("disk0".into(), (2000, 4000));
        let mut mount_dev = HashMap::new();
        mount_dev.insert("/".into(), "disk0".into());
        let rates = compute_io_rates(&prev, &curr, 1.0, &mount_dev);
        let (rd, wr) = rates.get("/").unwrap();
        assert!((rd - 1000.0).abs() < f64::EPSILON);
        assert!((wr - 2000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn compute_io_rates_zero_elapsed() {
        let prev = IoSnapshot::new();
        let curr = IoSnapshot::new();
        let mount_dev = HashMap::new();
        let rates = compute_io_rates(&prev, &curr, 0.0, &mount_dev);
        assert!(rates.is_empty());
    }

    #[test]
    fn apply_io_rates_sets_fields() {
        let mut entries = vec![DiskEntry {
            mount: "/".into(),
            used: 0,
            total: 0,
            pct: 0.0,
            kind: DiskKind::Unknown(-1),
            fs: "apfs".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        }];
        let mut rates = IoRates::new();
        rates.insert("/".into(), (100.0, 200.0));
        apply_io_rates(&mut entries, &rates);
        assert!((entries[0].io_read_rate.unwrap() - 100.0).abs() < f64::EPSILON);
        assert!((entries[0].io_write_rate.unwrap() - 200.0).abs() < f64::EPSILON);
    }

    #[test]
    fn apply_smart_status_sets_fields() {
        let mut entries = vec![DiskEntry {
            mount: "/".into(),
            used: 0,
            total: 0,
            pct: 0.0,
            kind: DiskKind::Unknown(-1),
            fs: "apfs".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        }];
        let mut smart = SmartMap::new();
        smart.insert("/".into(), SmartHealth::Verified);
        apply_smart_status(&mut entries, &smart);
        assert_eq!(entries[0].smart_status, Some(SmartHealth::Verified));
    }

    #[test]
    fn mount_to_device_map_returns_map() {
        let map = mount_to_device_map();
        // On any real system with mounted disks, should have entries
        // (may be empty in unusual CI environments)
        if !map.is_empty() {
            for (mount, dev) in &map {
                assert!(!mount.is_empty());
                assert!(!dev.is_empty());
            }
        }
    }

    #[test]
    fn scan_directory_on_tmp() {
        let entries = scan_directory("/tmp");
        // /tmp should exist and be readable
        // entries may be empty but should not panic
        for e in &entries {
            assert!(!e.name.is_empty());
            assert!(!e.path.is_empty());
        }
    }
}
