//! `App::all_themes()` listing behavior when a user-defined custom theme key
//! collides with the lowercase Debug-name of a builtin `ColorMode`.
//!
//! Bug class this guards against: silent UI-level collision between the
//! `custom_themes` map key namespace and the lowercase `format!("{:?}", mode)`
//! key used for builtins. The theme chooser identifies a row by its first
//! tuple element ("key"); `apply_selected_theme` walks `ColorMode::ALL` first
//! and accepts the first match by lowercase Debug. A custom theme whose name
//! lowercase-matches a builtin (e.g. `"default"`, `"green"`, `"matrix"`,
//! `"neonnoir"`) is therefore shadowed at apply-time even though the chooser
//! row exists.
//!
//! These tests do NOT call `apply_selected_theme` (it is `pub(crate)`), but
//! they pin the public-API symptom: `all_themes()` happily emits TWO entries
//! with the same key when a collision is set up, which is the necessary
//! condition for the shadowing bug. If a future refactor changes that
//! (e.g. dedupes or namespaces the custom keys), these tests catch the
//! semantic change.

#![allow(clippy::field_reassign_with_default)]

use std::sync::{Arc, Mutex};

use storageshower::app::App;
use storageshower::types::{ColorMode, SysStats, ThemeColors};

fn make_app() -> App {
    let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
    App::new_default(shared)
}

fn solid_theme(v: u8) -> ThemeColors {
    ThemeColors {
        blue: v,
        green: v,
        purple: v,
        light_purple: v,
        royal: v,
        dark_purple: v,
    }
}

/// When a custom theme is keyed with the lowercase `Debug` spelling of a
/// builtin (`"default"` for `ColorMode::Default`), `all_themes()` returns
/// TWO entries with the same key string. The chooser would render both rows
/// but only the builtin can ever be applied (because `apply_selected_theme`
/// walks `ColorMode::ALL` first and matches by lowercase Debug).
#[test]
fn custom_theme_named_default_appears_twice_with_same_key() {
    let mut app = make_app();
    app.prefs
        .custom_themes
        .insert("default".into(), solid_theme(42));

    let themes = app.all_themes();
    let with_default_key: Vec<&(String, String)> =
        themes.iter().filter(|(k, _)| k == "default").collect();
    assert_eq!(
        with_default_key.len(),
        2,
        "expected the builtin row AND the custom row both to carry key \"default\""
    );

    // Built-in row keeps its display label "Neon Sprawl"; custom row carries
    // its own key as the display label. That's how the bug shows up to the
    // user — two visually distinct rows in the chooser collapsing to the same
    // dispatch key.
    let labels: Vec<&str> = with_default_key.iter().map(|(_, n)| n.as_str()).collect();
    assert!(labels.contains(&"Neon Sprawl"));
    assert!(labels.contains(&"default"));
}

/// Customs are appended after every builtin. A collision shows up as the
/// LAST entry having an earlier-indexed twin in the builtin block. This pins
/// the append-after-builtins ordering and the resulting twin offset.
#[test]
fn custom_collision_lands_at_tail_after_all_builtins() {
    let mut app = make_app();
    app.prefs
        .custom_themes
        .insert("matrix".into(), solid_theme(1));

    let themes = app.all_themes();
    assert_eq!(themes.len(), ColorMode::ALL.len() + 1);
    let last = themes.last().expect("non-empty");
    assert_eq!(last.0, "matrix");
    // The builtin twin sits somewhere strictly before the tail.
    let builtin_twin_idx = themes
        .iter()
        .take(themes.len() - 1)
        .position(|(k, _)| k == "matrix");
    assert!(
        builtin_twin_idx.is_some(),
        "lowercase Debug of ColorMode::Matrix should also produce key \"matrix\""
    );
}

/// Sanity guard: a custom theme whose key does NOT collide is the unique row
/// for that key. The bug above is conditional on key choice, not unconditional.
#[test]
fn unique_custom_key_produces_unique_row() {
    let mut app = make_app();
    app.prefs
        .custom_themes
        .insert("zzz_unique_xyz".into(), solid_theme(7));

    let themes = app.all_themes();
    let matches: Vec<&(String, String)> = themes
        .iter()
        .filter(|(k, _)| k == "zzz_unique_xyz")
        .collect();
    assert_eq!(matches.len(), 1, "non-colliding custom must be unique");
}

/// Every builtin's lowercase Debug name is a forbidden zone for custom keys
/// if the bug is to be avoided. Pin the full set so a future ColorMode
/// variant addition is forced to consider this hazard.
#[test]
fn lowercase_debug_keys_for_every_builtin_are_distinct() {
    use std::collections::BTreeSet;
    let mut keys: BTreeSet<String> = BTreeSet::new();
    for &mode in ColorMode::ALL {
        let key = format!("{:?}", mode).to_lowercase();
        assert!(
            keys.insert(key.clone()),
            "two builtins lowercase-collide: {key}"
        );
    }
    assert_eq!(keys.len(), ColorMode::ALL.len());
}
