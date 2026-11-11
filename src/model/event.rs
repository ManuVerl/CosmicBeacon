use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Category of an event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Category {
    /// 🔭 Classical astronomy event
    Astronomical,
    /// 📡 Radio-astronomy event
    RadioAstronomical,
}

/// Sub-types for Astronomical events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstronomicalType {
    IssFlyover,
    CometVisible,
    MeteorShower,
    PlanetVisible,
    Other(String),
}

/// Sub-types for RadioAstronomical events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RadioAstronomicalType {
    IssRadio,
    SolarTransit,
    CometTransit,
    MilkyWayTransit,
    Other(String),
}

/// Union of all event sub-types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    Astronomical(AstronomicalType),
    Radio(RadioAstronomicalType),
}

/// Celestial coordinates (horizontal system).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyCoord {
    /// Azimuth in degrees (0 = North, clockwise)
    pub azimuth_deg: f64,
    /// Elevation above horizon in degrees
    pub elevation_deg: f64,
}

/// A timed horizontal position, used to describe the path of a moving object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyPoint {
    pub time: DateTime<Utc>,
    /// Azimuth in degrees (0 = North, clockwise)
    pub azimuth_deg: f64,
    /// Elevation above horizon in degrees
    pub elevation_deg: f64,
}

/// A single observable event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub category: Category,
    pub event_type: EventType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub sky_position: Option<SkyCoord>,
    /// Optional equipment suggestion (e.g. "150mm telescope")
    pub equipment: Option<String>,
    /// Identifier of the external source that provided this event
    pub source: String,
    pub description: Option<String>,
    /// Radio-specific: frequency range in MHz (None for Astronomical events)
    pub freq_min_mhz: Option<f64>,
    pub freq_max_mhz: Option<f64>,
    /// Radio-specific: listening direction
    pub listen_direction: Option<SkyCoord>,
    /// Path of a moving object (e.g. ISS pass), sampled over time. None for fixed/unknown targets.
    #[serde(default)]
    pub sky_track: Option<Vec<SkyPoint>>,
}

#[allow(dead_code)]
impl Event {
    pub fn is_past(&self) -> bool {
        self.end_time < Utc::now()
    }

    pub fn is_upcoming(&self) -> bool {
        self.start_time > Utc::now()
    }
}
