//! `App::update_sorted` with an empty disk list (integration path).

use std::sync::{Arc, Mutex};

use storageshower::app::App;
use storageshower::types::{SortMode, SysStats};

#[test]
fn empty_disks_sorted_is_empty() {
    let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
    let mut app = App::new_default(shared);
    app.disks = vec![];
    app.prefs.sort_mode = SortMode::Pct;
    app.update_sorted();
    assert!(app.sorted_disks().is_empty());
}

#[test]
fn empty_disks_with_show_all_false_still_empty() {
    let shared = Arc::new(Mutex::new((SysStats::default(), vec![])));
    let mut app = App::new_default(shared);
    app.disks = vec![];
    app.prefs.show_all = false;
    app.update_sorted();
    assert!(app.sorted_disks().is_empty());
}
