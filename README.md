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

```
╔═══════════════════════════════════════════════════════════════╗
║ ▶▶▶  DISK MATRIX  ◀◀◀  node:HOST  date:2026.03.26  clock:…  ║
╠═══════════════════════════════════════════════════════════════╣
║   MOUNT▲        │ USAGE                    │  PCT  USED/SIZE  ║
╠═══════════════════════════════════════════════════════════════╣
║ ◈ /             │ ████▓▓▒▒░░▸              │  45%  200G/500G  ║
║ ⚠ /home         │ ██████▓▓▒▒▒▒░░░▸         │  78%  195G/250G  ║
║ ✖ /var          │ █████████████████████▸    │  95%   45G/ 50G  ║
╠═══════════════════════════════════════════════════════════════╣
║ ⟦zpwr⊷cyberdeck⟧ ◀◀◀ ▣3vol │ sort:name▲ │ 1s │ gradient …  ║
╚═══════════════════════════════════════════════════════════════╝
```

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
| `q` | Disconnect (or close help overlay) |
| `h` | Toggle help HUD |
| `p` | Pause / resume data stream |
| `l` | Local disks only |
| `a` | Show all filesystems (incl. virtual) |
| `r` | Reverse sort vector |
| `/` | Filter mounts by text input |
| `0` | Purge filter |

#### `// SORT_PROTOCOL`

| `KEY` | `ACTION` |
|:---:|:---|
| `n` | Sort by mount name (again to reverse) |
| `u` | Sort by usage % |
| `s` | Sort by size |

#### `// DISPLAY_MODS`

| `KEY` | `ACTION` |
|:---:|:---|
| `b` | Cycle bar style — gradient / solid / thin / ascii |
| `c` | Cycle color theme — default / green / blue / purple |
| `v` | Toggle usage bars |
| `d` | Toggle used/size columns |
| `g` | Toggle column headers |
| `x` | Toggle border chrome |
| `m` | Compact mount names (16 chars) |
| `w` | Full mount paths |
| `i` | Cycle units — human / GiB / MiB / bytes |
| `f` | Cycle refresh rate — 1s / 2s / 5s / 10s |
| `t` | Cycle warn threshold — 50 / 60 / 70 / 80% |
| `T` | Cycle crit threshold — 80 / 85 / 90 / 95% |

---

### `> CONFIG_PERSISTENCE.log`

```
 ┌──────────────────────────────────────────────────┐
 │  ALL PREFS AUTO-SAVED TO ~/.storageshower.conf   │
 │  FORMAT: TOML ── RESTORED ON BOOT ── ZERO EDIT   │
 │                                                    │
 │  >> sort mode    >> sort direction   >> filter     │
 │  >> refresh rate >> bar style        >> color mode  │
 │  >> warn/crit    >> bar visibility   >> border      │
 │  >> col headers  >> compact mode     >> mount paths │
 └──────────────────────────────────────────────────┘
```

---

<p align="center">
  <code>⟦ END OF LINE ⟧</code><br>
  <code>// THE STREET FINDS ITS OWN USES FOR DISK SPACE //</code>
</p>
