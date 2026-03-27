# StorageShower Cyberpunk

A cyberpunk-themed terminal UI (TUI) for monitoring disk usage, built in pure Bash. Renders a full-screen, auto-refreshing dashboard with gradient usage bars, live system stats, and extensive keyboard-driven customization.

Created by MenkeTechnologies

## Features

- **Live disk usage display** with color-coded progress bars (gradient, solid, thin, or ASCII styles)
- **Real-time system stats** — load average, memory, CPU count, swap, process count, uptime, network IP, battery, git branch, and more
- **Threshold alerts** — configurable warning and critical thresholds with visual indicators (◈ normal, ⚠ warning, ✖ critical)
- **Sorting** — by mount name, usage percentage, or size; ascending or descending
- **Filtering** — grep-style filter on mount paths, toggle local-only (`/dev/*`) mounts
- **Multiple display units** — human-readable, 1K-blocks, or 1M-blocks
- **4 color themes** — default, green, blue, purple
- **Persistent preferences** — settings saved to `~/.storageshower.conf` and restored on next launch
- **Background stats collection** — system metrics gathered asynchronously to keep the UI responsive
- **Adaptive layout** — stat segments appear/hide based on terminal width; handles resize via `SIGWINCH`

## Requirements

- Bash 4+
- Standard Unix utilities: `df`, `awk`, `sort`, `stty`, `tput`
- macOS or Linux (auto-detects `vm_stat` vs `/proc/meminfo`, `pmset` vs sysfs for battery, etc.)

## Usage

```bash
# Run directly
./storageshowerCyberpunk.sh

# Or via the wrapper (passes args through)
./storageShower.sh [options] [lineCount] [RefreshTime(sec)]
```

### Options

| Flag | Description |
|------|-------------|
| `-h` | Display help |
| `-v` | Display version |

## Keyboard Shortcuts

| Key | Action | Key | Action |
|-----|--------|-----|--------|
| `q` | Quit (or close help) | `h` | Toggle help overlay |
| `p` | Pause / resume | `l` | Local disks only |
| `r` | Reverse sort order | `/` | Filter mounts by text |
| `0` | Clear filter | | |
| **Sort** | | | |
| `n` | Sort by mount name | `u` | Sort by usage % |
| `s` | Sort by size | | |
| **Display** | | | |
| `b` | Cycle bar style (gradient / solid / thin / ascii) | `c` | Cycle color theme |
| `v` | Toggle bars | `d` | Toggle used/size columns |
| `g` | Toggle column headers | `x` | Toggle border |
| `m` | Compact mount names | `w` | Full mount paths |
| `i` | Cycle units (human / 1K / 1M) | `f` | Cycle refresh rate (1s / 2s / 5s / 10s) |
| `t` | Cycle warn threshold (50-80%) | `T` | Cycle crit threshold (80-95%) |

## Configuration

All settings persist automatically to `~/.storageshower.conf` and are restored on startup. No manual editing required — just press keys to customize.
