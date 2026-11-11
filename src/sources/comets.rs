use crate::model::{
    event::{AstronomicalType, Category, Event, EventType, RadioAstronomicalType},
    position::Position,
};
use chrono::{Duration, TimeZone, Utc};
use serde::Deserialize;
use uuid::Uuid;

/// MPC JSON endpoint for "bright" comets (updated daily).
const MPC_COMETS_URL: &str =
    "https://www.minorplanetcenter.net/Extended_Files/Soft00Cmt.txt";

/// Fetches visible comets from the Minor Planet Center and produces both
/// astronomical (visual) and radio-astronomical events.
pub async fn fetch(
    position: &Position,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    let text = reqwest::get(MPC_COMETS_URL).await?.text().await?;
    parse_mpc_comets(&text, position)
}

/// MPC "Soft00Cmt.txt" format — fixed-width. We extract the fields we need.
/// Line format (columns 1-based):
///  1-4   : Periodic comet number (blank if not periodic)
///  5      : comet type code
///  6-12  : provisional designation or number
///  15-18 : year of perihelion
///  20-21 : month of perihelion
///  23-29 : day of perihelion (decimal)
///  31-39 : perihelion distance q (AU)
///  41-49 : eccentricity e
///  52-59 : argument of perihelion ω
///  62-69 : longitude of ascending node Ω
///  72-79 : inclination i
///  92-95 : absolute magnitude H
///  97-100: slope parameter G
///  103+  : object name
fn parse_mpc_comets(
    text: &str,
    position: &Position,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    let now = Utc::now();
    let window_end = now + Duration::days(365);
    let window_start = now - Duration::days(30);

    let mut events = Vec::new();

    for line in text.lines() {
        if line.len() < 103 {
            continue;
        }

        // Parse perihelion date
        let peri_year  = line[14..18].trim().parse::<i32>().unwrap_or(0);
        let peri_month = line[19..21].trim().parse::<u32>().unwrap_or(0);
        let peri_day_f = line[22..29].trim().parse::<f64>().unwrap_or(0.0);
        let peri_day   = peri_day_f.floor() as u32;

        if peri_year == 0 || peri_month == 0 || peri_day == 0 {
            continue;
        }

        let perihelion = match Utc.with_ymd_and_hms(peri_year, peri_month, peri_day, 0, 0, 0) {
            chrono::offset::LocalResult::Single(dt) => dt,
            _ => continue,
        };

        // Comets are typically observable ±3 months around perihelion
        let vis_start = perihelion - Duration::days(90);
        let vis_end   = perihelion + Duration::days(90);

        if vis_end < window_start || vis_start > window_end {
            continue;
        }

        // Parse perihelion distance q (AU)
        let q_au = line[30..39].trim().parse::<f64>().unwrap_or(999.0);
        // Parse absolute magnitude H
        let h_mag = line[91..95].trim().parse::<f64>().unwrap_or(20.0);

        // Estimate peak apparent magnitude (crude — ignores observer–comet distance)
        // Very rough: mag ≈ H + 5*log10(q) for observer near Earth at perihelion
        let est_mag = h_mag + 5.0 * q_au.log10();

        // Skip very faint comets (> mag 12)
        if est_mag > 12.0 {
            continue;
        }

        // Comet name from col 103 onward
        let name = line[102..].trim();
        if name.is_empty() {
            continue;
        }

        let equipment = if est_mag < 6.0 {
            "Naked eye"
        } else if est_mag < 9.0 {
            "Binoculars 10×50 or larger"
        } else {
            "Telescope recommended"
        };

        let desc = format!(
            "Perihelion {}/{}/{} — dist. {:.2} AU — est. mag {:.1}",
            peri_day, peri_month, peri_year, q_au, est_mag
        );

        // 1. Astronomical event (visual observation)
        events.push(Event {
            id: Uuid::new_v4(),
            title: format!("Comet {} visible", name),
            category: Category::Astronomical,
            event_type: EventType::Astronomical(AstronomicalType::CometVisible),
            start_time: vis_start,
            end_time: vis_end,
            sky_position: None, // full ephemeris would need Horizons query
            equipment: Some(equipment.to_string()),
            source: "Comets (MPC)".to_string(),
            description: Some(desc.clone()),
            freq_min_mhz: None,
            freq_max_mhz: None,
            listen_direction: None,
            sky_track: None,
        });

        // 2. Radio-astronomical event — outgassing produces OH emission at 1667/1665 MHz
        // and H2O maser at 22235 MHz near perihelion passage
        let radio_start = perihelion - Duration::days(30);
        let radio_end   = perihelion + Duration::days(30);
        if radio_end >= window_start && radio_start <= window_end {
            let _ = position; // position would be used for direction when ephemeris is available
            events.push(Event {
                id: Uuid::new_v4(),
                title: format!("Comet {} radio transit (OH/H₂O)", name),
                category: Category::RadioAstronomical,
                event_type: EventType::Radio(RadioAstronomicalType::CometTransit),
                start_time: radio_start,
                end_time: radio_end,
                sky_position: None,
                equipment: Some("Receiver 1.6–22 GHz, directional antenna".to_string()),
                source: "Comets (MPC)".to_string(),
                description: Some(format!(
                    "{desc}  |  Listen: OH line 1665/1667 MHz, H₂O maser 22235 MHz"
                )),
                freq_min_mhz: Some(1665.0),
                freq_max_mhz: Some(22235.0),
                listen_direction: None,
                sky_track: None,
            });
        }
    }

    events.sort_by_key(|e| e.start_time);
    Ok(events)
}

// Silence unused import warning — Deserialize is available for future JSON sources
#[allow(dead_code)]
fn _unused_deserialize_marker<'de, T: Deserialize<'de>>() {}
