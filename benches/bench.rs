use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use storageshower::app::{mount_col_width, right_col_width_static};
use storageshower::helpers::{format_bytes, format_uptime, truncate_mount};
use storageshower::prefs::Prefs;
use storageshower::system::{chrono_now, collect_disk_entries, collect_sys_stats, epoch_to_local};
use storageshower::types::*;
use storageshower::ui::{gradient_color_at, palette};
use sysinfo::{DiskKind, System};

// ─── Test data ──────────────────────────────────────────────────────────────

fn sample_disks(n: usize) -> Vec<DiskEntry> {
    (0..n)
        .map(|i| DiskEntry {
            mount: format!("/mnt/disk{i}"),
            used: (i as u64 + 1) * 107_374_182_400,   // ~100G increments
            total: (i as u64 + 2) * 107_374_182_400,
            pct: (i as f64 + 1.0) / (i as f64 + 2.0) * 100.0,
            kind: DiskKind::SSD,
            fs: "apfs".into(),
            latency_ms: None,
            io_read_rate: None,
            io_write_rate: None,
            smart_status: None,
        })
        .collect()
}

fn default_prefs() -> Prefs {
    Prefs::default()
}

// ─── Formatting ─────────────────────────────────────────────────────────────

fn bench_format_bytes(c: &mut Criterion) {
    let sizes: &[(u64, &str)] = &[
        (0, "zero"),
        (512, "512B"),
        (1_048_576, "1M"),
        (1_073_741_824, "1G"),
        (1_099_511_627_776, "1T"),
        (5_497_558_138_880, "5T"),
    ];
    let modes = [UnitMode::Human, UnitMode::GiB, UnitMode::MiB, UnitMode::Bytes];

    let mut group = c.benchmark_group("format_bytes");
    for (val, label) in sizes {
        for mode in &modes {
            group.bench_with_input(
                BenchmarkId::new(format!("{label}/{mode:?}"), val),
                val,
                |b, &v| b.iter(|| format_bytes(black_box(v), black_box(*mode))),
            );
        }
    }
    group.finish();
}

fn bench_format_uptime(c: &mut Criterion) {
    let durations: &[(u64, &str)] = &[
        (45 * 60, "45m"),
        (5 * 3600 + 30 * 60, "5h30m"),
        (2 * 86400 + 14 * 3600 + 3 * 60, "2d14h3m"),
        (365 * 86400, "365d"),
    ];

    let mut group = c.benchmark_group("format_uptime");
    for (secs, label) in durations {
        group.bench_with_input(
            BenchmarkId::new(*label, secs),
            secs,
            |b, &s| b.iter(|| format_uptime(black_box(s))),
        );
    }
    group.finish();
}

// ─── Layout ─────────────────────────────────────────────────────────────────

fn bench_truncate_mount(c: &mut Criterion) {
    let mounts = [
        "/",
        "/home",
        "/var/lib/docker/overlay2/abc123",
        "/System/Volumes/Data",
    ];

    let mut group = c.benchmark_group("truncate_mount");
    for mount in &mounts {
        for width in [8, 16, 32] {
            group.bench_with_input(
                BenchmarkId::new(format!("w{width}"), mount),
                &(*mount, width),
                |b, &(m, w)| b.iter(|| truncate_mount(black_box(m), black_box(w))),
            );
        }
    }
    group.finish();
}

fn bench_mount_col_width(c: &mut Criterion) {
    let mut group = c.benchmark_group("mount_col_width");
    let prefs = default_prefs();
    for inner_w in [60, 120, 200] {
        group.bench_with_input(
            BenchmarkId::from_parameter(inner_w),
            &inner_w,
            |b, &w| b.iter(|| mount_col_width(black_box(w), black_box(&prefs))),
        );
    }

    let mut compact_prefs = default_prefs();
    compact_prefs.compact = true;
    group.bench_function("compact", |b| {
        b.iter(|| mount_col_width(black_box(120), black_box(&compact_prefs)))
    });

    let mut custom_prefs = default_prefs();
    custom_prefs.col_mount_w = 20;
    group.bench_function("custom_w20", |b| {
        b.iter(|| mount_col_width(black_box(120), black_box(&custom_prefs)))
    });
    group.finish();
}

fn bench_right_col_width_static(c: &mut Criterion) {
    let mut group = c.benchmark_group("right_col_width_static");
    let prefs = default_prefs();
    group.bench_function("show_used", |b| {
        b.iter(|| right_col_width_static(black_box(&prefs)))
    });

    let mut no_used = default_prefs();
    no_used.show_used = false;
    group.bench_function("no_used", |b| {
        b.iter(|| right_col_width_static(black_box(&no_used)))
    });
    group.finish();
}

// ─── Color / theme ──────────────────────────────────────────────────────────

fn bench_palette(c: &mut Criterion) {
    let modes = [ColorMode::Default, ColorMode::Green, ColorMode::Blue, ColorMode::Purple];
    let mut group = c.benchmark_group("palette");
    for mode in modes {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{mode:?}")),
            &mode,
            |b, &m| b.iter(|| palette(black_box(m))),
        );
    }
    group.finish();
}

fn bench_gradient_color_at(c: &mut Criterion) {
    let fracs = [0.0, 0.2, 0.4, 0.6, 0.85, 1.0];
    let mut group = c.benchmark_group("gradient_color_at");
    for frac in fracs {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{frac:.2}")),
            &frac,
            |b, &f| b.iter(|| gradient_color_at(black_box(f), black_box(ColorMode::Default))),
        );
    }
    group.finish();
}

// ─── Time ───────────────────────────────────────────────────────────────────

fn bench_epoch_to_local(c: &mut Criterion) {
    let epochs: &[(i64, &str)] = &[
        (0, "unix_epoch"),
        (1_711_500_000, "2024"),
        (1_774_000_000, "2026"),
    ];
    let mut group = c.benchmark_group("epoch_to_local");
    for (epoch, label) in epochs {
        group.bench_with_input(
            BenchmarkId::new(*label, epoch),
            epoch,
            |b, &e| b.iter(|| epoch_to_local(black_box(e))),
        );
    }
    group.finish();
}

fn bench_chrono_now(c: &mut Criterion) {
    c.bench_function("chrono_now", |b| b.iter(chrono_now));
}

// ─── Data collection (I/O-bound, measured separately) ───────────────────────

fn bench_collect_disk_entries(c: &mut Criterion) {
    let mut group = c.benchmark_group("collect");
    group.sample_size(20);
    group.bench_function("disk_entries", |b| b.iter(collect_disk_entries));
    group.finish();
}

fn bench_collect_sys_stats(c: &mut Criterion) {
    let sys = System::new_all();
    let mut group = c.benchmark_group("collect");
    group.sample_size(20);
    group.bench_function("sys_stats", |b| {
        b.iter(|| collect_sys_stats(black_box(&sys)))
    });
    group.finish();
}

// ─── Config serialization ───────────────────────────────────────────────────

fn bench_prefs_serde(c: &mut Criterion) {
    let prefs = default_prefs();
    let toml_str = toml::to_string_pretty(&prefs).unwrap();

    let mut group = c.benchmark_group("prefs_serde");
    group.bench_function("serialize", |b| {
        b.iter(|| toml::to_string_pretty(black_box(&prefs)).unwrap())
    });
    group.bench_function("deserialize", |b| {
        b.iter(|| toml::from_str::<Prefs>(black_box(&toml_str)).unwrap())
    });
    group.finish();
}

// ─── Sorting (simulates sorted_disks hot path) ─────────────────────────────

fn bench_sort_disks(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_disks");
    for count in [10, 50, 200] {
        let disks = sample_disks(count);

        group.bench_with_input(
            BenchmarkId::new("by_name", count),
            &disks,
            |b, ds| {
                b.iter(|| {
                    let mut v = ds.clone();
                    v.sort_by(|a, b| a.mount.cmp(&b.mount));
                    black_box(v);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("by_pct", count),
            &disks,
            |b, ds| {
                b.iter(|| {
                    let mut v = ds.clone();
                    v.sort_by(|a, b| {
                        a.pct.partial_cmp(&b.pct).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    black_box(v);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("by_size", count),
            &disks,
            |b, ds| {
                b.iter(|| {
                    let mut v = ds.clone();
                    v.sort_by(|a, b| a.total.cmp(&b.total));
                    black_box(v);
                })
            },
        );
    }
    group.finish();
}

// ─── Filter (simulates filter substring match) ─────────────────────────────

fn bench_filter_disks(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter_disks");
    for count in [10, 50, 200] {
        let disks = sample_disks(count);

        group.bench_with_input(
            BenchmarkId::new("substring_match", count),
            &disks,
            |b, ds| {
                let filter = "disk1";
                b.iter(|| {
                    let f = filter.to_lowercase();
                    let v: Vec<_> = ds
                        .iter()
                        .filter(|d| d.mount.to_lowercase().contains(&f))
                        .collect();
                    black_box(v);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("no_match", count),
            &disks,
            |b, ds| {
                let filter = "zzzznotfound";
                b.iter(|| {
                    let f = filter.to_lowercase();
                    let v: Vec<_> = ds
                        .iter()
                        .filter(|d| d.mount.to_lowercase().contains(&f))
                        .collect();
                    black_box(v);
                })
            },
        );
    }
    group.finish();
}

// ─── Render pipeline (format_bytes across all disks) ────────────────────────

fn bench_format_all_disks(c: &mut Criterion) {
    let mut group = c.benchmark_group("format_all_disks");
    for count in [10, 50, 200] {
        let disks = sample_disks(count);
        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            &disks,
            |b, ds| {
                b.iter(|| {
                    for d in ds {
                        black_box(format_bytes(d.used, UnitMode::Human));
                        black_box(format_bytes(d.total, UnitMode::Human));
                    }
                })
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_format_bytes,
    bench_format_uptime,
    bench_truncate_mount,
    bench_mount_col_width,
    bench_right_col_width_static,
    bench_palette,
    bench_gradient_color_at,
    bench_epoch_to_local,
    bench_chrono_now,
    bench_collect_disk_entries,
    bench_collect_sys_stats,
    bench_prefs_serde,
    bench_sort_disks,
    bench_filter_disks,
    bench_format_all_disks,
);
criterion_main!(benches);
