use crate::model::{
    event::{Category, Event, EventType, RadioAstronomicalType, SkyCoord},
    position::Position,
};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc};
use std::f64::consts::PI;
use uuid::Uuid;

/// Produces daily solar transit events for the observer position.
/// The "transit" (solar noon) is the moment the Sun crosses the local meridian —
/// the best time to point a dish toward the Sun for radio observations (quiet Sun ~3 GHz).
///
/// We generate one event per day covering ±1 month past + 1 year future.
pub async fn fetch(
    position: &Position,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    let now = Utc::now();
    let start_day = now - Duration::days(30);
    let end_day   = now + Duration::days(365);

    let mut events = Vec::new();
    let mut day = start_day;

    while day <= end_day {
        let (transit, az, el) = solar_transit(position.latitude, position.longitude, day);

        // Only emit if the Sun reaches at least 5° elevation at transit
        // (polar regions or deep winter can produce below-horizon transits)
        if el > 5.0 {
            let event_start = transit - Duration::minutes(30);
            let event_end   = transit + Duration::minutes(30);

            events.push(Event {
                id: Uuid::new_v4(),
                title: format!(
                    "Solar transit — {:.0}° Az  {:.0}° El  ({})",
                    az, el,
                    transit.format("%Y-%m-%d")
                ),
                category: Category::RadioAstronomical,
                event_type: EventType::Radio(RadioAstronomicalType::SolarTransit),
                start_time: event_start,
                end_time: event_end,
                sky_position: Some(SkyCoord { azimuth_deg: az, elevation_deg: el }),
                equipment: Some("Dish antenna + receiver 1–10 GHz".to_string()),
                source: "Solar Transit (calc. local)".to_string(),
                description: Some(format!(
                    "Sun crossing the meridian at {:.0}° elevation. \
                     Recommended frequencies: 1.4 GHz (HI), 2.8 GHz (F10.7 flux), 10 GHz.",
                    el
                )),
                freq_min_mhz: Some(1400.0),
                freq_max_mhz: Some(10000.0),
                listen_direction: Some(SkyCoord { azimuth_deg: az, elevation_deg: el }),
                sky_track: None,
            });
        }

        day = day + Duration::days(1);
    }

    Ok(events)
}

// ── Solar geometry (NOAA simplified algorithm, ~1 minute accuracy) ───────────

/// Returns (transit_time_utc, azimuth_deg, elevation_deg_at_transit) for the given day.
fn solar_transit(
    lat_deg: f64,
    lon_deg: f64,
    day: chrono::DateTime<Utc>,
) -> (chrono::DateTime<Utc>, f64, f64) {
    let jd = julian_day(day);
    let n  = jd - 2451545.0;

    // Mean longitude / mean anomaly
    let l_deg = (280.460 + 0.9856474 * n) % 360.0;
    let g_deg = (357.528 + 0.9856003 * n) % 360.0;
    let g_rad = g_deg.to_radians();

    // Ecliptic longitude
    let lambda_deg = l_deg + 1.915 * g_rad.sin() + 0.020 * (2.0 * g_rad).sin();
    let lambda_rad = lambda_deg.to_radians();

    // Obliquity of ecliptic
    let eps_deg = 23.439 - 0.0000004 * n;
    let eps_rad = eps_deg.to_radians();

    // Right ascension and declination
    let ra_rad  = (eps_rad.cos() * lambda_rad.sin()).atan2(lambda_rad.cos());
    let dec_rad = (eps_rad.sin() * lambda_rad.sin()).asin();

    // Equation of time (minutes)
    let eot = 4.0 * (l_deg - 0.0057183 - ra_rad.to_degrees() + equation_of_time_correction(n));

    // Solar noon in decimal hours UTC
    let solar_noon_utc = 12.0 - (lon_deg / 15.0) - (eot / 60.0);

    // Convert to DateTime
    let noon_secs = (solar_noon_utc * 3600.0) as i64;
    let midnight = Utc
        .with_ymd_and_hms(day.year(), day.month(), day.day(), 0, 0, 0)
        .unwrap();
    let transit = midnight + Duration::seconds(noon_secs.max(0).min(86399));

    // Elevation at transit = 90° - |lat - dec|
    let lat_rad = lat_deg.to_radians();
    let el_rad = (PI / 2.0) - (lat_rad - dec_rad).abs();
    let el_deg = el_rad.to_degrees().clamp(-90.0, 90.0);

    // Azimuth at transit is 180° (South) in northern hemisphere, 0° in southern
    let az = if lat_deg >= 0.0 { 180.0 } else { 0.0 };

    (transit, az, el_deg)
}

/// Julian Day Number from a UTC DateTime.
fn julian_day(dt: chrono::DateTime<Utc>) -> f64 {
    let y = dt.year() as f64;
    let m = dt.month() as f64;
    let d = dt.day() as f64
        + dt.hour() as f64 / 24.0
        + dt.minute() as f64 / 1440.0
        + dt.second() as f64 / 86400.0;

    let (y, m) = if m <= 2.0 { (y - 1.0, m + 12.0) } else { (y, m) };
    let a = (y / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + d + b - 1524.5
}

/// Small correction term for the equation of time.
fn equation_of_time_correction(n: f64) -> f64 {
    // Longitude of perihelion
    let w_deg = 282.9404 + 4.70935e-5 * n;
    let w_rad = w_deg.to_radians();
    // Eccentricity
    let e = 0.016709 - 1.151e-9 * n;
    let g_deg = (357.528 + 0.9856003 * n) % 360.0;
    let g_rad = g_deg.to_radians();
    // Approximate correction in degrees
    -e * 180.0 / PI * g_rad.sin() + w_rad.to_degrees()
}
