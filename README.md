```
 ██████╗████████╗ ██████╗ ██████╗  █████╗  ██████╗ ███████╗
██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██╔══██╗██╔════╝ ██╔════╝
╚█████╗    ██║   ██║   ██║██████╔╝███████║██║  ███╗█████╗
 ╚═══██╗   ██║   ██║   ██║██╔══██╗██╔══██║██║   ██║██╔══╝
██████╔╝   ██║   ╚██████╔╝██║  ██║██║  ██║╚██████╔╝███████╗
╚═════╝    ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝
███████╗██╗  ██╗ ██████╗ ██╗    ██╗███████╗██████╗
██╔════╝██║  ██║██╔═══██╗██║    ██║██╔════╝██╔══██╗
╚█████╗ ███████║██║   ██║██║ █╗ ██║█████╗  ██████╔╝
 ╚═══██╗██╔══██║██║   ██║██║███╗██║██╔══╝  ██╔══██╗
███████║██║  ██║╚██████╔╝╚███╔███╔╝███████╗██║  ██║
╚══════╝╚═╝  ╚═╝ ╚═════╝  ╚══╝╚══╝ ╚══════╝╚═╝  ╚═╝
```

<p align="center">
  <code>[ SYSTEM://DISK_MATRIX v2.0 ]</code><br>
  <code>⟦ JACKING INTO YOUR FILESYSTEM ⟧</code><br><br>
  <strong>A neon-drenched terminal UI for monitoring disk usage</strong><br>
  <em>Built in Rust with <a href="https://github.com/ratatui/ratatui">ratatui</a> + <a href="https://github.com/crossterm-rs/crossterm">crossterm</a></em><br><br>
  <code>created by MenkeTechnologies</code>
</p>

---

```
 ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
 █ >> INITIALIZING FEATURE MATRIX...                    █
 █ >> STATUS: ALL SYSTEMS NOMINAL                       █
 ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
```

### `> FEATURE_DUMP.exe`

```
[RENDER_ENGINE]
  ├── Live disk usage display ─── color-coded progress bars
  │   ├── gradient ████▓▓▒▒░░
  │   ├── solid   █████████
  │   ├── thin    ▬▬▬▬▬▬▬▬▬
  │   └── ascii   #########
  │
[TELEMETRY_CORE]
  ├── Real-time system stats ─── load avg / memory / CPU
  │   ├── swap / process count / uptime
  │   ├── network IP / battery / TTY
  │   └── background thread @ 3s via Arc<Mutex<>>
  │
[ALERT_SUBSYSTEM]
  ├── Threshold alerts
  │   ├── ◈ NOMINAL ── all clear, choomba
  │   ├── ⚠ WARNING ── approaching redline
  │   └── ✖ CRITICAL ── flatlined
  │
[INTERFACE_DECK]
  ├── Sort ─── name / usage% / size / asc / desc
  ├── Filter ─── case-insensitive substring match
  ├── Units ─── human / GiB / MiB / raw bytes
  ├── Themes ─── default / green / blue / purple
  └── Persistent config ─── ~/.storageshower.conf (TOML)

[PLATFORM_COMPAT]
  ├── macOS ── SUPPORTED
  ├── Linux ── SUPPORTED
  └── auto-detects battery, memory, TTY, local IP
```

---

### `> RENDER_PREVIEW.dat`

#### `// DEFAULT_THEME`

<p align="center">
  <img src="screenshots/main-view.png" alt="Main View — Default Theme" width="800">
</p>

#### `// GREEN_THEME`

<p align="center">
  <img src="screenshots/green-theme.png" alt="Green Theme" width="800">
</p>

#### `// CLI_HELP`

<p align="center">
  <img src="screenshots/cli-help.png" alt="CLI Help — storageshower -h" width="800">
</p>

#### `// HELP_OVERLAY`

<p align="center">
  <img src="screenshots/help-view.png" alt="Help Overlay" width="800">
</p>

---

### `> REQUIRED_IMPLANTS.cfg`

```
RUST_VERSION  >= 1.70  [2021 edition]
TARGET_OS     == macOS || Linux
```

| `IMPLANT` | `PURPOSE` |
|:---:|:---|
| `ratatui` 0.29 | TUI rendering framework |
| `crossterm` 0.28 | Terminal events + manipulation |
| `sysinfo` 0.32 | Disk / memory / CPU / proc intel |
| `dirs` 5 | Home directory detection |
| `serde` 1 | Config serialization |
| `toml` 0.8 | Config file format |
| `libc` 0.2 | Unix syscalls (time, TTY) |

---

### `> COMPILE_SEQUENCE.sh`

```bash
# ── JACK IN ──────────────────────────────────
cargo build --release
# LTO enabled ── symbols stripped ── lean binary
```

```bash
# ── BOOT THE MATRIX ─────────────────────────
cargo run --release
# or go direct:
./target/release/storageshower
```

---

### `> KEYBIND_MATRIX.dat`

```
 ┌──────────────────────────────────────────────────┐
 │           ◈◈◈  COMMAND INTERFACE  ◈◈◈            │
 └──────────────────────────────────────────────────┘
```

#### `// GENERAL_OPS`

| `KEY` | `ACTION` |
|:---:|:---|
| `q` `Q` | Disconnect (or close help overlay) |
| `h` `H` `?` | Toggle help HUD |
| `p` `P` | Pause / resume data stream |
| `Esc` | Deselect current disk |

#### `// NAVIGATION`

| `KEY` | `ACTION` |
|:---:|:---|
| `j` `Down` | Select next disk |
| `k` `Up` | Select previous disk |
| `G` `End` | Jump to last disk |
| `Home` `Ctrl+g` | Jump to first disk |
| `Ctrl+d` | Half-page down |
| `Ctrl+u` | Half-page up |

#### `// SORT_PROTOCOL`

| `KEY` | `ACTION` |
|:---:|:---|
| `n` `N` | Sort by mount name (again to reverse) |
| `u` `U` | Sort by usage % (again to reverse) |
| `s` `S` | Sort by size (again to reverse) |
| `r` `R` | Reverse sort vector |

#### `// DISPLAY_MODS`

| `KEY` | `ACTION` |
|:---:|:---|
| `b` | Cycle bar style — gradient / solid / thin / ascii |
| `c` | Cycle color theme — default / green / blue / purple |
| `v` `V` | Toggle usage bars |
| `d` `D` | Toggle used/size columns |
| `g` | Toggle column headers |
| `x` `X` | Toggle border chrome |
| `m` `M` | Compact mount names |
| `w` `W` | Full mount paths |
| `i` `I` | Cycle units — human / GiB / MiB / bytes |
| `f` `F` | Cycle refresh rate — 1s / 2s / 5s / 10s |
| `t` | Cycle warn threshold — 50 / 60 / 70 / 80% |
| `T` | Cycle crit threshold — 80 / 85 / 90 / 95% |

#### `// FILTER_OPS`

| `KEY` | `ACTION` |
|:---:|:---|
| `l` `L` | Local disks only |
| `a` `A` | Show all filesystems (incl. virtual) |
| `/` | Enter filter mode |
| `0` | Purge filter |

#### `// FILTER_EDIT_MODE`

| `KEY` | `ACTION` |
|:---:|:---|
| `Enter` | Confirm filter |
| `Esc` | Cancel filter |
| `Backspace` `Ctrl+h` | Delete char before cursor |
| `Delete` | Delete char at cursor |
| `Ctrl+w` | Delete word backward |
| `Ctrl+u` | Clear line before cursor |
| `Ctrl+k` | Delete to end of line |
| `Ctrl+a` `Home` | Cursor to start |
| `Ctrl+e` `End` | Cursor to end |
| `Ctrl+b` `Left` | Cursor left |
| `Ctrl+f` `Right` | Cursor right |

#### `// DISK_OPS`

| `KEY` | `ACTION` |
|:---:|:---|
| `Enter` | Open selected mount in file manager |
| `y` `Y` | Copy mount path to clipboard |
| `e` `E` | Export disk matrix to file |

#### `// MOUSE_INPUT`

| `ACTION` | `EFFECT` |
|:---:|:---|
| `Left-drag` column separator | Resize mount / right column |
| `Right-click` | Toggle help overlay |

---

### `> CONFIG_PERSISTENCE.log`

```
 ┌──────────────────────────────────────────────────────┐
 │  ALL PREFS AUTO-SAVED TO ~/.storageshower.conf       │
 │  FORMAT: TOML ── RESTORED ON BOOT ── ZERO EDIT       │
 │                                                        │
 │  >> sort mode    >> sort direction   >> show all       │
 │  >> refresh rate >> bar style        >> color mode      │
 │  >> warn/crit    >> bar visibility   >> border          │
 │  >> col headers  >> compact mode     >> mount paths     │
 │  >> show used    >> show local       >> custom widths   │
 │  >> mount col w  >> right col w      >> pct col w       │
 └──────────────────────────────────────────────────────┘
```

---

<p align="center">
  <code>⟦ END OF LINE ⟧</code><br>
  <code>// THE STREET FINDS ITS OWN USES FOR DISK SPACE //</code>
</p>
