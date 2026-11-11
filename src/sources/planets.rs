use crate::model::{
    event::{AstronomicalType, Category, Event, EventType, SkyCoord},
    position::Position,
};
use chrono::{Duration, TimeZone, Utc};
use uuid::Uuid;

/// Planet definitions: (name, JPL Horizons target ID, min magnitude threshold for "visible")
const PLANETS: &[(&str, &str, f64)] = &[
    ("Mercure",  "199", 3.0),
    ("Vénus",    "299", 5.0),
    ("Mars",     "499", 3.0),
    ("Jupiter",  "599", 5.0),
    ("Saturne",  "699", 5.0),
    ("Uranus",   "799", 6.5),
    ("Neptune",  "899", 8.0),
];

/// Query JPL Horizons for a single planet's visibility windows over the next year.
pub async fn fetch(
    position: &Position,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    let mut events = Vec::new();

    // Sample every 3 days over the next 365 days
    let now = Utc::now();
    let step_days = 3i64;
    let lookahead_days = 365i64;

    for (name, target_id, mag_limit) in PLANETS {
        match fetch_planet(position, name, target_id, *mag_limit, now, step_days, lookahead_days).await {
            Ok(mut evts) => events.append(&mut evts),
            Err(e) => log::warn!("Horizons query failed for {}: {}", name, e),
        }
    }

    Ok(events)
}

async fn fetch_planet(
    position: &Position,
    name: &str,
    target_id: &str,
    mag_limit: f64,
    now: chrono::DateTime<Utc>,
    step_days: i64,
    lookahead_days: i64,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    let start_str = now.format("%Y-%b-%d").to_string();
    let stop_str = (now + Duration::days(lookahead_days)).format("%Y-%b-%d").to_string();

    // JPL Horizons REST API — quantities: 4=airmass/vis, 9=range, 20=ang_sep, 31=obs_az/el, 9=Vmag
    // We request: 4 (airmass), 10 (RA/Dec apparent), 20 (helio range), 29 (constellation),
    // and most importantly 4+31 for azimuth/elevation + visibility.
    // Quantity 4 = Astrometric RA & Dec (J2000)
    // Quantity 31 = Observer az/el
    // Quantity 9  = Range (km)
    // Quantity 29 = Constellation
    // Simplified: ask for az/el + apparent magnitude (quantities 31,9)
    let url = format!(
        "https://ssd.jpl.nasa.gov/api/horizons.api\
         ?format=text\
         &COMMAND={target_id}\
         &OBJ_DATA=NO\
         &MAKE_EPHEM=YES\
         &EPHEM_TYPE=OBSERVER\
         &CENTER=coord\
         &SITE_COORD='{lon},{lat},0'\
         &COORD_TYPE=GEODETIC\
         &START_TIME='{start}'\
         &STOP_TIME='{stop}'\
         &STEP_SIZE='{step}d'\
         &QUANTITIES='4,31'\
         &CAL_FORMAT=BOTH\
         &ANG_FORMAT=DEG\
         &APPARENT=AIRLESS\
         &REF_SYSTEM=ICRF\
         &SKIP_DAYLT=NO",
        target_id = target_id,
        lon = position.longitude,
        lat = position.latitude,
        start = start_str,
        stop = stop_str,
        step = step_days,
    );

    let text = reqwest::get(&url).await?.text().await?;
    parse_horizons_response(name, target_id, mag_limit, &text, step_days)
}

/// Parse the plain-text Horizons ephemeris output and produce visibility events.
fn parse_horizons_response(
    planet_name: &str,
    _target_id: &str,
    _mag_limit: f64,
    text: &str,
    step_days: i64,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    // Find the $$SOE / $$EOE markers
    let soe = text.find("$$SOE").ok_or("No $$SOE marker in Horizons response")?;
    let eoe = text.find("$$EOE").ok_or("No $$EOE marker in Horizons response")?;
    let data = &text[soe + 5..eoe];

    let mut events = Vec::new();
    let mut visible_start: Option<chrono::DateTime<Utc>> = None;
    let mut last_az = 0.0f64;
    let mut last_el = 0.0f64;
    let mut max_el = 0.0f64;

    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Each line: "YYYY-Mon-DD HH:MM  JD  RA(deg) Dec(deg)  Az(deg) El(deg)"
        // Split by whitespace
        let cols: Vec<&str> = line.split_whitespace().collect();
        // Horizons observer table with quantities 4,31:
        // [date_str] [time_str] [jd] [RA_app] [Dec_app] [Az] [El]
        // col count >= 7
        if cols.len() < 7 {
            continue;
        }

        // Parse date+time from cols[0] and cols[1]
        let dt_str = format!("{} {}", cols[0], cols[1]);
        let dt = chrono::NaiveDateTime::parse_from_str(&dt_str, "%Y-%b-%d %H:%M")
            .ok()
            .map(|ndt| Utc.from_utc_datetime(&ndt));

        // Az = cols[5], El = cols[6]
        let az = cols[5].parse::<f64>().unwrap_or(-999.0);
        let el = cols[6].parse::<f64>().unwrap_or(-90.0);

        if let Some(dt) = dt {
            if el > 5.0 {
                // Planet is above horizon
                if visible_start.is_none() {
                    visible_start = Some(dt);
                    max_el = el;
                    last_az = az;
                    last_el = el;
                } else if el > max_el {
                    max_el = el;
                    last_az = az;
                    last_el = el;
                }
            } else if let Some(start) = visible_start.take() {
                // End of a visibility window
                let end = dt;
                if (end - start).num_days() >= 1 {
                    // Only emit if window spans at least 1 step (avoid noise)
                    events.push(make_planet_event(
                        planet_name,
                        start,
                        end,
                        last_az,
                        last_el,
                    ));
                }
                max_el = 0.0;
                last_az = 0.0;
                last_el = 0.0;
            }
        }
        let _ = (last_el, step_days);
    }

    // Close any open window at end of data
    if let Some(start) = visible_start.take() {
        let end = start + Duration::days(step_days);
        events.push(make_planet_event(planet_name, start, end, last_az, last_el));
    }

    Ok(events)
}

fn make_planet_event(
    name: &str,
    start: chrono::DateTime<Utc>,
    end: chrono::DateTime<Utc>,
    az: f64,
    el: f64,
) -> Event {
    Event {
        id: Uuid::new_v4(),
        title: format!("{} visible", name),
        category: Category::Astronomical,
        event_type: EventType::Astronomical(AstronomicalType::PlanetVisible),
        start_time: start,
        end_time: end,
        sky_position: Some(SkyCoord { azimuth_deg: az, elevation_deg: el }),
        equipment: None,
        source: "Planets (JPL Horizons)".to_string(),
        description: Some(format!(
            "Max elevation ~{:.0}°  —  visible from your position",
            el
        )),
        freq_min_mhz: None,
        freq_max_mhz: None,
        listen_direction: None,
        sky_track: None,
    }
}
