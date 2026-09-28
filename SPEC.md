# Cosmic Beacon — Functional & Technical Specification

> Version: 0.3.0
> Date: 2025
> Status: **Active**

---

## 1. Overview

**Cosmic Beacon** is a cross-platform desktop application (Linux, Windows) written in **Rust** that tracks and displays **astronomical** and **radio-astronomical** events visible or audible from one or more user-defined geographic positions.

The application is built with the **egui/eframe** immediate-mode UI framework and relies entirely on free, open data sources. All network calls run asynchronously in the background; the UI remains fully responsive at all times.

---

## 2. Goals

| # | Goal |
|---|------|
| 1 | Display upcoming and past astronomical/radio-astronomical events for a given position |
| 2 | Support multiple named, geo-located observer positions |
| 3 | Automatically fetch data from free public APIs |
| 4 | Stay responsive (UI never blocked) even when external sources are unavailable |
| 5 | Provide minimal but complete settings: theme, language, update frequency |
| 6 | Fully internationalised: 6 UI languages, localised date/time formatting |

---

## 3. Features

### 3.1 Observer Positions

A **position** is described by:

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Free-text name (e.g. "Home", "Observatory") |
| `icon` | `Enum` | Icon from a predefined set: 🏠 Home, 🔭 Observatory, 🏕 Field, 📡 Station, 🏔 Mountain |
| `latitude` | `f64` | Decimal degrees (−90 … +90) |
| `longitude` | `f64` | Decimal degrees (−180 … +180) |
| `acquisition` | `Enum` | `GPS` (automatic) or `Manual` (keyboard entry) |

- The user may store **n** positions.
- One position is designated as the **active position** used for event computation.
- Deleting the active position redirects to the next available position.

#### 3.1.1 Automatic coordinate detection

A **🎯 button** next to the Lat/Lon fields triggers an automatic coordinate lookup without blocking the UI. The button is disabled while a lookup is in progress.

| Condition | Strategy |
|-----------|----------|
| **Name field is filled** | Forward geocoding via **Nominatim** (OpenStreetMap) — resolves city/place name to lat/lon |
| **Name field is empty** | IP geolocation via **ip-api.com** (free, no API key required) — resolves the machine's public IP to an approximate position |
| **Both strategies fail** | A warning message is shown in the form; fields remain editable |

Resolution is executed in a `std::thread::spawn` background thread. On completion, `egui` is notified via `ctx.request_repaint()` and the result is applied on the next frame:

- **Success**: Lat/Lon fields are populated with 6 decimal places. If the name field was empty, it is also pre-filled with the resolved location name.
- **Failure**: A localised ⚠ warning is displayed below the Lat/Lon row. Three distinct error states are distinguished: *name not found*, *network error*, *no position detected*.

All error messages are translated in all 6 supported languages (keys `positions.geo_err_*`).

> **Note**: GPS hardware access is not available in a desktop Rust/egui application; IP geolocation is used as the automatic fallback when no city name is provided.

### 3.2 Event Catalogue

Every event carries the following common fields:

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Uuid` | Unique identifier |
| `title` | `String` | Event label |
| `category` | `Enum` | `Astronomical` 🔭 or `RadioAstronomical` 📡 |
| `event_type` | `Enum` | Sub-type (see §3.2.1 and §3.2.2) |
| `start_time` | `DateTime<Utc>` | Event start (UTC) |
| `end_time` | `DateTime<Utc>` | Event end (UTC) |
| `sky_position` | `SkyCoord` | Azimuth + elevation above the observer's horizon |
| `equipment` | `Option<String>` | Suggested equipment (e.g. "150 mm telescope") |
| `source` | `String` | Identifier of the external source |
| `description` | `Option<String>` | Optional short description |

Radio-astronomical events additionally carry:

| Field | Type | Description |
|-------|------|-------------|
| `freq_min_mhz` | `f64` | Lower frequency bound (MHz) |
| `freq_max_mhz` | `f64` | Upper frequency bound (MHz) |
| `listen_direction` | `Option<SkyCoord>` | Listening direction (azimuth/elevation) |

#### 3.2.1 Astronomical Event Sub-types (`Astronomical` 🔭)

| Sub-type | Description |
|----------|-------------|
| `IssFlyover` | ISS pass over the observer position |
| `CometVisible` | Comet visible to the naked eye or telescope |
| `MeteorShower` | Notable meteor shower |
| `PlanetVisible` | Solar-system planet above the horizon |
| `Other` | Any other notable event |

#### 3.2.2 Radio-Astronomical Event Sub-types (`RadioAstronomical` 📡)

| Sub-type | Description |
|----------|-------------|
| `IssRadio` | ARISS amateur radio contact with the ISS |
| `SolarTransit` | Sun crossing the local meridian (best dish-pointing time) |
| `CometTransit` | Comet radio transit (frequency + listening axis) |
| `MilkyWayTransit` | Galactic Centre (Sgr A*) crossing the local meridian |
| `Other` | Any other notable radio event |

### 3.3 Display Time Window

| Direction | Duration |
|-----------|----------|
| Future | Up to **+1 year** from today |
| Past | Up to **−1 month** from today |

Events are sorted in **ascending chronological order**. On startup and after a refresh the list auto-scrolls to the first event at or after the current time.

### 3.4 Temporal Navigation Bar

A persistent control bar sits above the event list:

| Control | Behaviour |
|---------|-----------|
| **Now** button | Resets the time cursor to the current UTC time and triggers a scroll |
| **Go to** date/time field | Accepts `DD/MM/YYYY HH:MM`, `DD.MM.YYYY HH:MM` (German) or `YYYY-MM-DD HH:MM` (ISO); scrolls the list to the target time on a valid parse |
| Current cursor label | Right-aligned display of the active cursor time, updated every second |

### 3.5 External Data Sources

All network calls are executed asynchronously (Tokio). Each source has its own module under `src/sources/`.

| Source | URL / Method | Data provided |
|--------|-------------|---------------|
| **CelesTrak TLE** | `https://celestrak.org` | ISS TLE elements → pass prediction (SGP4) |
| **NASA JPL Horizons** | `https://ssd.jpl.nasa.gov/api/horizons.api` | Planetary ephemerides |
| **IMO Meteor Calendar** | `https://www.imo.net` | Meteor shower calendar |
| **IAU Minor Planet Center** | `https://minorplanetcenter.net` | Observable comets |
| **ARISS** | `https://www.ariss.org` (RSS) | ISS amateur radio contacts |
| **Solar Transit** | Local calculation (NOAA algorithm) | Daily solar meridian transit |
| **Milky Way Transit** | Local calculation (LST + Sgr A* coordinates) | Daily Galactic Centre meridian transit |

> **Mandatory rule**: all source calls run in background tasks. The UI thread is never blocked.

#### 3.5.1 Sync Report

A dedicated window (menu or status-bar button) lists for each source:

| Field | Description |
|-------|-------------|
| `source_name` | Source identifier |
| `status` | `OK` ✅ / `Error` ❌ / `Pending` 🔄 |
| `last_sync` | Timestamp of last successful sync (localised) |
| `error_message` | Error description if applicable |

### 3.6 Sun Widget

The Sun widget is displayed at the bottom of the left sidebar whenever an active position is set. It provides real-time solar ephemeris data recomputed every second.

#### Data displayed

```
[arc graphic — sun path across the sky]
Daylight: 12 h 30 min
Rise: 06:42  |  Set: 19:12
Altitude: -7.85°
Direction: 282.36° (WNW)
```

| Field | Description |
|-------|-------------|
| **Arc graphic** | Semi-circular arc representing the Sun's path across the sky; the portion already traversed is highlighted in golden amber; a dot marks the current Sun position |
| **Daylight** | Total duration of daylight for the day (hours and minutes); "Does not rise" under polar night |
| **Rise / Set** | Local sunrise and sunset times (HH:MM); "Does not rise" / `--:--` under polar conditions |
| **Altitude** | Current solar altitude above (or below) the local horizon in degrees, two decimal places; negative when the Sun is below the horizon |
| **Direction** | Current solar azimuth in degrees (0° = North, clockwise), two decimal places, followed by the 16-point compass abbreviation (e.g. `WNW`), localised for each language |

#### Calculation method

All calculations are performed in `src/utils/astro.rs` (`sun_ephemeris` + `calculate_sun_times`), accurate to ~0.01°, verified against NOAA Solar Calculator reference data.

**Step 1 — Solar ephemeris** (`sun_ephemeris(dt)`):

| Intermediate | Formula |
|---|---|
| Julian centuries | T = (JD − 2 451 545.0) / 36 525 |
| Geometric mean longitude | L₀ = 280.46646 + 36 000.76983·T + 0.0003032·T² (mod 360°) |
| Mean anomaly | M = 357.52911 + 35 999.05029·T − 0.0001537·T² (mod 360°) |
| Equation of centre | C = (1.914602 − 0.004817·T − 0.000014·T²)·sin M + (0.019993 − 0.000101·T)·sin 2M + 0.000289·sin 3M |
| True longitude | ☉ = L₀ + C |
| Apparent longitude | λ = ☉ − 0.00569° − 0.00478°·sin Ω, where Ω = 125.04 − 1934.136·T |
| Obliquity (corrected) | ε = ε₀ + 0.00256°·cos Ω |
| Declination | δ = arcsin(sin ε · sin λ) |
| Eccentricity | e = 0.016708634 − 0.000042037·T − 0.0000001267·T² |
| Equation of Time | EoT (min) = 4° · [y·sin 2L₀ − 2e·sin M + 4ey·sin M·cos 2L₀ − ½y²·sin 4L₀ − 1.25e²·sin 2M], y = tan²(ε/2) |

**Step 2 — Solar noon & rise/set** (`calculate_sun_times(lat, lon, now_local)`):

- **Solar noon** (UTC h) = 12 − lon/15 − EoT/60
- **Half-day hour angle**: cos(HA₀) = (cos 90.833° − sin lat · sin δ) / (cos lat · cos δ); 90.833° = atmospheric refraction (0.567°) + solar disc radius (0.267°)
- **Sunrise** = solar noon − HA₀/15 h; **Sunset** = solar noon + HA₀/15 h (both converted to local time)

**Step 3 — Current altitude & azimuth**:

- **Hour angle**: HA = (t_utc − solar_noon) × 15° (positive when Sun is west of meridian)
- **Altitude**: sin(alt) = sin(lat)·sin(δ) + cos(lat)·cos(δ)·cos(HA)
- **Azimuth**: cos(Az) = (sin(δ) − sin(lat)·sin(alt)) / (cos(lat)·cos(alt)); if HA > 0° → Az = 360° − Az_raw, else Az = Az_raw
- **cos(alt)** computed as √(1 − sin²(alt)) for numerical stability near the horizon

**Accuracy** (verified vs. reference data for Attiches, France, 17 Sep 2026):

| Value | Reference | Computed | Δ |
|---|---|---|---|
| Sunrise (CEST) | 07:26 | 07:27 | 1 min |
| Sunset (CEST) | 19:57 | 19:57 | 0 min |
| Altitude | −11.10° | −11.08° | 0.02° |
| Azimuth | 287.14° | 287.12° | 0.02° |

- **Source**: NOAA Solar Calculator algorithms — <https://gml.noaa.gov/grad/solcalc/>

#### Polar conditions

When the Sun never rises (polar night, cos(HA₀) > 1) or never sets (midnight sun, cos(HA₀) < −1), `sunrise_local` and `sunset_local` are `None`; the Daylight field shows "Does not rise" and the arc graphic places the Sun below the horizon line.

---

### 3.7 Moon Widget

The Moon widget appears immediately below the Sun widget in the left sidebar whenever an active position is set. It is also recomputed every second.

#### Data displayed

```
🌒 Waxing Crescent
Illumination: 40.0%
Rise: 09:14  |  Set: 22:47
Altitude: 10.4° (above horizon)
Direction: 17.5° (NNE)
```

| Field | Description |
|-------|-------------|
| **Disc graphic** | Vector rendering of the lunar disc showing the illuminated crescent/gibbous geometry, updated in real time |
| **Phase name** | Localised name of the current phase (e.g. "Waxing Crescent", "Pleine Lune") |
| **Illumination** | Percentage of the lunar disc that is illuminated (one decimal place) |
| **Rise / Set** | Local time of moonrise and moonset (HH:MM); `--:--` or "Does not rise" under polar conditions |
| **Altitude** | Current altitude of the Moon above or below the local horizon in degrees, with an "above/below horizon" qualifier |
| **Direction** | Current azimuth in degrees (0° = North, clockwise) followed by the 16-point compass abbreviation (N, NNE, NE, …, NNW), localised (O/SO/… in French, Spanish, Portuguese, Italian) |

#### Calculation method

- **Phase & illumination**: Julian Day delta from a known New Moon epoch (JD 2451549.760 = 6 Jan 2000 18:14 UTC); synodic month = 29.53058867 days; illumination = `(1 − cos(2π·phase)) / 2`.
- **Lunar ecliptic coordinates**: Jean Meeus *Astronomical Algorithms* ch. 47 — 13 leading longitude terms + 10 latitude terms from the fundamental arguments L′, D, M, M′, F.
- **Equatorial coordinates**: ecliptic→equatorial rotation using the mean obliquity of the ecliptic (23.439° − 0.013°·T).
- **Horizontal coordinates** (altitude, azimuth): equatorial→horizontal via Local Sidereal Time (Greenwich MST + observer longitude), standard hour-angle formula.
- **Moonrise / Moonset**: iterative step-search at 10-minute resolution over a 48-hour window from the preceding midnight UTC; crossing times are refined by linear interpolation.

#### Polar conditions

When the Moon stays permanently above or below the horizon, the rise/set fields display `--:--` or "Does not rise" respectively.

---

### 3.8 Event Display

- **List view**: chronological list with category icon, title, localised date/time, sky position (Az/El), and frequency range for radio events.
- **Dimming**: past events (end time < now) are displayed at reduced opacity.
- **Filters** (sidebar):
  - By category: Astronomical 🔭 / Radio-Astronomical 📡 / All

### 3.9 Settings

| Parameter | Values | Default |
|-----------|--------|---------|
| Theme | `Dark` / `Light` / `Teal` / `Pink` / `Navy` | `Dark` |
| Language | `fr` / `en` / `es` / `pt` / `de` / `it` | System locale |
| Update frequency | `OnStartup` / `Weekly` / `Monthly` | `OnStartup` |
| Active position | Any registered position | First created |

All settings are persisted to `{config_dir}/cosmic-beacon/cosmic_beacon_config.toml` (TOML format) and restored on the next launch.

---

## 4. Internationalisation

### 4.1 Supported Languages

| Code | Language |
|------|----------|
| `fr` | French |
| `en` | English |
| `es` | Spanish |
| `pt` | Portuguese |
| `de` | German |
| `it` | Italian |

Language detection reads `LANG` / `LANGUAGE` / `LC_ALL` environment variables on startup. The selected language is saved in settings and takes effect immediately (UI strings, date/time formats, moon phase names, cardinal directions).

### 4.2 Date/Time Formatting

All displayed dates and times are formatted according to the active language:

| Language | Date format example |
|----------|---------------------|
| fr / es / pt / it | `Lundi 14 janvier 2025` |
| en | `Monday, January 14 2025` |
| de | `Montag, 14. Januar 2025` |

Event timestamps always include the UTC indicator (`HH:MM UTC`).

---

## 5. Visual Themes

Five themes are available, inspired by classic X11 window manager colour palettes:

| Theme | Base style | Background |
|-------|-----------|------------|
| **Dark** | egui default dark | Very dark grey |
| **Light** | Softened off-white | Warm light grey |
| **Teal** | CDE / Motif X11 | Deep teal (`#006060`) |
| **Pink** | SGI Irix / X11 mauve | Deep magenta (`#50003c`) |
| **Navy** | FVWM / TWM navy | Deep navy blue (`#0a1446`) |

All coloured themes derive from `Visuals::dark()` to ensure readable contrast. The selection is applied immediately and persisted.

### 5.1 Accessibility — Semantic colour tokens

Two theme-aware colour functions are exposed in `src/ui/theme.rs` and must be used instead of hard-coded values wherever diagnostic feedback is shown:

| Function | Purpose | WCAG AA contrast (≥ 4.5:1) |
|----------|---------|-----------------------------|
| `error_color(theme)` | Error / warning messages | ✅ Guaranteed on all 5 backgrounds |
| `muted_color(theme)` | Secondary info, "searching…", coordinates | ✅ Guaranteed on all 5 backgrounds |

Colour values per theme:

| Theme | `error_color` | `muted_color` |
|-------|---------------|---------------|
| **Dark** | `#FF7070` — soft red | `#B4B9C8` — light grey |
| **Light** | `#C0270A` — dark red | `#50586A` — mid grey |
| **Teal** | `#FFB3B3` — light rose | `#A0E6E6` — teal-white |
| **Pink** | `#FFD0A0` — apricot | `#E6BEE1` — soft lilac |
| **Navy** | `#FF9E9E` — salmon | `#A0BEF5` — periwinkle |

> **Rule**: never use `ui.visuals().weak_text_color()` or a hard-coded `Color32` for user-visible diagnostic labels. Always call `error_color()` or `muted_color()` from `src/ui/theme.rs`.

---

## 6. Technical Architecture

### 6.1 Language & Platform

| Item | Choice |
|------|--------|
| Language | **Rust** (edition 2021) |
| Targets | `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc` |
| Toolchain | `stable` ≥ 1.75 |

### 6.2 UI Framework

**egui 0.27 / eframe 0.27** — immediate-mode, no system dependencies beyond OpenGL/wgpu, cross-platform.

### 6.3 Key Crates

| Crate | Purpose |
|-------|---------|
| `eframe` / `egui` | UI framework |
| `tokio` (full features) | Async runtime for background source fetches |
| `reqwest` (blocking + json) | HTTP client (rustls TLS) — async source fetches **and** blocking geocoding calls in background threads |
| `sgp4` | SGP4 orbital mechanics (ISS pass prediction) |
| `quick-xml` | XML / RSS parsing (ARISS feed) |
| `serde` / `serde_json` / `toml` | Serialisation |
| `chrono` | Date/time handling |
| `uuid` | Unique event identifiers |
| `dirs` | Cross-platform config/data directories |
| `log` + `env_logger` | Logging |

### 6.4 Project Structure

```
cosmic-beacon/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── SPEC.md
└── src/
    ├── main.rs                # Entry point, Tokio runtime init, eframe run
    ├── app.rs                 # AstroFeedApp struct, global state, eframe::App impl
    ├── i18n.rs                # Lang enum, t() translation fn, date/time helpers
    ├── ui/
    │   ├── mod.rs
    │   ├── main_window.rs     # Main layout: menu, status bar, sidebar, central panel
    │   ├── event_list.rs      # Event list + temporal navigation bar
    │   ├── event_detail.rs    # Event detail view (stub)
    │   ├── positions.rs       # Position management window
    │   ├── settings.rs        # Settings window (theme, language, update freq)
    │   ├── sidebar_info.rs    # Clock, sun arc widget, moon phase widget
    │   ├── sync_report.rs     # Sync report window
    │   ├── about.rs           # About window
    │   └── theme.rs           # 5 Visuals definitions
    ├── model/
    │   ├── mod.rs
    │   ├── event.rs           # Event, SkyCoord, EventType, Category
    │   └── position.rs        # Position, PositionIcon
    ├── sources/
    │   ├── mod.rs
    │   ├── manager.rs         # SourceManager, SyncReport, SyncStatus
    │   ├── iss_passes.rs      # ISS passes (CelesTrak TLE + SGP4)
    │   ├── planets.rs         # Planets (JPL Horizons)
    │   ├── meteors.rs         # Meteor showers (IMO)
    │   ├── comets.rs          # Comets (MPC)
    │   ├── solar_transit.rs   # Solar meridian transit (local calculation)
    │   ├── iss_radio.rs       # ISS radio contacts (ARISS RSS)
    │   └── milky_way.rs       # Galactic Centre meridian transit (local calculation)
    ├── config/
    │   ├── mod.rs
    │   └── settings.rs        # Settings struct, Theme enum, TOML persistence
    └── utils/
        ├── mod.rs
        ├── astro.rs           # Sun times, moon phase calculations
        ├── geo.rs             # Coordinate helpers (Haversine, deg↔rad)
        └── geocoding.rs       # Nominatim + IP-geolocation helpers
```

### 6.5 Data Flow

```
┌──────────────────────────────────────────────────┐
│               UI Thread (egui/eframe)            │
│  EventList ◄─── AstroFeedApp ◄─── Arc<Mutex<>>  │
└────────────────────────────┬─────────────────────┘
                             │  Arc<Mutex<Vec<Event>>>
                             ▼
┌──────────────────────────────────────────────────┐
│            Background Tasks (tokio::spawn)       │
│  SourceManager ──► tokio::join! [               │
│      iss_passes, planets, meteors, comets,       │
│      solar_transit, iss_radio, milky_way         │
│  ] ──► normalize ──► sort by start_time          │
└──────────────────────────────────────────────────┘
```

### 6.6 Persistence

| Data | Location |
|------|----------|
| Settings (TOML) | `{config_dir}/cosmic-beacon/cosmic_beacon_config.toml` |

No embedded database is required. Events are recomputed from sources on each refresh cycle.

---

## 7. Non-Functional Requirements

| Requirement | Detail |
|-------------|--------|
| **Network resilience** | Any unavailable source produces an error entry in the sync report; it never crashes or blocks the app |
| **UI performance** | UI remains fluid (≥ 30 fps) during background network calls |
| **Portability** | Compiles without modification on Linux and Windows |
| **Minimal storage** | No heavy embedded database; TOML config only |
| **Privacy** | Only lat/lon coordinates are sent to external APIs; no personal data |

---

## 8. Roadmap

| Milestone | Content | Status |
|-----------|---------|--------|
| **M0 — Scaffolding** | Project structure, data models, skeleton UI | ✅ Done |
| **M1 — Positions** | Position CRUD, active selection, TOML persistence | ✅ Done |
| **M2 — ISS source** | ISS pass prediction via CelesTrak TLE + SGP4 | ✅ Done |
| **M3 — More sources** | Planets (JPL), meteors (IMO), comets (MPC) | ✅ Done |
| **M4 — Radio astronomy** | ARISS contacts, solar transit, Milky Way transit | ✅ Done |
| **M5 — Settings & polish** | 5 themes, sync report, sidebar ephemeris, app icon | ✅ Done |
| **M6 — i18n & UX** | 6 languages, temporal navigation, stable window IDs | ✅ Done (v0.2.0) |
| **M7 — Geo auto-detect** | 🎯 Automatic lat/lon detection (Nominatim + IP geoloc) in position form | ✅ Done (v0.3.0) |
| **M8 — Future** | Sky map view, event detail panel, packaging (AppImage / MSI) | 🔲 Planned |

---

## 9. Open Questions

| # | Question |
|---|----------|
| Q1 | Add a polar sky map view (azimuthal projection) for a future minor release? |
| Q2 | Package as AppImage (Linux) and NSIS/MSI installer (Windows)? |
| Q3 | Add a SQLite event cache for faster restarts when sources are slow? |
| Q4 | Expose an optional HTTP/WebSocket API for integration with external tools? |

---

*End of specification — Cosmic Beacon v0.3.0*
