use crate::app::{AstroFeedApp, CategoryFilter};
use crate::i18n;
use egui::{Context, TopBottomPanel, CentralPanel, SidePanel};

pub fn show(app: &mut AstroFeedApp, ctx: &Context, _frame: &mut eframe::Frame) {
    let lang = app.settings.language.clone();

    // ── Top menu bar ──────────────────────────────────────────────────────────
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button(i18n::t(&lang, "menu.file"), |ui| {
                if ui.button(i18n::t(&lang, "menu.refresh")).clicked() {
                    app.trigger_refresh();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button(i18n::t(&lang, "menu.quit")).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.menu_button(i18n::t(&lang, "menu.view"), |ui| {
                if ui.button(i18n::t(&lang, "menu.fullscreen")).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
                    ui.close_menu();
                }
                if ui.button(i18n::t(&lang, "menu.windowed")).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
                    ui.close_menu();
                }
            });
            ui.menu_button(i18n::t(&lang, "menu.edit"), |ui| {
                if ui.button(i18n::t(&lang, "menu.positions")).clicked() {
                    app.show_positions = true;
                    ui.close_menu();
                }
                if ui.button(i18n::t(&lang, "menu.settings")).clicked() {
                    app.show_settings = true;
                    ui.close_menu();
                }
            });
            ui.menu_button(i18n::t(&lang, "menu.help"), |ui| {
                if ui.button(i18n::t(&lang, "menu.about")).clicked() {
                    app.show_about = true;
                    ui.close_menu();
                }
            });

            // Spacer then position selector
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔄").on_hover_text(i18n::t(&lang, "menu.refresh_tooltip")).clicked() {
                    app.trigger_refresh();
                }
                ui.separator();

                let prev_index = app.active_position_index;
                let selected_name = app.active_position()
                    .map(|p| format!("{} {}", p.icon.emoji(), p.name))
                    .unwrap_or_else(|| i18n::t(&lang, "menu.no_position").to_string());

                // Snapshot position labels to avoid borrow conflict inside show_ui
                let pos_labels: Vec<(usize, String)> = app.positions
                    .iter()
                    .enumerate()
                    .map(|(i, p)| (i, format!("{} {}", p.icon.emoji(), p.name)))
                    .collect();

                egui::ComboBox::from_id_source("position_selector")
                    .selected_text(selected_name)
                    .show_ui(ui, |ui| {
                        for (i, label) in &pos_labels {
                            ui.selectable_value(&mut app.active_position_index, *i, label);
                        }
                    });

                // When the user picks a different position: save + refresh
                if app.active_position_index != prev_index {
                    app.settings.active_position_index = app.active_position_index;
                    app.settings.save();
                    app.trigger_refresh();
                }

                ui.label("📍");
            });
        });
    });

    // ── Status bar ────────────────────────────────────────────────────────────
    TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let report = app.sync_report.lock().unwrap();
            let errors = report.iter().filter(|r| {
                r.status == crate::sources::manager::SyncStatus::Error
            }).count();
            let pending = report.iter().filter(|r| {
                r.status == crate::sources::manager::SyncStatus::Pending
            }).count();

            if pending > 0 {
                ui.spinner();
                ui.label(format!(
                    "{} ({} {})…",
                    i18n::t(&lang, "status.syncing"),
                    pending,
                    i18n::t(&lang, "status.source_s")
                ));
            } else if errors > 0 {
                ui.colored_label(
                    egui::Color32::from_rgb(220, 80, 80),
                    format!("⚠ {} {}", errors, i18n::t(&lang, "status.error")),
                );
            } else if !report.is_empty() {
                ui.label(i18n::t(&lang, "status.ok"));
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button(i18n::t(&lang, "status.sync_report")).clicked() {
                    app.show_sync_report = !app.show_sync_report;
                }
            });
        });
    });

    // Request a repaint every second to ensure the real-time clock and solar elevation update smoothly
    ctx.request_repaint_after(std::time::Duration::from_secs(1));

    // ── Filter sidebar ────────────────────────────────────────────────────────
    SidePanel::left("filter_panel")
        .resizable(false)
        .min_width(190.0)
        .max_width(220.0)
        .show(ctx, |ui| {
            // Top section: Category filters
            ui.add_space(2.0);
            ui.label(egui::RichText::new(i18n::t(&lang, "filter.categories")).strong().size(11.0));
            ui.separator();
            ui.selectable_value(&mut app.category_filter, CategoryFilter::All, i18n::t(&lang, "filter.all"));
            ui.selectable_value(&mut app.category_filter, CategoryFilter::Astronomical, i18n::t(&lang, "filter.astro"));
            ui.selectable_value(&mut app.category_filter, CategoryFilter::RadioAstronomical, i18n::t(&lang, "filter.radio"));

            // Bottom section: Ephemerides & Clock panel pinned to the bottom
            let avail_rect = ui.available_rect_before_wrap();
            let panel_height = 360.0;
            let bottom_rect = egui::Rect::from_min_max(
                egui::pos2(avail_rect.min.x, (avail_rect.max.y - panel_height).max(avail_rect.min.y)),
                avail_rect.max,
            );

            let mut child_ui = ui.child_ui(bottom_rect, egui::Layout::top_down_justified(egui::Align::Min));
            crate::ui::sidebar_info::show_sidebar_info(app, &mut child_ui);
        });

    // ── Main event list ───────────────────────────────────────────────────────
    CentralPanel::default().show(ctx, |ui| {
        crate::ui::event_list::show(app, ui);
    });

    // ── Modal panels ─────────────────────────────────────────────────────────
    if app.show_sync_report {
        crate::ui::sync_report::show(app, ctx);
    }
    if app.show_settings {
        crate::ui::settings::show(app, ctx);
    }
    if app.show_positions {
        crate::ui::positions::show(app, ctx);
    }
    if app.show_about {
        crate::ui::about::show(app, ctx);
    }
    if app.sky_view_open {
        crate::ui::sky_view::show(app, ctx);
    }
}
