use clap::Parser;
use crossterm::event::{self, EnableMouseCapture, DisableMouseCapture, Event};
use ratatui::DefaultTerminal;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use sysinfo::System;

use storageshower::app::App;
use storageshower::cli::Cli;
use storageshower::system::{collect_disk_entries, collect_sys_stats, spawn_bg_collector};
use storageshower::types::{DiskEntry, SysStats};
use storageshower::ui::draw;

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let sys = System::new_all();
    let initial_stats = collect_sys_stats(&sys);
    let initial_disks = collect_disk_entries();
    drop(sys);

    let shared: Arc<Mutex<(SysStats, Vec<DiskEntry>)>> =
        Arc::new(Mutex::new((initial_stats, initial_disks)));

    spawn_bg_collector(Arc::clone(&shared));

    let mut terminal = ratatui::init();
    crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;
    let mut app = App::new(Arc::clone(&shared), &cli);
    let result = run_app(&mut terminal, &mut app);
    crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;
    ratatui::restore();
    app.save();
    result
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    let mut last_data_refresh = Instant::now();

    loop {
        let refresh_dur = Duration::from_secs(app.prefs.refresh_rate);
        if last_data_refresh.elapsed() >= refresh_dur {
            app.refresh_data();
            last_data_refresh = Instant::now();
        }

        terminal.draw(|f| draw(f, app))?;

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        app.handle_key(key);
                        if app.quit {
                            return Ok(());
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    let size = terminal.size()?;
                    app.handle_mouse(mouse, size.width);
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }
}
