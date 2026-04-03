//! Integration coverage for core `types` enums and structs used across the crate boundary.

use storageshower::types::{
    BarStyle, ColorMode, DirEntry, DragTarget, DrillSortMode, HoverZone, SmartHealth, SortMode,
    SysStats, UnitMode, ViewMode,
};
use sysinfo::DiskKind;

#[test]
fn sort_mode_exhaustive_ne_pairs() {
    assert_ne!(SortMode::Name, SortMode::Pct);
    assert_ne!(SortMode::Pct, SortMode::Size);
    assert_ne!(SortMode::Size, SortMode::Name);
}

#[test]
fn bar_style_four_variants_distinct() {
    let v = [
        BarStyle::Gradient,
        BarStyle::Solid,
        BarStyle::Thin,
        BarStyle::Ascii,
    ];
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            assert_ne!(v[i], v[j], "BarStyle duplicate at {i},{j}");
        }
    }
}

#[test]
fn unit_mode_four_variants_distinct() {
    assert_ne!(UnitMode::Human, UnitMode::GiB);
    assert_ne!(UnitMode::GiB, UnitMode::MiB);
    assert_ne!(UnitMode::MiB, UnitMode::Bytes);
}

#[test]
fn drill_sort_mode_both_variants() {
    assert_ne!(DrillSortMode::Name, DrillSortMode::Size);
}

#[test]
fn view_mode_both_variants() {
    assert_ne!(ViewMode::Disks, ViewMode::DrillDown);
}

#[test]
fn hover_zone_variants_disk_row_index() {
    assert_eq!(HoverZone::DiskRow(0), HoverZone::DiskRow(0));
    assert_ne!(HoverZone::DiskRow(0), HoverZone::DiskRow(1));
    assert_ne!(HoverZone::None, HoverZone::TitleBar);
    assert_ne!(HoverZone::TitleBar, HoverZone::FooterBar);
}

#[test]
fn smart_health_three_way_ne() {
    assert_ne!(SmartHealth::Verified, SmartHealth::Failing);
    assert_ne!(SmartHealth::Failing, SmartHealth::Unknown);
    assert_ne!(SmartHealth::Unknown, SmartHealth::Verified);
}

#[test]
fn drag_target_variants_are_constructible() {
    let _ = DragTarget::MountSep;
    let _ = DragTarget::BarEndSep;
    let _ = DragTarget::PctSep;
}

#[test]
fn dir_entry_clone_preserves_fields() {
    let d = DirEntry {
        path: "/x/y".into(),
        name: "y".into(),
        size: 999,
        is_dir: false,
    };
    let c = d.clone();
    assert_eq!(c.path, d.path);
    assert_eq!(c.size, 999);
    assert!(!c.is_dir);
}

#[test]
fn sys_stats_default_mem_total_nonzero() {
    let s = SysStats::default();
    assert_eq!(s.mem_total, 1);
}

#[test]
fn disk_kind_unknown_roundtrip_debug() {
    let k = DiskKind::Unknown(42);
    let s = format!("{k:?}");
    assert!(!s.is_empty());
}

#[test]
fn color_mode_all_starts_with_default() {
    assert_eq!(ColorMode::ALL[0], ColorMode::Default);
}

#[test]
fn hover_zone_debug_contains_variant_names() {
    assert!(
        format!("{:?}", HoverZone::TitleBar)
            .to_lowercase()
            .contains("title")
    );
    assert!(
        format!("{:?}", HoverZone::FooterBar)
            .to_lowercase()
            .contains("footer")
    );
}

#[test]
fn view_mode_debug_nonempty() {
    assert!(!format!("{:?}", ViewMode::Disks).is_empty());
}

#[test]
fn drill_sort_mode_debug_nonempty() {
    assert!(!format!("{:?}", DrillSortMode::Name).is_empty());
}
