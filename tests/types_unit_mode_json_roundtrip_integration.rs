//! `serde_json` roundtrip for all `UnitMode` variants.

use storageshower::types::UnitMode;

#[test]
fn all_unit_modes_json_roundtrip() {
    for mode in [
        UnitMode::Human,
        UnitMode::GiB,
        UnitMode::MiB,
        UnitMode::Bytes,
    ] {
        let s = serde_json::to_string(&mode).unwrap();
        let back: UnitMode = serde_json::from_str(&s).unwrap();
        assert_eq!(back, mode, "json={s}");
    }
}
