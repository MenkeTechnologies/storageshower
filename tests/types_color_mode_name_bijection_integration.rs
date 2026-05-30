//! ColorMode helper contracts not pinned by the existing cycle/json tests:
//!
//! - Every `.name()` string is unique within ColorMode::ALL — a duplicate
//!   surfaces as two indistinguishable theme entries in the picker UI.
//! - `.name()` strings stay ASCII-printable and ≤ 16 chars so they fit the
//!   theme-picker column without truncation (matches truncate_mount budget).
//! - `.next()` is a bijection over ColorMode::ALL — every variant is the
//!   `.next()` of exactly one predecessor, so cycling through `n` steps
//!   visits every variant exactly once and never loops short.
//! - `.name()` is stable across re-invocation (no interior mutability).

use std::collections::HashSet;

use storageshower::types::ColorMode;

#[test]
fn every_color_mode_name_is_unique() {
    let mut seen: HashSet<&'static str> = HashSet::new();
    for &mode in ColorMode::ALL {
        let n = mode.name();
        assert!(
            seen.insert(n),
            "duplicate ColorMode::name() {:?} on variant {:?}",
            n,
            mode
        );
    }
}

#[test]
fn every_color_mode_name_is_ascii_printable_and_short() {
    // The theme picker column has a hard width budget (~16 chars). Pin both
    // ASCII-ness (so it renders identically on every terminal codepage) and
    // a soft cap so a future "Synthwave Rider" longer than 16 trips here
    // instead of silently truncating in the UI.
    for &mode in ColorMode::ALL {
        let n = mode.name();
        assert!(
            n.chars().all(|c| c.is_ascii_graphic() || c == ' '),
            "name {:?} for {:?} contains non-ASCII-printable",
            n,
            mode
        );
        assert!(
            n.len() <= 16,
            "name {:?} for {:?} is {} chars (cap 16)",
            n,
            mode,
            n.len()
        );
    }
}

#[test]
fn next_is_a_bijection_over_all() {
    // Every variant must appear as the .next() of exactly one predecessor.
    // If any variant has zero predecessors, the cycle has a fork; if any
    // has two, two different variants map to the same successor and the
    // cycle has a merge — either case breaks the "press T to cycle" UX.
    let mut succ_counts = vec![0usize; ColorMode::ALL.len()];
    for &m in ColorMode::ALL {
        let s = m.next();
        let idx = ColorMode::ALL
            .iter()
            .position(|&v| v == s)
            .expect(".next() outside ALL");
        succ_counts[idx] += 1;
    }
    for (idx, &count) in succ_counts.iter().enumerate() {
        assert_eq!(
            count, 1,
            "variant {:?} has {} predecessors (expected exactly 1)",
            ColorMode::ALL[idx], count
        );
    }
}

#[test]
fn n_steps_of_next_visits_every_variant_exactly_once() {
    // Stronger than the existing "wraps around" test: pin that the orbit
    // from ColorMode::ALL[0] passes through every variant in ALL exactly
    // once before wrapping. Guards against a future refactor where two
    // sub-cycles are accidentally introduced. Use a parallel bitmap over
    // ALL indices since ColorMode doesn't impl Hash.
    let n = ColorMode::ALL.len();
    let mut visited = vec![false; n];
    let mut m = ColorMode::ALL[0];
    for step in 0..n {
        let idx = ColorMode::ALL
            .iter()
            .position(|&v| v == m)
            .expect("ColorMode out of ALL");
        assert!(!visited[idx], "cycle revisits {:?} at step {}", m, step);
        visited[idx] = true;
        m = m.next();
    }
    assert!(
        visited.iter().all(|&b| b),
        "didn't visit every ColorMode variant"
    );
}

#[test]
fn name_is_stable_across_invocations() {
    // Pin: `.name()` returns a `&'static str` and must not vary between
    // calls. Catches a future move to `String::from(...)` that would
    // accidentally lose the const-time guarantee.
    for &mode in ColorMode::ALL {
        let a = mode.name();
        let b = mode.name();
        assert_eq!(a.as_ptr(), b.as_ptr(), "name pointer drifted for {:?}", mode);
    }
}
