use crate::app::{AstroFeedApp, CategoryFilter};
use crate::i18n;
use crate::model::event::{Category, Event};
use chrono::Utc;
use egui::{Ui, Vec2};

// ── Temporal navigation bar ────────────────────────────────────────────────

/// Draws the navigation bar above the event list.
/// Returns the active time cursor (either the pinned time or Utc::now()).
fn show_nav_bar(app: &mut AstroFeedApp, ui: &mut Ui) -> chrono::DateTime<Utc> {
    let lang = app.settings.language.clone();

    let cursor = app
        .event_time_cursor
        .unwrap_or_else(Utc::now);

    ui.horizontal(|ui| {
        // ── "Now" button ─────────────────────────────────────────────────────
        if ui.button(i18n::t(&lang, "nav.now")).clicked() {
            app.event_time_cursor = None;
            app.nav_date_input.clear();
            app.scroll_to_now = true;
        }

        ui.separator();

        // ── "Go to" date/time input ───────────────────────────────────────────
        ui.label(i18n::t(&lang, "nav.goto"));

        let hint = i18n::t(&lang, "nav.date_placeholder");
        let response = ui.add(
            egui::TextEdit::singleline(&mut app.nav_date_input)
                .hint_text(hint)
                .desired_width(160.0),
        );

        // Try to parse on every change
        if response.changed() || response.lost_focus() {
            let parsed = parse_date_input(&app.nav_date_input);
            if let Some(dt) = parsed {
                app.event_time_cursor = Some(dt);
                app.scroll_to_now = true;
            } else if app.nav_date_input.is_empty() {
                app.event_time_cursor = None;
                app.scroll_to_now = true;
            }
        }

        // Right-aligned: show current cursor time as small label
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let display = if app.event_time_cursor.is_some() {
                i18n::format_event_datetime(&lang, &cursor)
            } else {
                i18n::format_event_datetime(&lang, &Utc::now())
            };
            ui.label(egui::RichText::new(display).small().weak());
        });
    });

    cursor
}

/// Try to parse a user date/time string.  Accepts:
/// - "DD/MM/YYYY HH:MM"   (most locales)
/// - "MM/DD/YYYY HH:MM"   (en-US style — tried as fallback)
fn parse_date_input(s: &str) -> Option<chrono::DateTime<Utc>> {
    use chrono::NaiveDateTime;
    let s = s.trim();
    // Try ISO first: YYYY-MM-DD HH:MM
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M") {
        return Some(ndt.and_utc());
    }
    // DD/MM/YYYY HH:MM
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%d/%m/%Y %H:%M") {
        return Some(ndt.and_utc());
    }
    // DD.MM.YYYY HH:MM (German style)
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%d.%m.%Y %H:%M") {
        return Some(ndt.and_utc());
    }
    None
}

// ── Main show function ─────────────────────────────────────────────────────

pub fn show(app: &mut AstroFeedApp, ui: &mut Ui) {
    let lang = app.settings.language.clone();
    let events = app.events.lock().unwrap().clone();

    let filtered: Vec<&Event> = events
        .iter()
        .filter(|e| match &app.category_filter {
            CategoryFilter::All => true,
            CategoryFilter::Astronomical => e.category == Category::Astronomical,
            CategoryFilter::RadioAstronomical => e.category == Category::RadioAstronomical,
        })
        .collect();

    // ── Navigation bar ────────────────────────────────────────────────────
    let cursor_time = show_nav_bar(app, ui);
    ui.separator();

    if filtered.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label(i18n::t(&lang, "events.empty"));
        });
        return;
    }

    // ── Determine scroll target ────────────────────────────────────────────
    // Find the index of the first event that is >= cursor_time.
    let scroll_idx = filtered
        .iter()
        .position(|e| e.start_time >= cursor_time)
        .unwrap_or(filtered.len().saturating_sub(1));

    let want_scroll = app.scroll_to_now;
    if want_scroll {
        app.scroll_to_now = false;
    }

    // ── Scrollable event list ──────────────────────────────────────────────
    // We pre-compute approximate row height to derive the scroll offset.
    // egui's ScrollArea supports `show_rows` for virtual lists, but our rows
    // have variable height. We use `scroll_to_rect` via a tagged widget instead.
    let scroll_id = egui::Id::new("event_scroll");

    egui::ScrollArea::vertical()
        .id_source(scroll_id)
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            for (i, event) in filtered.iter().enumerate() {
                // Allocate a zero-height rect at the top of this row so we can
                // scroll to it precisely.
                if i == scroll_idx && want_scroll {
                    let top_rect = ui.next_widget_position();
                    // scroll_to_rect needs a Rect; we create a tiny one at this y
                    let r = egui::Rect::from_min_size(top_rect, Vec2::new(1.0, 1.0));
                    ui.scroll_to_rect(r, Some(egui::Align::TOP));
                }

                let selected = app.selected_event == Some(event.id);
                if event_row(ui, event, &lang, selected) {
                    app.selected_event = Some(event.id);
                    app.sky_view_open = true;
                }
                ui.separator();
            }
        });
}

// ── Event row ──────────────────────────────────────────────────────────────

/// Draws one event row. The whole row (icon, text and empty space) is a single
/// click target; returns true when it was clicked.
fn event_row(ui: &mut Ui, event: &Event, lang: &crate::i18n::Lang, selected: bool) -> bool {
    // Reserve a background shape drawn behind the row content (filled once hover state is known).
    let bg_idx = ui.painter().add(egui::Shape::Noop);

    let content = ui.scope(|ui| {
        // Selectable labels would swallow the clicks meant for the row.
        ui.style_mut().interaction.selectable_labels = false;
        event_row_content(ui, event, lang);
    });

    // Extend the row to the full available width so that empty space is clickable too.
    let rect = egui::Rect::from_min_max(
        content.response.rect.min,
        egui::pos2(ui.max_rect().right(), content.response.rect.max.y),
    );
    let response = ui
        .interact(rect, ui.id().with(("event_row", event.id)), egui::Sense::click())
        .on_hover_cursor(egui::CursorIcon::PointingHand);

    let fill = if selected {
        Some(ui.visuals().selection.bg_fill.linear_multiply(0.5))
    } else if response.hovered() {
        Some(ui.visuals().widgets.hovered.weak_bg_fill)
    } else {
        None
    };
    if let Some(fill) = fill {
        ui.painter().set(bg_idx, egui::Shape::rect_filled(rect, 2.0, fill));
    }

    response.clicked()
}

fn event_row_content(ui: &mut Ui, event: &Event, lang: &crate::i18n::Lang) {
    let icon = match event.category {
        Category::Astronomical => "🔭",
        Category::RadioAstronomical => "📡",
    };

    // Dim past events slightly
    let is_past = event.end_time < Utc::now();
    let alpha = if is_past { 0.55 } else { 1.0 };

    ui.horizontal(|ui| {
        // Icon: allocate a fixed-width column, centred vertically on the row.
        let icon_size = 28.0;
        let row_height = ui.spacing().interact_size.y * 2.0
            + ui.text_style_height(&egui::TextStyle::Small)
            + ui.spacing().item_spacing.y;
        let (icon_rect, _) = ui.allocate_exact_size(
            egui::vec2(icon_size + 8.0, row_height.max(icon_size + 4.0)),
            egui::Sense::hover(),
        );
        // Paint the icon centred inside that rect
        let galley = ui.painter().layout_no_wrap(
            icon.to_string(),
            egui::FontId::proportional(icon_size),
            ui.visuals().text_color(),
        );
        let text_pos = icon_rect.center() - galley.size() / 2.0;
        ui.painter().galley(text_pos, galley, ui.visuals().text_color());

        ui.vertical(|ui| {
            let title_text = egui::RichText::new(&event.title).strong();
            let title_text = if is_past {
                title_text.weak()
            } else {
                title_text
            };
            ui.label(title_text);

            ui.horizontal(|ui| {
                let dt_str = format!("🗓 {}", i18n::format_event_datetime(lang, &event.start_time));
                let dt_label = egui::RichText::new(dt_str).small();
                let dt_label = if is_past {
                    dt_label.color(egui::Color32::from_white_alpha((255.0 * alpha * 0.7) as u8))
                } else {
                    dt_label
                };
                ui.label(dt_label);

                if let Some(sky) = &event.sky_position {
                    ui.label(
                        egui::RichText::new(format!(" | Az {:.0}°  El {:.0}°", sky.azimuth_deg, sky.elevation_deg))
                            .small(),
                    );
                }
                if let (Some(fmin), Some(fmax)) = (event.freq_min_mhz, event.freq_max_mhz) {
                    ui.label(
                        egui::RichText::new(format!(" | {:.1}–{:.1} MHz", fmin, fmax)).small(),
                    );
                }
            });
            if let Some(desc) = &event.description {
                ui.label(egui::RichText::new(desc).small().italics());
            }
        });
    });
}
