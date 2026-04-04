//! `serde_json` roundtrip for `SortMode`.

use storageshower::types::SortMode;

#[test]
fn sort_mode_all_variants_roundtrip() {
    for mode in [SortMode::Name, SortMode::Pct, SortMode::Size] {
        let s = serde_json::to_string(&mode).unwrap();
        let back: SortMode = serde_json::from_str(&s).unwrap();
        assert_eq!(back, mode);
    }
}
