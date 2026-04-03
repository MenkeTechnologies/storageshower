//! `ColorMode::next` and `ColorMode::ALL` invariants (integration crate boundary).

use std::collections::BTreeSet;

use clap::ValueEnum;
use storageshower::types::ColorMode;

#[test]
fn color_mode_all_matches_next_cycle_length() {
    let n = ColorMode::ALL.len();
    let mut m = ColorMode::ALL[0];
    for _ in 0..n {
        m = m.next();
    }
    assert_eq!(m, ColorMode::ALL[0]);
}

#[test]
fn each_next_differs_until_wrap() {
    let n = ColorMode::ALL.len();
    let mut m = ColorMode::ALL[0];
    for i in 1..n {
        let prev = m;
        m = m.next();
        assert_ne!(prev, m, "step {i}");
    }
}

#[test]
fn value_variants_len_matches_all() {
    assert_eq!(
        ColorMode::value_variants().len(),
        ColorMode::ALL.len(),
        "clap ValueEnum and ALL must stay in sync"
    );
}

#[test]
fn all_names_non_empty() {
    for &mode in ColorMode::ALL {
        assert!(!mode.name().is_empty(), "{mode:?}");
    }
}

#[test]
fn serde_json_roundtrip_each_variant() {
    for &mode in ColorMode::ALL {
        let s = serde_json::to_string(&mode).unwrap();
        let back: ColorMode = serde_json::from_str(&s).unwrap();
        assert_eq!(back, mode);
    }
}

#[test]
fn debug_strings_unique_in_all() {
    let mut seen = BTreeSet::new();
    for &m in ColorMode::ALL {
        let key = format!("{m:?}");
        assert!(seen.insert(key), "duplicate Debug for {m:?}");
    }
}
