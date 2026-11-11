use crate::model::{
    event::{Category, Event, EventType, RadioAstronomicalType, SkyCoord},
    position::Position,
};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc};
use uuid::Uuid;

/// Galactic centre coordinates (J2000):
///   RA  = 17h 45m 40.04s  → 266.405°
///   Dec = −29° 00' 28.1"  → −29.008°
const GC_RA_DEG:  f64 = 266.405;
const GC_DEC_DEG: f64 = -29.008;

/// Minimum elevation (degrees) above the horizon for the transit to be emitted.
const MIN_ELEVATION_DEG: f64 = 10.0;

/// Compute the Local Sidereal Time in degrees for a given UTC instant and longitude.
fn lst_deg(dt: chrono::DateTime<Utc>, lon_deg: f64) -> f64 {
    let jd = julian_day(dt);
    let d  = jd - 2451545.0;                          // days from J2000.0
    let gmst_deg = (280.46061837 + 360.98564736629 * d) % 360.0;
    (gmst_deg + lon_deg).rem_euclid(360.0)
}

/// Convert equatorial (RA/Dec) to horizontal (Az/El) coordinates.
/// Returns (azimuth_deg, elevation_deg).
fn eq_to_horiz(ra_deg: f64, dec_deg: f64, lat_deg: f64, lst: f64) -> (f64, f64) {
    let ha = (lst - ra_deg).rem_euclid(360.0); // hour angle in degrees
    let ha_rad  = ha.to_radians();
    let dec_rad = dec_deg.to_radians();
    let lat_rad = lat_deg.to_radians();

    let sin_el = dec_rad.sin() * lat_rad.sin()
        + dec_rad.cos() * lat_rad.cos() * ha_rad.cos();
    let el_rad = sin_el.asin();

    let cos_az = (dec_rad.sin() - el_rad.sin() * lat_rad.sin())
        / (el_rad.cos() * lat_rad.cos());
    let cos_az = cos_az.clamp(-1.0, 1.0);
    let az_raw = cos_az.acos().to_degrees();
    let az = if ha_rad.sin() > 0.0 { 360.0 - az_raw } else { az_raw };

    (az, el_rad.to_degrees())
}

/// Find the UTC time (within `day`) when the Galactic Centre transits the local meridian.
/// Returns None if the transit falls outside [0, 86400) seconds of that day.
fn meridian_transit(lat_deg: f64, lon_deg: f64, day: chrono::DateTime<Utc>)
    -> Option<(chrono::DateTime<Utc>, f64, f64)>
{
    // Scan with 1-minute resolution; record sign change of hour angle
    let midnight = Utc
        .with_ymd_and_hms(day.year(), day.month(), day.day(), 0, 0, 0)
        .unwrap();

    let mut best: Option<chrono::DateTime<Utc>> = None;
    let mut prev_ha = f64::NAN;

    for min in 0..=1440_i64 {
        let t = midnight + Duration::minutes(min);
        let lst = lst_deg(t, lon_deg);
        let ha  = (lst - GC_RA_DEG).rem_euclid(360.0); // 0..360

        if !prev_ha.is_nan() {
            // Upper transit: HA crosses 0 (or 360→0) from the positive side
            let prev_hi = prev_ha > 180.0;
            let cur_lo  = ha < 180.0;
            if prev_hi && cur_lo {
                best = Some(t - Duration::minutes(1));
                break;
            }
        }
        prev_ha = ha;
    }

    best.map(|t| {
        let lst = lst_deg(t, lon_deg);
        let (az, el) = eq_to_horiz(GC_RA_DEG, GC_DEC_DEG, lat_deg, lst);
        (t, az, el)
    })
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

/// Produces Milky Way (Galactic Centre) meridian transit events for the observer position.
///
/// The Galactic Centre (Sgr A*) is the strongest natural radio source in the sky and
/// produces a detectable continuum at 408 MHz–10 GHz. We emit one event per day covering
/// ±30 days past + 1 year future, but only when the transit elevation exceeds 10°.
pub async fn fetch(
    position: &Position,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    let now       = Utc::now();
    let start_day = now - Duration::days(30);
    let end_day   = now + Duration::days(365);

    let mut events = Vec::new();
    let mut day = start_day;

    while day <= end_day {
        if let Some((transit, az, el)) =
            meridian_transit(position.latitude, position.longitude, day)
        {
            if el > MIN_ELEVATION_DEG {
                let event_start = transit - Duration::minutes(45);
                let event_end   = transit + Duration::minutes(45);

                events.push(Event {
                    id: Uuid::new_v4(),
                    title: format!(
                        "Milky Way transit (Sgr A*) — {:.0}° Az  {:.0}° El  ({})",
                        az, el,
                        transit.format("%Y-%m-%d")
                    ),
                    category: Category::RadioAstronomical,
                    event_type: EventType::Radio(RadioAstronomicalType::MilkyWayTransit),
                    start_time: event_start,
                    end_time: event_end,
                    sky_position: Some(SkyCoord { azimuth_deg: az, elevation_deg: el }),
                    equipment: Some("Dish antenna + receiver 408 MHz – 10 GHz".to_string()),
                    source: "Milky Way Transit (calc. local)".to_string(),
                    description: Some(format!(
                        "Galactic Centre (Sgr A*) crossing the meridian at {:.0}° elevation. \
                         Recommended frequencies: 408 MHz (continuum), 1.42 GHz (HI 21 cm), \
                         4.8–10 GHz (thermal emission).",
                        el
                    )),
                    freq_min_mhz: Some(408.0),
                    freq_max_mhz: Some(10000.0),
                    listen_direction: Some(SkyCoord { azimuth_deg: az, elevation_deg: el }),
                    sky_track: None,
                });
            }
        }

        day = day + Duration::days(1);
    }

    Ok(events)
}
