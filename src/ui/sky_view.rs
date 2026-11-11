//! Detached "sky preview" window: a simple panoramic view (horizon at the bottom,
//! azimuth on the X axis, elevation on the Y axis) of where to look for an event.

use crate::app::AstroFeedApp;
use crate::i18n;
use crate::model::event::{Event, EventType, RadioAstronomicalType};
use crate::ui::theme;
use crate::utils::astro;
use chrono::{DateTime, Duration, Utc};
use egui::{pos2, Align2, Color32, Context, FontId, Pos2, Sense, Shape, Stroke, Ui, Vec2};
use uuid::Uuid;

const DEFAULT_FOV_DEG: f64 = 120.0;
const MIN_FOV_DEG: f64 = 30.0;
const MAX_FOV_DEG: f64 = 180.0;
/// Height (px) of the ground band below the horizon, where labels are drawn.
const GROUND_H: f32 = 44.0;
/// Height (px) reserved below the sky canvas for the information bar.
const INFO_H: f32 = 110.0;

const SKY: Color32 = Color32::from_rgb(12, 18, 40);
const GROUND: Color32 = Color32::from_rgb(30, 32, 36);
const HORIZON: Color32 = Color32::from_rgb(200, 205, 215);
const GRID: Color32 = Color32::from_rgb(50, 60, 90);
const TEXT: Color32 = Color32::from_rgb(210, 215, 225);
const TARGET: Color32 = Color32::from_rgb(255, 214, 80);
const TRACK: Color32 = Color32::from_rgb(120, 190, 255);

/// View state of the sky preview window (kept across frames).
pub struct SkyViewState {
    /// Azimuth (degrees) at the centre of the view.
    center_az: f64,
    /// Horizontal field of view (degrees).
    fov_deg: f64,
    /// Reference time chosen with the slider; None = event start.
    t_ref: Option<DateTime<Utc>>,
    /// Event the view was last initialised for (used to reset the view on a new selection).
    last_event: Option<Uuid>,
}

impl Default for SkyViewState {
    fn default() -> Self {
        Self {
            center_az: 180.0,
            fov_deg: DEFAULT_FOV_DEG,
            t_ref: None,
            last_event: None,
        }
    }
}

/// Show (or refresh) the sky preview in its own OS window. The same viewport id is
/// always used, so the window is reused when another event is selected.
pub fn show(app: &mut AstroFeedApp, ctx: &Context) {
    let lang = app.settings.language.clone();
    let title = i18n::t(&lang, "skyview.title");

    let builder = egui::ViewportBuilder::default()
        .with_title(title)
        .with_inner_size([900.0, 560.0])
        .with_min_inner_size([480.0, 360.0]);

    ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("sky_view"), builder, |ctx, class| {
        if ctx.input(|i| i.viewport().close_requested()) {
            close(app);
            return;
        }
        if matches!(class, egui::ViewportClass::Embedded) {
            // Backend without multi-viewport support: fall back to a floating egui window.
            let mut open = true;
            egui::Window::new(title)
                .open(&mut open)
                .default_size([700.0, 500.0])
                .show(ctx, |ui| content(app, ui));
            if !open {
                close(app);
            }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| content(app, ui));
        }
    });
}

fn close(app: &mut AstroFeedApp) {
    app.sky_view_open = false;
    app.selected_event = None;
}

// ── Content ────────────────────────────────────────────────────────────────

fn content(app: &mut AstroFeedApp, ui: &mut Ui) {
    let lang = app.settings.language.clone();
    let ui_theme = app.settings.theme.clone();
    let muted = theme::muted_color(&ui_theme);
    let observer = app.active_position().map(|p| (p.latitude, p.longitude));

    let event: Option<Event> = app
        .selected_event
        .and_then(|id| app.events.lock().unwrap().iter().find(|e| e.id == id).cloned());

    let Some(event) = event else {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new(i18n::t(&lang, "skyview.no_event")).color(muted));
        });
        return;
    };

    let mut st = std::mem::take(&mut app.sky_view);

    // New selection: reset the view on the target.
    if st.last_event != Some(event.id) {
        st.last_event = Some(event.id);
        st.fov_deg = DEFAULT_FOV_DEG;
        st.t_ref = None;
        st.center_az = target_at(&event, event.start_time).map_or(180.0, |(az, _)| az);
    }

    let t_ref = st.t_ref.unwrap_or(event.start_time);
    let target = target_at(&event, t_ref);
    let track_range = track_range(&event);

    // ── Controls ──────────────────────────────────────────────────────────
    ui.horizontal(|ui| {
        if ui.button(i18n::t(&lang, "skyview.recentre")).clicked() {
            st.fov_deg = DEFAULT_FOV_DEG;
            if let Some((az, _)) = target {
                st.center_az = az;
            }
        }
        if let Some((t0, t1)) = track_range {
            ui.separator();
            ui.label(i18n::t(&lang, "skyview.time"));
            let total = (t1 - t0).num_seconds().max(1) as f64;
            let mut secs = ((t_ref - t0).num_seconds() as f64).clamp(0.0, total);
            ui.spacing_mut().slider_width = (ui.available_width() - 330.0).max(100.0);
            if ui
                .add(egui::Slider::new(&mut secs, 0.0..=total).show_value(false))
                .changed()
            {
                st.t_ref = Some(t0 + Duration::seconds(secs as i64));
            }
            ui.label(egui::RichText::new(i18n::format_event_datetime(&lang, &t_ref)).small());
            let now = Utc::now();
            if now >= t0 && now <= t1 && ui.button(i18n::t(&lang, "skyview.now")).clicked() {
                st.t_ref = Some(now);
            }
        }
    });

    // ── Sky canvas ────────────────────────────────────────────────────────
    let canvas_h = (ui.available_height() - INFO_H).max(120.0);
    let (response, painter) =
        ui.allocate_painter(Vec2::new(ui.available_width(), canvas_h), Sense::click_and_drag());
    let rect = response.rect;

    if response.dragged() {
        let ppd = rect.width() as f64 / st.fov_deg;
        st.center_az = (st.center_az - response.drag_delta().x as f64 / ppd).rem_euclid(360.0);
    }
    if response.hovered() {
        let scroll = ui.input(|i| i.raw_scroll_delta.y);
        if scroll != 0.0 {
            st.fov_deg = (st.fov_deg * (-scroll as f64 * 0.002).exp()).clamp(MIN_FOV_DEG, MAX_FOV_DEG);
        }
    }

    let center = st.center_az;
    let ppd = rect.width() as f64 / st.fov_deg; // pixels per degree (both axes)
    let horizon_y = rect.bottom() - GROUND_H;
    let half_w = rect.width() / 2.0 + 30.0;
    let project = |az: f64, el: f64| -> Pos2 {
        let d = wrap_delta(az, center);
        pos2(
            rect.center().x + (d * ppd) as f32,
            horizon_y - (el * ppd) as f32,
        )
    };

    // Background, ground and horizon line
    painter.rect_filled(rect, 0.0, SKY);
    painter.rect_filled(
        egui::Rect::from_min_max(pos2(rect.left(), horizon_y), rect.right_bottom()),
        0.0,
        GROUND,
    );

    // Elevation graduations (every 10°) on the left edge
    let mut el = 10.0;
    while el <= 90.0 {
        let y = horizon_y - (el * ppd) as f32;
        if y > rect.top() {
            painter.line_segment(
                [pos2(rect.left(), y), pos2(rect.right(), y)],
                Stroke::new(1.0_f32, GRID),
            );
            painter.text(
                pos2(rect.left() + 4.0, y - 2.0),
                Align2::LEFT_BOTTOM,
                format!("{}°", el as i32),
                FontId::proportional(11.0),
                TEXT,
            );
        }
        el += 10.0;
    }

    painter.line_segment(
        [pos2(rect.left(), horizon_y), pos2(rect.right(), horizon_y)],
        Stroke::new(1.5_f32, HORIZON),
    );

    // Azimuth ticks every 15°, labelled every 45°
    for k in 0..24 {
        let az = k as f64 * 15.0;
        let x = project(az, 0.0).x;
        if (x - rect.center().x).abs() > half_w {
            continue;
        }
        if k % 3 == 0 {
            painter.line_segment([pos2(x, rect.top()), pos2(x, horizon_y)], Stroke::new(1.0_f32, GRID));
            painter.line_segment([pos2(x, horizon_y), pos2(x, horizon_y + 8.0)], Stroke::new(1.5_f32, HORIZON));
            painter.text(
                pos2(x, horizon_y + 9.0),
                Align2::CENTER_TOP,
                i18n::t(&lang, astro::azimuth_to_compass_key(az)),
                FontId::proportional(15.0),
                TEXT,
            );
            painter.text(
                pos2(x, horizon_y + 27.0),
                Align2::CENTER_TOP,
                format!("{}°", az as i32),
                FontId::proportional(10.0),
                TEXT.gamma_multiply(0.7),
            );
        } else {
            painter.line_segment([pos2(x, horizon_y), pos2(x, horizon_y + 5.0)], Stroke::new(1.0_f32, HORIZON));
        }
    }

    // Context objects: Sun and Moon
    if let Some((lat, lon)) = observer {
        let (saz, salt) = astro::sun_horizontal_position(t_ref, lat, lon);
        if salt > -10.0 {
            draw_body(&painter, project(saz, salt), 7.0, Color32::from_rgba_unmultiplied(255, 200, 80, 170), i18n::t(&lang, "skyview.sun"));
        }
        let (maz, malt) = astro::moon_horizontal_position(t_ref, lat, lon);
        if malt > -10.0 {
            draw_body(&painter, project(maz, malt), 6.0, Color32::from_rgba_unmultiplied(220, 225, 235, 170), i18n::t(&lang, "skyview.moon"));
        }
    }

    // Trajectory of a moving object
    if let Some(track) = event.sky_track.as_ref().filter(|t| t.len() >= 2) {
        let mut segments: Vec<Vec<Pos2>> = vec![Vec::new()];
        let mut prev_d: Option<f64> = None;
        for p in track {
            let d = wrap_delta(p.azimuth_deg, center);
            if prev_d.is_some_and(|pd| (d - pd).abs() > 180.0) {
                segments.push(Vec::new());
            }
            prev_d = Some(d);
            segments.last_mut().unwrap().push(project(p.azimuth_deg, p.elevation_deg));
        }
        for seg in segments.iter().filter(|s| s.len() >= 2) {
            painter.add(Shape::line(seg.clone(), Stroke::new(2.0_f32, TRACK)));
        }

        let first = &track[0];
        painter.circle_stroke(project(first.azimuth_deg, first.elevation_deg), 4.0, Stroke::new(1.5_f32, TRACK));
        if let Some(top) = track.iter().max_by(|a, b| a.elevation_deg.total_cmp(&b.elevation_deg)) {
            painter.circle_filled(project(top.azimuth_deg, top.elevation_deg), 3.0, TRACK);
        }
        // Direction arrow at the end of the path
        let last = project(track[track.len() - 1].azimuth_deg, track[track.len() - 1].elevation_deg);
        let before = project(track[track.len() - 2].azimuth_deg, track[track.len() - 2].elevation_deg);
        let dir = last - before;
        if dir.length() > 0.5 && dir.length() < rect.width() {
            let dir = dir.normalized();
            painter.arrow(last - dir * 18.0, dir * 18.0, Stroke::new(2.0_f32, TRACK));
        }
    }

    // Target marker
    if let Some((az, el)) = target {
        let p = project(az, el);
        let label = format!(
            "Az {:.1}°  El {:.1}°  {}",
            az,
            el,
            i18n::t(&lang, astro::azimuth_to_compass_key(az))
        );
        if (p.x - rect.center().x).abs() <= rect.width() / 2.0 {
            let stroke = Stroke::new(2.0_f32, TARGET);
            painter.circle_stroke(p, 12.0, stroke);
            painter.circle_filled(p, 3.0, TARGET);
            for dir in [Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y] {
                painter.line_segment([p + dir * 15.0, p + dir * 21.0], stroke);
            }
            painter.text(p + Vec2::new(24.0, -4.0), Align2::LEFT_BOTTOM, label, FontId::proportional(13.0), TARGET);
        } else if wrap_delta(az, center) < 0.0 {
            painter.text(pos2(rect.left() + 6.0, rect.top() + 16.0), Align2::LEFT_CENTER, format!("◀ {label}"), FontId::proportional(13.0), TARGET);
        } else {
            painter.text(pos2(rect.right() - 6.0, rect.top() + 16.0), Align2::RIGHT_CENTER, format!("{label} ▶"), FontId::proportional(13.0), TARGET);
        }
    }

    // ── Info bar ──────────────────────────────────────────────────────────
    ui.add_space(4.0);
    ui.label(egui::RichText::new(&event.title).strong());
    ui.label(
        egui::RichText::new(format!(
            "{}  →  {}",
            i18n::format_event_datetime(&lang, &event.start_time),
            i18n::format_event_datetime(&lang, &event.end_time)
        ))
        .small(),
    );
    match target {
        Some((az, el)) => {
            ui.label(format!(
                "Az {:.1}°  El {:.1}°  ({})",
                az,
                el,
                i18n::t(&lang, astro::azimuth_to_compass_key(az))
            ));
        }
        None => {
            ui.label(egui::RichText::new(i18n::t(&lang, "skyview.no_position")).color(muted));
        }
    }
    if let Some(eq) = &event.equipment {
        ui.label(egui::RichText::new(eq).small());
    }
    if let Some(desc) = &event.description {
        ui.label(egui::RichText::new(desc).small().italics());
    }
    if observer.is_none() {
        ui.label(egui::RichText::new(i18n::t(&lang, "skyview.no_observer")).small().color(muted));
    }

    app.sky_view = st;
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// Signed azimuth difference `az - center`, wrapped to (-180, 180].
fn wrap_delta(az: f64, center: f64) -> f64 {
    (az - center + 540.0).rem_euclid(360.0) - 180.0
}

fn draw_body(painter: &egui::Painter, p: Pos2, radius: f32, color: Color32, label: &str) {
    painter.circle_filled(p, radius, color);
    painter.text(
        p + Vec2::new(radius + 4.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.0),
        color,
    );
}

/// Time span covered by the event's sky track (None when there is no usable track).
fn track_range(event: &Event) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let track = event.sky_track.as_ref().filter(|t| t.len() >= 2)?;
    Some((track[0].time, track[track.len() - 1].time))
}

/// (azimuth, elevation) of the event's target at time `t`.
/// Moving objects are linearly interpolated along their track; fixed targets use
/// `sky_position`, then `listen_direction`. Returns None when no position is known.
fn target_at(event: &Event, t: DateTime<Utc>) -> Option<(f64, f64)> {
    if let Some(track) = event.sky_track.as_ref().filter(|tr| !tr.is_empty()) {
        let first = &track[0];
        let last = &track[track.len() - 1];
        if t <= first.time {
            return Some((first.azimuth_deg, first.elevation_deg));
        }
        if t >= last.time {
            return Some((last.azimuth_deg, last.elevation_deg));
        }
        let i = track.iter().position(|p| p.time > t).unwrap_or(track.len() - 1).max(1);
        let (a, b) = (&track[i - 1], &track[i]);
        let span = (b.time - a.time).num_milliseconds().max(1) as f64;
        let frac = (t - a.time).num_milliseconds() as f64 / span;
        let az = (a.azimuth_deg + wrap_delta(b.azimuth_deg, a.azimuth_deg) * frac).rem_euclid(360.0);
        let el = a.elevation_deg + (b.elevation_deg - a.elevation_deg) * frac;
        return Some((az, el));
    }
    if let Some(s) = &event.sky_position {
        return Some((s.azimuth_deg, s.elevation_deg));
    }
    // The ARISS listening direction is a placeholder (Az 0 / El 0), not a real target.
    if matches!(event.event_type, EventType::Radio(RadioAstronomicalType::IssRadio)) {
        return None;
    }
    event.listen_direction.as_ref().map(|d| (d.azimuth_deg, d.elevation_deg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::event::{AstronomicalType, Category, SkyPoint};
    use chrono::TimeZone;

    fn iss_event(track: Vec<SkyPoint>) -> Event {
        Event {
            id: Uuid::new_v4(),
            title: String::new(),
            category: Category::Astronomical,
            event_type: EventType::Astronomical(AstronomicalType::IssFlyover),
            start_time: track[0].time,
            end_time: track[track.len() - 1].time,
            sky_position: None,
            equipment: None,
            source: String::new(),
            description: None,
            freq_min_mhz: None,
            freq_max_mhz: None,
            listen_direction: None,
            sky_track: Some(track),
        }
    }

    #[test]
    fn wrap_delta_wraps_around_north() {
        assert!((wrap_delta(350.0, 10.0) + 20.0).abs() < 1e-9);
        assert!((wrap_delta(10.0, 350.0) - 20.0).abs() < 1e-9);
    }

    #[test]
    fn target_interpolates_along_track_across_north() {
        let t0 = Utc.with_ymd_and_hms(2026, 1, 1, 20, 0, 0).unwrap();
        let t1 = t0 + Duration::seconds(60);
        let ev = iss_event(vec![
            SkyPoint { time: t0, azimuth_deg: 350.0, elevation_deg: 10.0 },
            SkyPoint { time: t1, azimuth_deg: 10.0, elevation_deg: 30.0 },
        ]);
        let (az, el) = target_at(&ev, t0 + Duration::seconds(30)).unwrap();
        assert!(az.abs() < 1e-6 || (az - 360.0).abs() < 1e-6, "az={az}");
        assert!((el - 20.0).abs() < 1e-6);
        // Clamped outside the track
        assert_eq!(target_at(&ev, t0 - Duration::hours(1)).unwrap().0, 350.0);
    }

    #[test]
    fn sun_helper_matches_sun_times() {
        let now = Utc.with_ymd_and_hms(2026, 9, 17, 18, 0, 0).unwrap();
        let (lat, lon) = (50.5, 3.0);
        let reference = astro::calculate_sun_times(lat, lon, now.with_timezone(&chrono::Local));
        let (az, alt) = astro::sun_horizontal_position(now, lat, lon);
        assert!((alt - reference.current_elevation).abs() < 0.3, "alt {alt} vs {}", reference.current_elevation);
        assert!((az - reference.current_azimuth).abs() < 0.5, "az {az} vs {}", reference.current_azimuth);
    }
}
