use crate::model::{
    event::{AstronomicalType, Category, Event, EventType},
    position::Position,
};
use chrono::{Duration, TimeZone, Utc};
use uuid::Uuid;

/// IMO annual meteor shower calendar (hard-coded — rarely changes year to year).
/// Fields: (name, peak_month, peak_day, active_start_day_offset, active_end_day_offset,
///          ZHR_peak, parent_body, equipment)
/// day offsets are relative to the peak date.
struct ShowerEntry {
    name: &'static str,
    peak_month: u32,
    peak_day: u32,
    /// Days before peak when activity begins
    days_before: i64,
    /// Days after peak when activity ends
    days_after: i64,
    /// Zenithal Hourly Rate at peak
    zhr: u32,
    parent: &'static str,
}

const SHOWERS: &[ShowerEntry] = &[
    ShowerEntry { name: "Quadrantids",          peak_month: 1,  peak_day: 4,  days_before: 3,  days_after: 2,  zhr: 120, parent: "Asteroid 2003 EH1" },
    ShowerEntry { name: "Lyrids",               peak_month: 4,  peak_day: 22, days_before: 3,  days_after: 2,  zhr: 18,  parent: "Comet Thatcher" },
    ShowerEntry { name: "Eta-Aquariids",         peak_month: 5,  peak_day: 6,  days_before: 5,  days_after: 5,  zhr: 50,  parent: "Comet Halley" },
    ShowerEntry { name: "Delta-Aquariids South", peak_month: 7,  peak_day: 30, days_before: 10, days_after: 10, zhr: 25,  parent: "Comet 96P/Machholz" },
    ShowerEntry { name: "Perseids",             peak_month: 8,  peak_day: 13, days_before: 5,  days_after: 5,  zhr: 100, parent: "Comet 109P/Swift-Tuttle" },
    ShowerEntry { name: "Orionids",             peak_month: 10, peak_day: 21, days_before: 5,  days_after: 5,  zhr: 20,  parent: "Comet Halley" },
    ShowerEntry { name: "Taurids South",        peak_month: 11, peak_day: 5,  days_before: 15, days_after: 15, zhr: 5,   parent: "Comet 2P/Encke" },
    ShowerEntry { name: "Taurids North",        peak_month: 11, peak_day: 12, days_before: 15, days_after: 15, zhr: 5,   parent: "Comet 2P/Encke" },
    ShowerEntry { name: "Leonids",              peak_month: 11, peak_day: 17, days_before: 3,  days_after: 3,  zhr: 15,  parent: "Comet 55P/Tempel-Tuttle" },
    ShowerEntry { name: "Geminids",             peak_month: 12, peak_day: 14, days_before: 4,  days_after: 4,  zhr: 150, parent: "Asteroid 3200 Phaethon" },
    ShowerEntry { name: "Ursids",               peak_month: 12, peak_day: 22, days_before: 2,  days_after: 2,  zhr: 10,  parent: "Comet 8P/Tuttle" },
];

/// Returns meteor shower events spanning ±1 month from today up to +1 year.
/// No network call — all data is built-in.
pub async fn fetch(
    _position: &Position,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    let now = Utc::now();
    let window_start = now - Duration::days(30);
    let window_end   = now + Duration::days(365);

    let mut events = Vec::new();

    // Produce entries for this year and next year so the window is always covered
    for year_offset in 0i32..=1 {
        let year = now.year() + year_offset;
        for s in SHOWERS {
            // Build peak datetime
            let peak = match Utc.with_ymd_and_hms(year, s.peak_month, s.peak_day, 0, 0, 0) {
                chrono::offset::LocalResult::Single(dt) => dt,
                _ => continue,
            };
            let start = peak - Duration::days(s.days_before);
            let end   = peak + Duration::days(s.days_after);

            // Skip if outside our display window
            if end < window_start || start > window_end {
                continue;
            }

            let equipment = if s.zhr >= 50 {
                "Naked eye — excellent show"
            } else if s.zhr >= 20 {
                "Naked eye"
            } else {
                "Naked eye — moderate activity"
            };

            events.push(Event {
                id: Uuid::new_v4(),
                title: format!("Meteor shower — {}", s.name),
                category: Category::Astronomical,
                event_type: EventType::Astronomical(AstronomicalType::MeteorShower),
                start_time: start,
                end_time: end,
                sky_position: None, // radiant varies by shower, not computed here
                equipment: Some(equipment.to_string()),
                source: "Meteor Showers (IMO calendar)".to_string(),
                description: Some(format!(
                    "Peak on {}/{} — max ZHR ~{}  |  Parent body: {}",
                    peak.day(), peak.month(), s.zhr, s.parent
                )),
                freq_min_mhz: None,
                freq_max_mhz: None,
                listen_direction: None,
                sky_track: None,
            });
        }
    }

    // Sort chronologically
    events.sort_by_key(|e| e.start_time);
    Ok(events)
}

// Need year() from Datelike
use chrono::Datelike;
