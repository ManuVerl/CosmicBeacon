use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};
use std::f64::consts::PI;

// ── Compass helpers ────────────────────────────────────────────────────────────

/// Return a 16-point compass abbreviation key for a given azimuth (0° = North, clockwise).
/// Returns an i18n key like "compass.N", "compass.NNE", …
pub fn azimuth_to_compass_key(az: f64) -> &'static str {
    let az = az.rem_euclid(360.0);
    let idx = ((az + 11.25) / 22.5) as usize % 16;
    [
        "compass.N",  "compass.NNE", "compass.NE",  "compass.ENE",
        "compass.E",  "compass.ESE", "compass.SE",  "compass.SSE",
        "compass.S",  "compass.SSW", "compass.SW",  "compass.WSW",
        "compass.W",  "compass.WNW", "compass.NW",  "compass.NNW",
    ][idx]
}

/// Julian Day Number from a UTC DateTime.
pub fn julian_day(dt: DateTime<Utc>) -> f64 {
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

/// Lunar phase + position information.
#[derive(Debug, Clone)]
pub struct MoonPhaseInfo {
    /// Phase value from 0.0 to 1.0 (0.0 = New Moon, 0.25 = First Quarter, 0.5 = Full Moon, 0.75 = Last Quarter)
    pub phase: f64,
    /// Fraction of lunar disc illuminated (0.0 to 1.0)
    pub illumination: f64,
    /// Age in days within lunar cycle (~29.53 days)
    #[allow(dead_code)]
    pub age_days: f64,
    /// Current altitude above/below horizon in degrees
    pub altitude_deg: f64,
    /// Current azimuth in degrees (0° = North, clockwise)
    pub azimuth_deg: f64,
    /// Moonrise time (local), None during polar conditions
    pub rise_local: Option<DateTime<Local>>,
    /// Moonset time (local), None during polar conditions
    pub set_local: Option<DateTime<Local>>,
}

// ── Low-precision lunar position (Jean Meeus, "Astronomical Algorithms" ch.47) ─

/// Compute geocentric ecliptic longitude and latitude of the Moon (degrees).
fn moon_ecliptic_coords(jd: f64) -> (f64, f64) {
    let t = (jd - 2451545.0) / 36525.0;

    // Fundamental arguments (degrees)
    let lp = (218.3164477 + 481267.88123421 * t
        - 0.0015786 * t * t
        + t * t * t / 538841.0
        - t * t * t * t / 65194000.0).rem_euclid(360.0);

    let d  = (297.8501921 + 445267.1114034 * t
        - 0.0018819 * t * t
        + t * t * t / 545868.0
        - t * t * t * t / 113065000.0).rem_euclid(360.0);

    let m  = (357.5291092 + 35999.0502909 * t
        - 0.0001536 * t * t
        + t * t * t / 24490000.0).rem_euclid(360.0);

    let mp = (134.9633964 + 477198.8675055 * t
        + 0.0087414 * t * t
        + t * t * t / 69699.0
        - t * t * t * t / 14712000.0).rem_euclid(360.0);

    let f  = (93.2720950 + 483202.0175233 * t
        - 0.0036539 * t * t
        - t * t * t / 3526000.0
        + t * t * t * t / 863310000.0).rem_euclid(360.0);

    // Convert to radians
    let (_lp_r, d_r, m_r, mp_r, f_r) = (
        lp.to_radians(), d.to_radians(), m.to_radians(),
        mp.to_radians(), f.to_radians(),
    );

    // Leading longitude terms (degrees)
    let mut lon = 6.288774 * mp_r.sin()
        + 1.274027 * (2.0 * d_r - mp_r).sin()
        + 0.658314 * (2.0 * d_r).sin()
        + 0.213618 * (2.0 * mp_r).sin()
        - 0.185116 * m_r.sin()
        - 0.114332 * (2.0 * f_r).sin()
        + 0.058793 * (2.0 * d_r - 2.0 * mp_r).sin()
        + 0.057066 * (2.0 * d_r - m_r - mp_r).sin()
        + 0.053322 * (2.0 * d_r + mp_r).sin()
        + 0.045758 * (2.0 * d_r - m_r).sin()
        - 0.040923 * (m_r - mp_r).sin()
        - 0.034720 * d_r.sin()
        - 0.030383 * (m_r + mp_r).sin();
    lon = lp + lon;

    // Leading latitude terms (degrees)
    let lat = 5.128122 * f_r.sin()
        + 0.280602 * (mp_r + f_r).sin()
        + 0.277693 * (mp_r - f_r).sin()
        + 0.173237 * (2.0 * d_r - f_r).sin()
        + 0.055413 * (2.0 * d_r - mp_r + f_r).sin()
        + 0.046272 * (2.0 * d_r - mp_r - f_r).sin()
        + 0.032573 * (2.0 * d_r + f_r).sin()
        + 0.017198 * (2.0 * mp_r + f_r).sin()
        + 0.009267 * (2.0 * d_r + mp_r - f_r).sin()
        + 0.008823 * (2.0 * mp_r - f_r).sin();

    (lon.rem_euclid(360.0), lat)
}

/// Convert geocentric ecliptic (lon, lat) degrees to equatorial (RA hours, Dec degrees) at epoch JD.
fn ecliptic_to_equatorial(lon_deg: f64, lat_deg: f64, jd: f64) -> (f64, f64) {
    let t = (jd - 2451545.0) / 36525.0;
    let eps = (23.439291111 - 0.013004167 * t).to_radians();

    let lon_r = lon_deg.to_radians();
    let lat_r = lat_deg.to_radians();

    let sin_dec = lat_r.sin() * eps.cos() + lat_r.cos() * eps.sin() * lon_r.sin();
    let dec = sin_dec.asin();

    let ra = (lon_r.sin() * eps.cos() - lat_r.tan() * eps.sin())
        .atan2(lon_r.cos());

    (ra.to_degrees().rem_euclid(360.0) / 15.0, dec.to_degrees())
}

/// Convert equatorial (RA hours, Dec degrees) to horizontal (azimuth, altitude) degrees.
/// `lst_hours` = Local Sidereal Time in hours, `lat_deg` = observer latitude.
fn equatorial_to_horizontal(ra_hours: f64, dec_deg: f64, lst_hours: f64, lat_deg: f64) -> (f64, f64) {
    let ha  = ((lst_hours - ra_hours) * 15.0).to_radians();
    let dec = dec_deg.to_radians();
    let lat = lat_deg.to_radians();

    let sin_alt = lat.sin() * dec.sin() + lat.cos() * dec.cos() * ha.cos();
    let alt = sin_alt.asin().to_degrees();

    let cos_az = (dec.sin() - lat.sin() * sin_alt) / (lat.cos() * alt.to_radians().cos());
    let cos_az = cos_az.clamp(-1.0, 1.0);
    let az_raw = cos_az.acos().to_degrees();
    let az = if ha.sin() < 0.0 { az_raw } else { 360.0 - az_raw };

    (az, alt)
}

/// Compute Local Sidereal Time (hours) for a given UTC datetime and longitude (degrees).
fn local_sidereal_time(dt: DateTime<Utc>, lon_deg: f64) -> f64 {
    let jd = julian_day(dt);
    let t  = (jd - 2451545.0) / 36525.0;
    let gmst_deg = 280.46061837
        + 360.98564736629 * (jd - 2451545.0)
        + 0.000387933 * t * t
        - t * t * t / 38710000.0;
    ((gmst_deg + lon_deg) / 15.0).rem_euclid(24.0)
}

/// Compute the moon's topocentric altitude and azimuth at the given UTC time and observer position.
pub fn moon_horizontal_position(dt: DateTime<Utc>, lat_deg: f64, lon_deg: f64) -> (f64, f64) {
    let jd = julian_day(dt);
    let (lon, lat) = moon_ecliptic_coords(jd);
    let (ra, dec) = ecliptic_to_equatorial(lon, lat, jd);
    let lst = local_sidereal_time(dt, lon_deg);
    equatorial_to_horizontal(ra, dec, lst, lat_deg)
}

/// Estimate moonrise and moonset for the current UTC day at the observer position.
/// Uses a simple iterative step-search at 10-minute resolution over 48 hours.
pub fn moon_rise_set(dt: DateTime<Utc>, lat_deg: f64, lon_deg: f64)
    -> (Option<DateTime<Local>>, Option<DateTime<Local>>)
{
    // Start search from previous midnight UTC
    let midnight = Utc
        .with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 0, 0, 0)
        .unwrap();

    let step_mins: i64 = 10;
    let steps = (48 * 60) / step_mins; // search 48 hours to catch edge cases

    let alt_at = |offset_mins: i64| -> f64 {
        let t = midnight + Duration::minutes(offset_mins);
        moon_horizontal_position(t, lat_deg, lon_deg).1
    };

    let mut rise: Option<DateTime<Local>> = None;
    let mut set:  Option<DateTime<Local>> = None;

    let mut prev_alt = alt_at(0);
    let mut i = step_mins;
    while i <= steps * step_mins {
        let curr_alt = alt_at(i);
        if prev_alt < 0.0 && curr_alt >= 0.0 && rise.is_none() {
            // Refine crossing by linear interpolation
            let frac = (-prev_alt) / (curr_alt - prev_alt);
            let rise_mins = (i - step_mins) as f64 + frac * step_mins as f64;
            let rise_utc = midnight + Duration::seconds((rise_mins * 60.0) as i64);
            rise = Some(rise_utc.with_timezone(&Local));
        }
        if prev_alt >= 0.0 && curr_alt < 0.0 && set.is_none() {
            let frac = prev_alt / (prev_alt - curr_alt);
            let set_mins = (i - step_mins) as f64 + frac * step_mins as f64;
            let set_utc = midnight + Duration::seconds((set_mins * 60.0) as i64);
            set = Some(set_utc.with_timezone(&Local));
        }
        // Stop early once both found
        if rise.is_some() && set.is_some() { break; }
        prev_alt = curr_alt;
        i += step_mins;
    }

    (rise, set)
}

/// Full moon info with observer position (altitude, azimuth, rise, set all computed).
pub fn calculate_moon_info(dt: DateTime<Utc>, lat_deg: f64, lon_deg: f64) -> MoonPhaseInfo {
    let jd = julian_day(dt);
    let synodic_month = 29.53058867;
    let known_new_moon = 2451549.760;

    let days_since = jd - known_new_moon;
    let phase = (days_since / synodic_month).rem_euclid(1.0);
    let age_days = phase * synodic_month;
    let illumination = (1.0 - (2.0 * PI * phase).cos()) / 2.0;

    let (azimuth_deg, altitude_deg) = moon_horizontal_position(dt, lat_deg, lon_deg);
    let (rise_local, set_local) = moon_rise_set(dt, lat_deg, lon_deg);

    MoonPhaseInfo { phase, illumination, age_days, altitude_deg, azimuth_deg, rise_local, set_local }
}

/// Solar ephemeris for a given UTC datetime.
/// Returns (declination_rad, equation_of_time_minutes, right_ascension_rad).
/// Algorithm: NOAA / Jean Meeus low-precision, accurate to ~0.01°.
fn sun_ephemeris(dt: DateTime<Utc>) -> (f64, f64, f64) {
    let jd = julian_day(dt);
    // Julian centuries from J2000.0
    let t = (jd - 2451545.0) / 36525.0;

    // Geometric mean longitude of the Sun (degrees), referred to the mean equinox of the date
    let l0 = (280.46646 + 36000.76983 * t + 0.0003032 * t * t).rem_euclid(360.0);

    // Mean anomaly of the Sun (degrees)
    let m_deg = (357.52911 + 35999.05029 * t - 0.0001537 * t * t).rem_euclid(360.0);
    let m_rad = m_deg.to_radians();

    // Equation of centre (degrees)
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * m_rad.sin()
          + (0.019993 - 0.000101 * t) * (2.0 * m_rad).sin()
          +  0.000289 * (3.0 * m_rad).sin();

    // Sun's true longitude (degrees)
    let sun_lon = l0 + c;
    // Sun's true anomaly (degrees, unused but conceptually present)

    // Apparent longitude (correct for aberration, degrees)
    let omega = 125.04 - 1934.136 * t;
    let apparent_lon_deg = sun_lon - 0.00569 - 0.00478 * omega.to_radians().sin();
    let apparent_lon_rad = apparent_lon_deg.to_radians();

    // Mean obliquity of the ecliptic (degrees)
    let eps0 = 23.0 + 26.0 / 60.0 + 21.448 / 3600.0
        - (46.8150 / 3600.0) * t
        - (0.00059 / 3600.0) * t * t
        + (0.001813 / 3600.0) * t * t * t;
    // Corrected obliquity
    let eps_rad = (eps0 + 0.00256 * omega.to_radians().cos()).to_radians();

    // Declination (radians)
    let dec_rad = (eps_rad.sin() * apparent_lon_rad.sin()).asin();

    // Right ascension (radians, unwrapped to [0, 2π))
    let ra_rad = (eps_rad.cos() * apparent_lon_rad.sin())
        .atan2(apparent_lon_rad.cos())
        .rem_euclid(2.0 * PI);

    // Equation of Time (minutes) — NOAA spreadsheet formula (Meeus, ch.28)
    // e = orbital eccentricity of the Earth
    let e = 0.016708634 - t * (0.000042037 + t * 0.0000001267);
    // y = tan²(ε/2)
    let y = (eps_rad / 2.0).tan().powi(2);
    let l0_rad = l0.to_radians();
    // EoT in radians of time, then converted to minutes via ×4 (1 radian = 4 × (180/π) minutes)
    let eot_rad = y * (2.0 * l0_rad).sin()
        - 2.0 * e * m_rad.sin()
        + 4.0 * e * y * m_rad.sin() * (2.0 * l0_rad).cos()
        - 0.5 * y * y * (4.0 * l0_rad).sin()
        - 1.25 * e * e * (2.0 * m_rad).sin();
    let eot_min = 4.0 * eot_rad.to_degrees();

    (dec_rad, eot_min, ra_rad)
}

/// Sun's azimuth and altitude (degrees, 0° = North clockwise) at an arbitrary UTC time.
pub fn sun_horizontal_position(dt: DateTime<Utc>, lat_deg: f64, lon_deg: f64) -> (f64, f64) {
    let (dec_rad, _eot, ra_rad) = sun_ephemeris(dt);
    let lst = local_sidereal_time(dt, lon_deg);
    equatorial_to_horizontal(ra_rad.to_degrees() / 15.0, dec_rad.to_degrees(), lst, lat_deg)
}


/// Sunrise and sunset times for a given day and observer coordinates.
#[derive(Debug, Clone)]
pub struct SunTimes {
    pub sunrise_local: Option<DateTime<Local>>,
    pub sunset_local: Option<DateTime<Local>>,
    #[allow(dead_code)]
    pub transit_local: Option<DateTime<Local>>,
    /// Solar altitude in degrees right now (negative = below horizon)
    pub current_elevation: f64,
    /// Solar azimuth in degrees right now (0° = North, clockwise)
    pub current_azimuth: f64,
    #[allow(dead_code)]
    pub is_daylight: bool,
    /// Fraction of daylight elapsed (0.0 at sunrise, 1.0 at sunset, None if night or polar)
    pub daylight_progress: Option<f32>,
    /// Total daylight duration in seconds (None under polar conditions)
    pub daylight_duration_secs: Option<i64>,
}

/// Calculate Sun times and current position for observer location on the given local day.
pub fn calculate_sun_times(lat_deg: f64, lon_deg: f64, now_local: DateTime<Local>) -> SunTimes {
    let now_utc = now_local.with_timezone(&Utc);

    // Midnight UTC of the current calendar day
    let midnight_utc = Utc
        .with_ymd_and_hms(now_utc.year(), now_utc.month(), now_utc.day(), 0, 0, 0)
        .unwrap();

    let (dec_rad, eot_min, _ra_rad) = sun_ephemeris(now_utc);
    let lat_rad = lat_deg.to_radians();

    // Solar noon in UTC hours: 12h minus longitude correction minus EoT correction
    // lon_deg/15 converts degrees of longitude to hours; EoT is in minutes → /60
    let solar_noon_utc_h = 12.0 - lon_deg / 15.0 - eot_min / 60.0;

    let transit_utc = midnight_utc + Duration::seconds((solar_noon_utc_h * 3600.0) as i64);
    let transit_local: DateTime<Local> = transit_utc.with_timezone(&Local);

    // Hour-angle at sunrise/sunset: cos(HA) = (cos(90.833°) − sin(lat)·sin(dec)) / (cos(lat)·cos(dec))
    // 90.833° accounts for atmospheric refraction (0.5667°) + solar disc radius (0.2667°)
    let cos_ha0 = ((-90.833_f64).to_radians().cos() - lat_rad.sin() * dec_rad.sin())
        / (lat_rad.cos() * dec_rad.cos());

    let (sunrise_local, sunset_local) = if cos_ha0 > 1.0 {
        // Polar night — Sun never rises
        (None, None)
    } else if cos_ha0 < -1.0 {
        // Midnight sun — Sun never sets
        (None, None)
    } else {
        let ha0_h = cos_ha0.acos().to_degrees() / 15.0; // hours of half-day
        let rise_utc = midnight_utc + Duration::seconds(((solar_noon_utc_h - ha0_h) * 3600.0) as i64);
        let set_utc  = midnight_utc + Duration::seconds(((solar_noon_utc_h + ha0_h) * 3600.0) as i64);
        (Some(rise_utc.with_timezone(&Local)), Some(set_utc.with_timezone(&Local)))
    };

    // ── Current altitude and azimuth ─────────────────────────────────────────
    // Hour angle at current UTC instant (radians)
    // HA = LST − RA, but for the Sun we use the solar-noon shortcut:
    // HA_hours = t_utc_hours − solar_noon_utc_h  (positive = Sun west of meridian)
    let current_utc_h = now_utc.hour() as f64
        + now_utc.minute() as f64 / 60.0
        + now_utc.second() as f64 / 3600.0;
    let ha_rad = ((current_utc_h - solar_noon_utc_h) * 15.0).to_radians();

    // sin(alt) = sin(lat)·sin(dec) + cos(lat)·cos(dec)·cos(HA)
    let sin_alt = lat_rad.sin() * dec_rad.sin()
        + lat_rad.cos() * dec_rad.cos() * ha_rad.cos();
    let sin_alt = sin_alt.clamp(-1.0, 1.0);
    let current_elevation = sin_alt.asin().to_degrees();

    // cos(az) = (sin(dec) − sin(lat)·sin(alt)) / (cos(lat)·cos(alt))
    // cos(alt) = sqrt(1 − sin²(alt)) — avoids re-computing trig
    let cos_alt = (1.0 - sin_alt * sin_alt).sqrt().max(1e-10);
    let cos_az = ((dec_rad.sin() - lat_rad.sin() * sin_alt) / (lat_rad.cos() * cos_alt))
        .clamp(-1.0, 1.0);
    let az_raw = cos_az.acos().to_degrees();
    // Quadrant: HA > 0 → Sun is west → azimuth > 180°
    let current_azimuth = if ha_rad.sin() > 0.0 { 360.0 - az_raw } else { az_raw };

    let is_daylight = current_elevation > -0.8333;

    let daylight_duration_secs = match (&sunrise_local, &sunset_local) {
        (Some(rise), Some(set)) => Some((*set - *rise).num_seconds()),
        _ => None,
    };

    let daylight_progress = match (&sunrise_local, &sunset_local) {
        (Some(rise), Some(set)) => {
            let total = (*set - *rise).num_seconds() as f32;
            let elapsed = (now_local - *rise).num_seconds() as f32;
            if total > 0.0 { Some((elapsed / total).clamp(0.0, 1.0)) } else { None }
        }
        _ => None,
    };

    SunTimes {
        sunrise_local,
        sunset_local,
        transit_local: Some(transit_local),
        current_elevation,
        current_azimuth,
        is_daylight,
        daylight_progress,
        daylight_duration_secs,
    }
}
