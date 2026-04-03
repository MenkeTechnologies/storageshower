//! Public `storageshower::app::copy_to_clipboard` — succeeds when a host tool exists, otherwise returns a documented error.

use storageshower::app::copy_to_clipboard;

#[test]
fn copy_to_clipboard_ok_or_expected_err() {
    match copy_to_clipboard("storageshower-clipboard-integration-probe") {
        Ok(()) => {}
        Err(e) => {
            assert!(
                e.contains("clipboard") || e.contains("pbcopy") || e.contains("wl-copy"),
                "unexpected error message: {e}"
            );
        }
    }
}

#[test]
fn copy_to_clipboard_empty_string_does_not_panic() {
    let _ = copy_to_clipboard("");
}
