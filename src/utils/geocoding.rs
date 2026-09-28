/// Automatic lat/lon resolution utilities.
///
/// Resolution order when a city name is provided:
///   1. Nominatim (OpenStreetMap) geocoding
///
/// Resolution order when no name is provided:
///   1. System GPS (via `navigator.geolocation` — desktop: not available → skipped)
///   2. IP geolocation via ip-api.com (free, no key required)

#[derive(Debug)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
    /// Human-readable description of the resolved location (city, country …)
    pub display_name: String,
}

/// Error type for geocoding failures.
#[derive(Debug, Clone)]
pub enum GeoError {
    /// No name was given and no automatic method succeeded.
    #[allow(dead_code)]
    NoLocationFound,
    /// Network or HTTP error.
    NetworkError(String),
    /// The geocoding service returned no result for the given query.
    NoResult,
}

impl std::fmt::Display for GeoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoError::NoLocationFound => write!(f, "no_location_found"),
            GeoError::NetworkError(e) => write!(f, "network_error: {e}"),
            GeoError::NoResult       => write!(f, "no_result"),
        }
    }
}

// ── Nominatim (name → lat/lon) ────────────────────────────────────────────────

/// Resolve a city/place name to coordinates using the Nominatim API.
/// Returns the first result if any.
pub fn geocode_name(name: &str) -> Result<LatLon, GeoError> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("AstroFeed/1.0 (cosmic-beacon)")
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .map_err(|e| GeoError::NetworkError(e.to_string()))?;

    let response = client
        .get("https://nominatim.openstreetmap.org/search")
        .query(&[("q", name), ("format", "json"), ("limit", "1")])
        .send()
        .map_err(|e| GeoError::NetworkError(e.to_string()))?;

    let json: serde_json::Value = response
        .json()
        .map_err(|e| GeoError::NetworkError(e.to_string()))?;

    let arr = json.as_array().ok_or(GeoError::NoResult)?;
    let first = arr.first().ok_or(GeoError::NoResult)?;

    let lat: f64 = first["lat"]
        .as_str()
        .and_then(|s| s.parse().ok())
        .ok_or(GeoError::NoResult)?;
    let lon: f64 = first["lon"]
        .as_str()
        .and_then(|s| s.parse().ok())
        .ok_or(GeoError::NoResult)?;
    let display_name = first["display_name"]
        .as_str()
        .unwrap_or(name)
        .to_string();

    Ok(LatLon { lat, lon, display_name })
}

// ── IP geolocation (no name → lat/lon) ───────────────────────────────────────

/// Resolve coordinates from the machine's public IP address using ip-api.com.
pub fn geocode_by_ip() -> Result<LatLon, GeoError> {
    let response = reqwest::blocking::Client::builder()
        .user_agent("AstroFeed/1.0 (cosmic-beacon)")
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .map_err(|e| GeoError::NetworkError(e.to_string()))?
        .get("http://ip-api.com/json/?fields=status,lat,lon,city,country")
        .send()
        .map_err(|e| GeoError::NetworkError(e.to_string()))?;

    let json: serde_json::Value = response
        .json()
        .map_err(|e| GeoError::NetworkError(e.to_string()))?;

    if json["status"].as_str() != Some("success") {
        return Err(GeoError::NoResult);
    }

    let lat = json["lat"].as_f64().ok_or(GeoError::NoResult)?;
    let lon = json["lon"].as_f64().ok_or(GeoError::NoResult)?;
    let city    = json["city"].as_str().unwrap_or("");
    let country = json["country"].as_str().unwrap_or("");
    let display_name = if city.is_empty() {
        country.to_string()
    } else {
        format!("{city}, {country}")
    };

    Ok(LatLon { lat, lon, display_name })
}
