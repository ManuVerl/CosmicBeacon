# Cosmic Beacon

**Astronomical & radio-astronomical event tracker**

> Written in Rust with IBM Bob · egui/eframe UI · Linux & Windows

![light mode](./img/light.png)

![dark mode](./img/dark.png)

---

## Features

- 🔭 **Astronomical events**: ISS flybys, visible planets, meteor showers, observable comets
- 📡 **Radio-astronomical events**: ARISS ISS radio contacts, solar transit, Milky Way (Galactic Centre) transit
- 📍 **Multiple observer positions**: name, icon, lat/lon — with **🎯 auto-detect** (geocoding by city name via Nominatim, or IP geolocation as fallback)
- 🔄 **Automatic refresh** from free public data sources (CelesTrak TLE, JPL Horizons, IMO, MPC, ARISS)
- 🎨 **5 colour themes**: Dark · Light · Teal · Pink · Navy (X11-inspired)
- 🌍 **6 UI languages**: French · English · Spanish · Portuguese · German · Italian (auto-detected from system locale, saved across sessions)
- ⏱ **Temporal navigation bar**: jump to *Now* or seek to any date/time directly from the event list
- 📋 **Sync report** window showing per-source status and last update time
- 🌅 **Sidebar ephemeris**: real-time clock, sun arc widget (sunrise/sunset), moon phase disc

---

## Requirements

- Rust stable ≥ 1.75 ([install rustup](https://rustup.rs))
- **Linux**: packages `libgtk-3-dev`, `libxcb-*` (required by egui/eframe)
- **Windows**: Visual Studio Build Tools (MSVC toolchain)

---

## Build

```bash
# Debug build
cargo build

# Optimised release build
cargo build --release
```

The release binary is written to `target/release/cosmic-beacon` (Linux) or `target\release\cosmic-beacon.exe` (Windows).

---

## Run

```bash
cargo run --release
```

---

## Configuration

Settings are persisted automatically at `{config_dir}/cosmic-beacon/cosmic_beacon_config.toml`:

| Platform | Path |
|----------|------|
| Windows  | `%APPDATA%\cosmic-beacon\cosmic_beacon_config.toml` |
| Linux    | `~/.config/cosmic-beacon/cosmic_beacon_config.toml` |
| macOS    | `~/Library/Application Support/cosmic-beacon/cosmic_beacon_config.toml` |

Saved settings include: selected theme, UI language, observer positions, and update frequency.

---

## Project layout

```
src/
├── main.rs          # Entry point, Tokio runtime, eframe bootstrap
├── app.rs           # Global application state (CosmicBeaconApp)
├── i18n.rs          # Translations & date/time localisation (6 languages)
├── ui/              # UI components (egui panels and windows)
│   ├── main_window.rs
│   ├── event_list.rs    # Event list + temporal navigation bar
│   ├── settings.rs
│   ├── positions.rs
│   ├── sidebar_info.rs  # Clock, sun arc, moon phase widgets
│   ├── sync_report.rs
│   ├── about.rs
│   └── theme.rs         # 5 theme definitions
├── model/           # Data structures (Event, Position, …)
├── sources/         # External data connectors
│   ├── manager.rs
│   ├── iss_passes.rs
│   ├── planets.rs
│   ├── meteors.rs
│   ├── comets.rs
│   ├── solar_transit.rs
│   ├── iss_radio.rs
│   └── milky_way.rs     # Galactic Centre meridian transit
├── config/          # TOML settings persistence
└── utils/           # Geo helpers, astronomical calculations, geocoding
    └── geocoding.rs     # Nominatim (name → lat/lon) + ip-api.com (IP → lat/lon)
```

---

## Specification

See [SPEC.md](./SPEC.md) for the full functional and technical specification.

---

## License

MIT
