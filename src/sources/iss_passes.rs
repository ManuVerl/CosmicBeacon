use crate::model::{
    event::{AstronomicalType, Category, Event, EventType, SkyCoord, SkyPoint},
    position::Position,
};
use chrono::{Duration, TimeZone, Utc};
use sgp4::{Elements, MinutesSinceEpoch};
use uuid::Uuid;

const CELESTRAK_ISS_URL: &str =
    "https://celestrak.org/NORAD/elements/gp.php?CATNR=25544&FORMAT=TLE";

/// Parse a TLE text body that is either:
///   3-line: "NAME\n1 …\n2 …"
///   2-line: "1 …\n2 …"  (no name line)
/// Returns (optional_name, tle_line1, tle_line2).
fn parse_tle_lines(text: &str) -> Option<(Option<String>, &str, &str)> {
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty()).collect();

    // Identify TLE lines by their leading digit ('1' or '2')
    let tle1_idx = lines.iter().position(|l| l.starts_with('1') && l.len() >= 69)?;
    let tle2_idx = lines.iter().position(|l| l.starts_with('2') && l.len() >= 69)?;

    if tle2_idx <= tle1_idx {
        return None; // line 2 must come after line 1
    }

    let name = if tle1_idx > 0 {
        Some(lines[tle1_idx - 1].to_string())
    } else {
        None
    };

    Some((name, lines[tle1_idx], lines[tle2_idx]))
}

/// Fetches ISS TLE from Celestrak and computes passes over the given position
/// for the next 10 days.
pub async fn fetch(
    position: &Position,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    // Download TLE (two-line element set)
    // CelesTrak requires a User-Agent header; requests without one may get a 403.
    let client = reqwest::Client::builder()
        .user_agent("CosmicBeacon/0.2.0 (contact: cosmic-beacon@users)")
        .build()?;
    let tle_text = client.get(CELESTRAK_ISS_URL).send().await?.text().await?;

    let (name, line1, line2) = parse_tle_lines(&tle_text)
        .ok_or("Could not locate TLE lines in Celestrak response")?;

    let elements = Elements::from_tle(name, line1.as_bytes(), line2.as_bytes())?;

    let constants = sgp4::Constants::from_elements(&elements)?;

    let lat_rad = position.latitude.to_radians();
    let lon_rad = position.longitude.to_radians();
    let alt_km = 0.0_f64; // observer altitude above sea level (km)

    let now = Utc::now();
    let search_end = now + Duration::days(10);

    // Step through time in 60-second increments looking for passes above 10°
    let mut events: Vec<Event> = Vec::new();
    let mut in_pass = false;
    let mut pass_start = now;
    let mut max_elevation = 0.0_f64;
    let mut max_az = 0.0_f64;
    let mut track: Vec<SkyPoint> = Vec::new();

    let step_secs = 60i64;
    let mut t = now;

    while t < search_end {
        // Minutes since TLE epoch
        let epoch_dt = tle_epoch_to_datetime(&elements);
        let minutes = (t - epoch_dt).num_seconds() as f64 / 60.0;

        if let Ok(prediction) = constants.propagate(MinutesSinceEpoch(minutes)) {
            let el = elevation_deg(
                lat_rad,
                lon_rad,
                alt_km,
                prediction.position,
                t,
            );

            if el > 10.0 {
                let az = azimuth_deg(lat_rad, lon_rad, prediction.position, t);
                if !in_pass {
                    in_pass = true;
                    pass_start = t;
                    max_elevation = el;
                    max_az = az;
                    track.clear();
                } else if el > max_elevation {
                    max_elevation = el;
                    max_az = az;
                }
                track.push(SkyPoint { time: t, azimuth_deg: az, elevation_deg: el });
            } else if in_pass {
                in_pass = false;
                let pass_end = t;

                events.push(Event {
                    id: Uuid::new_v4(),
                    title: format!(
                        "ISS Flyover — max elev. {:.0}°  ({})",
                        max_elevation,
                        pass_start.format("%Y-%m-%d %H:%M UTC")
                    ),
                    category: Category::Astronomical,
                    event_type: EventType::Astronomical(AstronomicalType::IssFlyover),
                    start_time: pass_start,
                    end_time: pass_end,
                    sky_position: Some(SkyCoord {
                        azimuth_deg: max_az,
                        elevation_deg: max_elevation,
                    }),
                    equipment: Some("Naked eye".to_string()),
                    source: "ISS Passes (Celestrak TLE)".to_string(),
                    description: Some(format!(
                        "Duration ~{}min  |  Max elev. {:.0}°",
                        (pass_end - pass_start).num_minutes(),
                        max_elevation
                    )),
                    freq_min_mhz: None,
                    freq_max_mhz: None,
                    listen_direction: None,
                    sky_track: Some(std::mem::take(&mut track)),
                });
            }
        }

        t = t + Duration::seconds(step_secs);
    }

    Ok(events)
}

// ── Astronomical geometry helpers ────────────────────────────────────────────

/// Return the TLE epoch as a UTC DateTime.
/// sgp4 v2 stores the epoch as a `chrono::NaiveDateTime`; we attach UTC.
fn tle_epoch_to_datetime(elements: &Elements) -> chrono::DateTime<Utc> {
    Utc.from_utc_datetime(&elements.datetime)
}

/// Earth's equatorial radius in km.
const RE: f64 = 6378.137;

/// Convert ECI (Earth-Centred Inertial) position [km] to observer elevation angle [degrees].
fn elevation_deg(
    obs_lat_rad: f64,
    obs_lon_rad: f64,
    obs_alt_km: f64,
    eci_km: [f64; 3],
    t: chrono::DateTime<Utc>,
) -> f64 {
    let (rx, ry, rz) = eci_to_ecef(eci_km, t);
    let (ox, oy, oz) = geodetic_to_ecef(obs_lat_rad, obs_lon_rad, obs_alt_km);

    // Range vector (observer → satellite)
    let dx = rx - ox;
    let dy = ry - oy;
    let dz = rz - oz;
    let range = (dx * dx + dy * dy + dz * dz).sqrt();

    // South, East, Zenith unit vectors at observer
    let sin_lat = obs_lat_rad.sin();
    let cos_lat = obs_lat_rad.cos();
    let sin_lon = obs_lon_rad.sin();
    let cos_lon = obs_lon_rad.cos();

    // Zenith unit vector
    let zx = cos_lat * cos_lon;
    let zy = cos_lat * sin_lon;
    let zz = sin_lat;

    // Elevation = asin(dot(range_unit, zenith))
    let dot = (dx * zx + dy * zy + dz * zz) / range;
    dot.asin().to_degrees()
}

/// Azimuth of the satellite as seen from the observer [degrees, 0=North, clockwise].
fn azimuth_deg(
    obs_lat_rad: f64,
    obs_lon_rad: f64,
    eci_km: [f64; 3],
    t: chrono::DateTime<Utc>,
) -> f64 {
    let (rx, ry, rz) = eci_to_ecef(eci_km, t);
    let (ox, oy, oz) = geodetic_to_ecef(obs_lat_rad, obs_lon_rad, 0.0);

    let dx = rx - ox;
    let dy = ry - oy;
    let dz = rz - oz;

    let sin_lat = obs_lat_rad.sin();
    let cos_lat = obs_lat_rad.cos();
    let sin_lon = obs_lon_rad.sin();
    let cos_lon = obs_lon_rad.cos();

    // South unit vector
    let sx = sin_lat * cos_lon;
    let sy = sin_lat * sin_lon;
    let sz = -cos_lat;

    // East unit vector
    let ex = -sin_lon;
    let ey = cos_lon;
    // ez = 0

    let south = dx * sx + dy * sy + dz * sz;
    let east = dx * ex + dy * ey; // + dz*0

    let az = (-south).atan2(east).to_degrees();
    (az + 360.0) % 360.0
}

/// Rotate ECI to ECEF using Greenwich Sidereal Time.
fn eci_to_ecef(eci: [f64; 3], t: chrono::DateTime<Utc>) -> (f64, f64, f64) {
    let gst = greenwich_sidereal_time(t);
    let cos_gst = gst.cos();
    let sin_gst = gst.sin();
    let x = eci[0] * cos_gst + eci[1] * sin_gst;
    let y = -eci[0] * sin_gst + eci[1] * cos_gst;
    let z = eci[2];
    (x, y, z)
}

/// Greenwich Apparent Sidereal Time in radians (simplified).
fn greenwich_sidereal_time(t: chrono::DateTime<Utc>) -> f64 {
    use std::f64::consts::TAU;
    let j2000 = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let days = (t - j2000).num_seconds() as f64 / 86_400.0;
    // GMST in radians (simplified formula, good to ~0.1°)
    let theta: f64 = 280.46061837 + 360.98564736629 * days;
    theta.to_radians().rem_euclid(TAU)
}

/// Convert geodetic coordinates to ECEF [km].
fn geodetic_to_ecef(lat_rad: f64, lon_rad: f64, alt_km: f64) -> (f64, f64, f64) {
    let r = RE + alt_km;
    let x = r * lat_rad.cos() * lon_rad.cos();
    let y = r * lat_rad.cos() * lon_rad.sin();
    let z = r * lat_rad.sin();
    (x, y, z)
}
