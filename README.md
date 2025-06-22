# Rust Task Manager

A modern, extensible system resource monitor for desktop, inspired by Windows Task Manager.  
Built with [eframe/egui](https://crates.io/crates/eframe) for the GUI and [sysinfo](https://crates.io/crates/sysinfo) for system data.

## Features

- **Real-time per-core CPU usage graphs**
- **Memory usage history graph**
- **Disk usage graphs** (per-disk, with history)
- **Network usage graphs** (per-interface, with history)
- **Process table** with CPU, memory, and search/filter
- **Process actions:** Kill, Suspend, Resume
- **Configurable update interval**
- Modular, extensible codebase

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2021 or newer)
- A desktop OS supported by `egui`/`eframe` and `sysinfo` (Linux, Windows, macOS)

### Clone & Build

```sh
git clone https://github.com/ChrisSch-dev/task_manager_rust.git
cd task_manager_rust
cargo run --release
```

## Usage

- Search/filter processes via the search bar.
- Change update interval with the slider.
- Use action buttons to kill/suspend/resume processes.
- View live graphs of CPU, memory, disk, and network usage.

## File Structure

```
src/
├── main.rs            # Entry point
├── app.rs             # Application logic & UI
├── sysinfo_mod.rs     # System info gathering & logic
├── process_table.rs   # Process table UI
├── ui_utils.rs        # Graphs and reusable UI widgets
└── types/
    └── mod.rs         # Type definitions & data models
```

## Extending

- Add new features by extending `types/mod.rs` and the relevant UI module.
- Use `sysinfo` for more system data, or integrate with other crates for GPU, sensors, etc.

## License

MIT

---

**Inspired by Windows Task Manager, but open source and cross-platform!**
