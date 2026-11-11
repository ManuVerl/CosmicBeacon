use crate::model::{
    event::{Category, Event, EventType, RadioAstronomicalType, SkyCoord},
    position::Position,
};
use chrono::{Duration, NaiveDateTime, TimeZone, Utc};
use uuid::Uuid;

/// ARISS upcoming contacts RSS feed (no auth required).
const ARISS_RSS_URL: &str = "https://www.ariss.org/feed/";

/// Fetches upcoming ARISS ISS radio contact events from the RSS feed.
/// Each contact is when ISS crew talks to a school via amateur radio — 
/// anyone can listen on 145.800 MHz (downlink).
pub async fn fetch(
    _position: &Position,
) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    let text = reqwest::get(ARISS_RSS_URL).await?.text().await?;
    parse_rss(&text)
}

fn parse_rss(xml: &str) -> Result<Vec<Event>, Box<dyn std::error::Error + Send + Sync>> {
    use quick_xml::events::Event as XmlEvent;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut events: Vec<Event> = Vec::new();

    // Simple state machine: collect <title>, <description>, <pubDate> per <item>
    let mut in_item = false;
    let mut current_tag = String::new();
    let mut title_buf = String::new();
    let mut desc_buf = String::new();
    let mut date_buf = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => {
                let tag = std::str::from_utf8(e.name().as_ref()).unwrap_or("").to_string();
                match tag.as_str() {
                    "item" => {
                        in_item = true;
                        title_buf.clear();
                        desc_buf.clear();
                        date_buf.clear();
                    }
                    _ => { current_tag = tag; }
                }
            }
            Ok(XmlEvent::Text(e)) => {
                if in_item {
                    let text = e.unescape().unwrap_or_default().to_string();
                    match current_tag.as_str() {
                        "title"       => title_buf.push_str(&text),
                        "description" => desc_buf.push_str(&text),
                        "pubDate"     => date_buf.push_str(&text),
                        _ => {}
                    }
                }
            }
            Ok(XmlEvent::End(ref e)) => {
                let name_bytes = e.name();
                let tag = std::str::from_utf8(name_bytes.as_ref()).unwrap_or("");
                if tag == "item" && in_item {
                    in_item = false;
                    if let Some(ev) = build_ariss_event(&title_buf, &desc_buf, &date_buf) {
                        events.push(ev);
                    }
                }
                current_tag.clear();
            }
            Ok(XmlEvent::CData(e)) => {
                if in_item {
                    let text = String::from_utf8_lossy(e.as_ref()).to_string();
                    match current_tag.as_str() {
                        "title"       => title_buf.push_str(&text),
                        "description" => desc_buf.push_str(&text),
                        "pubDate"     => date_buf.push_str(&text),
                        _ => {}
                    }
                }
            }
            Ok(XmlEvent::Eof) => break,
            Err(e) => {
                log::warn!("ARISS RSS parse error: {}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    events.sort_by_key(|e| e.start_time);
    Ok(events)
}

/// Attempt to build an Event from a parsed RSS item.
fn build_ariss_event(title: &str, description: &str, pub_date: &str) -> Option<Event> {
    // Try to parse RFC 2822 date from pubDate
    // e.g. "Mon, 07 Sep 2026 14:00:00 +0000"
    let dt = parse_rfc2822(pub_date)?;

    let now = Utc::now();
    let window_start = now - Duration::days(30);
    let window_end   = now + Duration::days(365);

    if dt < window_start || dt > window_end {
        return None;
    }

    // Strip HTML tags from description (very basic)
    let clean_desc = strip_html(description);

    // Contact duration is typically 10 minutes
    let end = dt + Duration::minutes(10);

    Some(Event {
        id: Uuid::new_v4(),
        title: format!("ISS Radio ARISS — {}", sanitize_title(title)),
        category: Category::RadioAstronomical,
        event_type: EventType::Radio(RadioAstronomicalType::IssRadio),
        start_time: dt,
        end_time: end,
        sky_position: None, // would need TLE calculation for exact pass
        equipment: Some("FM receiver or SDR — 145.800 MHz (ISS downlink)".to_string()),
        source: "ISS Radio (ARISS RSS)".to_string(),
        description: Some(if clean_desc.is_empty() {
            "ARISS radio contact between the ISS and a school. Free listening on 145.800 MHz."
                .to_string()
        } else {
            format!("{} | Downlink: 145.800 MHz", truncate(&clean_desc, 200))
        }),
        freq_min_mhz: Some(145.8),
        freq_max_mhz: Some(145.8),
        listen_direction: Some(SkyCoord { azimuth_deg: 0.0, elevation_deg: 0.0 }),
        sky_track: None,
    })
}

/// Parse RFC 2822 date string to UTC DateTime.
fn parse_rfc2822(s: &str) -> Option<chrono::DateTime<Utc>> {
    // Try common formats
    // "Mon, 07 Sep 2026 14:00:00 +0000"
    let formats = [
        "%a, %d %b %Y %H:%M:%S %z",
        "%d %b %Y %H:%M:%S %z",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%d %H:%M:%S",
    ];
    for fmt in &formats {
        if let Ok(dt) = chrono::DateTime::parse_from_str(s.trim(), fmt) {
            return Some(dt.with_timezone(&Utc));
        }
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s.trim(), fmt) {
            return Some(Utc.from_utc_datetime(&ndt));
        }
    }
    None
}

/// Remove HTML tags from a string.
fn strip_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    // Collapse whitespace
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn sanitize_title(s: &str) -> String {
    let clean = strip_html(s);
    truncate(&clean, 80).to_string()
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        let mut idx = max;
        while !s.is_char_boundary(idx) { idx -= 1; }
        &s[..idx]
    }
}
