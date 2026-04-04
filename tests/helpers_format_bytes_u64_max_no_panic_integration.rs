//! `format_bytes` must not panic on `u64::MAX` for each `UnitMode`.

use storageshower::helpers::format_bytes;
use storageshower::types::UnitMode;

#[test]
fn u64_max_all_modes() {
    let n = u64::MAX;
    for mode in [
        UnitMode::Human,
        UnitMode::Bytes,
        UnitMode::GiB,
        UnitMode::MiB,
    ] {
        let s = format_bytes(n, mode);
        assert!(!s.is_empty(), "mode={mode:?}");
    }
}
