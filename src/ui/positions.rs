use crate::app::AstroFeedApp;
use crate::i18n;
use crate::model::position::{CoordAcquisition, Position, PositionIcon};
use crate::ui::theme::{error_color, muted_color};
use crate::utils::geocoding::{geocode_by_ip, geocode_name, GeoError};
use egui::{Context, Window};
use std::sync::{Arc, Mutex};
use std::time::Instant;

// ── Geo-lookup async state ────────────────────────────────────────────────────

/// Result that a background geo thread writes back into form state.
#[derive(Clone)]
enum GeoResult {
    Ok { lat: f64, lon: f64, display_name: String },
    Err(String),
}

// ── Temporary form state stored in egui memory ────────────────────────────────

#[derive(Clone, Default)]
struct FormState {
    /// None = "add new"; Some(idx) = "editing existing position at index"
    editing_index: Option<usize>,
    icon: PositionIcon,
    name: String,
    lat_s: String,
    lon_s: String,
    /// The active_position_index that was current before the user clicked a row
    /// to edit it. Restored when Cancel is pressed.
    prev_active_index: usize,
    /// Shared slot written by the background geo-lookup thread.
    geo_pending: Option<Arc<Mutex<Option<GeoResult>>>>,
    /// Message shown below the Name/Lat/Lon rows (searching… / error text).
    geo_status: Option<String>,
    /// True when the current status is an error (vs. an info/searching message).
    geo_is_error: bool,
    /// Instant at which the search was started — used to enforce a 2 s minimum
    /// display time for the "searching…" message so the user can actually read it.
    geo_search_started_at: Option<Instant>,
}

// ── Public entry point ────────────────────────────────────────────────────────

pub fn show(app: &mut AstroFeedApp, ctx: &Context) {
    let mut open = app.show_positions;
    let lang = app.settings.language.clone();
    let form_id = egui::Id::new("positions_form_state");

    // Load form state from egui memory
    let mut form: FormState = ctx.data_mut(|d| d.get_temp(form_id).unwrap_or_default());

    // ── Poll background geo thread ────────────────────────────────────────────
    // Extract any finished result first (releasing the borrow), then apply mutations.
    let finished_geo: Option<GeoResult> = form
        .geo_pending
        .as_ref()
        .and_then(|slot| slot.try_lock().ok().and_then(|g| g.clone()));

    if let Some(result) = finished_geo {
        // Enforce minimum 2 s display of the "searching…" message.
        let elapsed = form
            .geo_search_started_at
            .map(|t| t.elapsed().as_millis())
            .unwrap_or(u128::MAX);

        if elapsed >= 2_000 {
            match result {
                GeoResult::Ok { lat, lon, display_name } => {
                    form.lat_s = format!("{:.6}", lat);
                    form.lon_s = format!("{:.6}", lon);
                    // Fill name only if it is still empty
                    if form.name.trim().is_empty() {
                        form.name = display_name;
                    }
                    form.geo_status  = None;
                    form.geo_is_error = false;
                }
                GeoResult::Err(msg) => {
                    form.geo_status  = Some(msg);
                    form.geo_is_error = true;
                }
            }
            form.geo_pending           = None;
            form.geo_search_started_at = None;
        } else {
            // Result arrived too quickly — keep the slot alive so we re-poll next frame.
            // Schedule a repaint after the remaining delay.
            let remaining_ms = (2_000 - elapsed) as u64;
            ctx.request_repaint_after(std::time::Duration::from_millis(remaining_ms));
        }
    }

    Window::new(i18n::t(&lang, "positions.title"))
        .id(egui::Id::new("positions_window"))
        .open(&mut open)
        .resizable(true)
        .collapsible(false)
        .min_width(480.0)
        .show(ctx, |ui| {

            // ── Position list ─────────────────────────────────────────────────
            // Snapshot so we can mutate app freely after the loop.
            let rows: Vec<(usize, String, String, f64, f64, PositionIcon)> = app.positions
                .iter()
                .enumerate()
                .map(|(i, p)| (i, p.icon.emoji().to_string(), p.name.clone(),
                               p.latitude, p.longitude, p.icon.clone()))
                .collect();

            let mut to_delete: Option<usize> = None;
            let mut clicked_idx: Option<(usize, PositionIcon, String, f64, f64)> = None;

            for (i, emoji, name, lat, lon, icon) in &rows {
                let i = *i;
                ui.horizontal(|ui| {
                    // Show emoji + user-given name only (no "Home / Observatory / …")
                    let label = format!("{} {}", emoji, name);
                    let selected = i == app.active_position_index;

                    if ui.selectable_label(selected, &label).clicked() {
                        clicked_idx = Some((i, icon.clone(), name.clone(), *lat, *lon));
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("🗑")
                            .on_hover_text(i18n::t(&lang, "positions.delete_tooltip"))
                            .clicked()
                        {
                            to_delete = Some(i);
                        }
                        ui.label(
                            egui::RichText::new(format!("({:.4}°, {:.4}°)", lat, lon))
                                .weak().small(),
                        );
                    });
                });
            }

            // Deferred: handle click (set active + load form)
            if let Some((i, icon, name, lat, lon)) = clicked_idx {
                form.prev_active_index     = app.active_position_index;
                app.active_position_index  = i;
                form.editing_index         = Some(i);
                form.icon                  = icon;
                form.name                  = name;
                form.lat_s                 = format!("{}", lat);
                form.lon_s                 = format!("{}", lon);
                form.geo_pending           = None;
                form.geo_status            = None;
                form.geo_is_error          = false;
                form.geo_search_started_at = None;
                save_positions(app);
            }

            // Deferred: handle delete
            if let Some(idx) = to_delete {
                if form.editing_index == Some(idx) {
                    form = FormState::default();
                } else if let Some(ei) = form.editing_index {
                    if idx < ei { form.editing_index = Some(ei - 1); }
                }
                app.positions.remove(idx);
                if app.positions.is_empty() {
                    app.active_position_index = 0;
                } else if app.active_position_index >= app.positions.len() {
                    app.active_position_index = app.positions.len() - 1;
                }
                save_positions(app);
            }

            ui.separator();

            // ── Add / Edit form ───────────────────────────────────────────────
            let is_editing = form.editing_index.is_some();
            ui.label(if is_editing {
                i18n::t(&lang, "positions.edit_label")
            } else {
                i18n::t(&lang, "positions.add_label")
            });

            // Row 1: Icon picker
            ui.horizontal(|ui| {
                ui.label(i18n::t(&lang, "positions.icon_label"));
                egui::ComboBox::from_id_source("pos_icon_picker")
                    .selected_text(form.icon.picker_label())
                    .show_ui(ui, |ui| {
                        for icon in PositionIcon::all() {
                            ui.selectable_value(
                                &mut form.icon,
                                icon.clone(),
                                icon.picker_label(),
                            );
                        }
                    });
            });

            // Row 2: Name + 🎯 button
            let searching = form.geo_pending.is_some();
            ui.horizontal(|ui| {
                ui.label(i18n::t(&lang, "positions.name"));
                ui.add(egui::TextEdit::singleline(&mut form.name).desired_width(200.0));

                // 🎯 auto-detect button — placed next to the name field
                let geolocate_btn = ui
                    .add_enabled(!searching, egui::Button::new("🎯"))
                    .on_hover_text(i18n::t(&lang, "positions.geolocate_tooltip"));

                if geolocate_btn.clicked() {
                    let city_name = form.name.trim().to_string();
                    let slot: Arc<Mutex<Option<GeoResult>>> = Arc::new(Mutex::new(None));
                    form.geo_pending           = Some(Arc::clone(&slot));
                    form.geo_status            = Some(i18n::t(&lang, "positions.geo_searching").to_string());
                    form.geo_is_error          = false;
                    form.geo_search_started_at = Some(Instant::now());
                    form.lat_s.clear();
                    form.lon_s.clear();

                    let ctx_clone       = ctx.clone();
                    let err_no_result   = i18n::t(&lang, "positions.geo_err_no_result").to_string();
                    let err_network     = i18n::t(&lang, "positions.geo_err_network").to_string();
                    let err_no_location = i18n::t(&lang, "positions.geo_err_no_location").to_string();

                    std::thread::spawn(move || {
                        let result = if !city_name.is_empty() {
                            match geocode_name(&city_name) {
                                Ok(ll) => GeoResult::Ok {
                                    lat: ll.lat,
                                    lon: ll.lon,
                                    display_name: ll.display_name,
                                },
                                Err(GeoError::NoResult)        => GeoResult::Err(err_no_result),
                                Err(GeoError::NetworkError(_)) => GeoResult::Err(err_network),
                                Err(GeoError::NoLocationFound) => GeoResult::Err(err_no_location),
                            }
                        } else {
                            match geocode_by_ip() {
                                Ok(ll) => GeoResult::Ok {
                                    lat: ll.lat,
                                    lon: ll.lon,
                                    display_name: ll.display_name,
                                },
                                Err(GeoError::NetworkError(_)) => GeoResult::Err(err_network),
                                Err(_)                         => GeoResult::Err(err_no_location),
                            }
                        };

                        if let Ok(mut guard) = slot.lock() {
                            *guard = Some(result);
                        }
                        ctx_clone.request_repaint();
                    });
                }
            });

            // Row 3: Lat / Lon (plain, no button here any more)
            ui.horizontal(|ui| {
                ui.label(i18n::t(&lang, "positions.lat"));
                ui.add(egui::TextEdit::singleline(&mut form.lat_s).desired_width(90.0));
                ui.label(i18n::t(&lang, "positions.lon"));
                ui.add(egui::TextEdit::singleline(&mut form.lon_s).desired_width(90.0));
            });

            // Status message (searching… or error) — theme-aware colours, normal font size
            if let Some(msg) = &form.geo_status {
                let color = if form.geo_is_error {
                    error_color(&app.settings.theme)
                } else {
                    muted_color(&app.settings.theme)
                };
                ui.label(egui::RichText::new(msg).color(color));
            }

            // Row 4: Buttons
            ui.horizontal(|ui| {
                let action_label = if is_editing {
                    i18n::t(&lang, "positions.save_btn")
                } else {
                    i18n::t(&lang, "positions.add_btn")
                };

                if ui.button(action_label).clicked() {
                    if let (Ok(lat), Ok(lon)) = (
                        form.lat_s.trim().parse::<f64>(),
                        form.lon_s.trim().parse::<f64>(),
                    ) {
                        let name = if form.name.trim().is_empty() {
                            i18n::t(&lang, "positions.default_name").to_string()
                        } else {
                            form.name.trim().to_string()
                        };
                        let new_pos = Position {
                            name,
                            icon: form.icon.clone(),
                            latitude: lat,
                            longitude: lon,
                            acquisition: CoordAcquisition::Manual,
                        };

                        if let Some(idx) = form.editing_index {
                            if let Some(existing) = app.positions.get_mut(idx) {
                                *existing = new_pos;
                            }
                        } else {
                            app.positions.push(new_pos);
                            app.active_position_index = app.positions.len() - 1;
                        }

                        save_positions(app);
                        form = FormState::default();
                    }
                }

                if is_editing && ui.button(i18n::t(&lang, "positions.cancel_btn")).clicked() {
                    // Restore the selection that existed before the user clicked the row
                    app.active_position_index = form.prev_active_index;
                    app.settings.active_position_index = form.prev_active_index;
                    app.settings.save();
                    form = FormState::default();
                }
            });
        });

    // Persist form state back into egui memory
    ctx.data_mut(|d| d.insert_temp(form_id, form));

    app.show_positions = open;
}

// ── Save helper ───────────────────────────────────────────────────────────────

/// Sync positions + active index into settings and flush to disk immediately.
fn save_positions(app: &mut AstroFeedApp) {
    app.settings.positions = app.positions.clone();
    app.settings.active_position_index = app.active_position_index;
    app.settings.save();
}
