use crate::config::Settings;
use crate::model::{event::Event, position::Position};
use crate::sources::manager::{SourceManager, SyncReport};
use crate::ui::main_window;
use std::sync::{Arc, Mutex};

/// Global application state shared between UI and background tasks.
pub struct AstroFeedApp {
    pub settings: Settings,
    pub positions: Vec<Position>,
    pub active_position_index: usize,
    pub events: Arc<Mutex<Vec<Event>>>,
    pub sync_report: Arc<Mutex<Vec<SyncReport>>>,
    pub show_sync_report: bool,
    pub show_settings: bool,
    pub show_positions: bool,
    pub show_about: bool,
    pub category_filter: CategoryFilter,
    /// The point in time the event list is focused on. None = current time (live).
    pub event_time_cursor: Option<chrono::DateTime<chrono::Utc>>,
    /// Raw text in the date/time navigation field.
    pub nav_date_input: String,
    /// Set to true to request the scroll area to jump to the cursor position on the next frame.
    pub scroll_to_now: bool,
    source_manager: SourceManager,
    /// Snapshot of `settings.disabled_sources` taken when the settings window opens.
    /// Used to restore the value if the window is closed without saving.
    pub settings_sources_snapshot: Option<Vec<String>>,
    /// Event currently previewed in the sky view window.
    pub selected_event: Option<uuid::Uuid>,
    /// Whether the detached sky view window is open.
    pub sky_view_open: bool,
    /// Pan/zoom/time state of the sky view window.
    pub sky_view: crate::ui::sky_view::SkyViewState,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CategoryFilter {
    #[default]
    All,
    Astronomical,
    RadioAstronomical,
}

impl AstroFeedApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure font fallbacks so system emojis (Segoe UI Emoji / Apple Color Emoji) render properly
        Self::configure_fonts(&cc.egui_ctx);

        let settings = Settings::load();

        // Apply theme (dark or softened light mode)
        crate::ui::theme::apply_theme(&cc.egui_ctx, &settings.theme);

        let positions = settings.positions.clone();
        // Restore last active index, clamped to valid range
        let active_position_index = settings.active_position_index
            .min(positions.len().saturating_sub(1));

        let events = Arc::new(Mutex::new(Vec::new()));
        let sync_report = Arc::new(Mutex::new(Vec::new()));

        let source_manager = SourceManager::new(
            Arc::clone(&events),
            Arc::clone(&sync_report),
            cc.egui_ctx.clone(),
        );

        let mut app = Self {
            settings,
            positions,
            active_position_index,
            events,
            sync_report,
            show_sync_report: false,
            show_settings: false,
            show_positions: false,
            show_about: false,
            category_filter: CategoryFilter::All,
            event_time_cursor: None,
            nav_date_input: String::new(),
            scroll_to_now: true,
            source_manager,
            settings_sources_snapshot: None,
            selected_event: None,
            sky_view_open: false,
            sky_view: Default::default(),
        };

        // Trigger initial refresh if configured
        app.maybe_refresh_on_startup();
        app
    }

    pub fn active_position(&self) -> Option<&Position> {
        self.positions.get(self.active_position_index)
    }

    pub fn trigger_refresh(&mut self) {
        if let Some(pos) = self.active_position().cloned() {
            let disabled: std::collections::HashSet<String> =
                self.settings.disabled_sources.iter().cloned().collect();
            self.source_manager.refresh(pos, disabled);
            // After a refresh the list will be rebuilt; scroll back to "now"
            self.scroll_to_now = true;
        }
    }

    fn maybe_refresh_on_startup(&mut self) {
        use crate::config::UpdateFrequency;
        match self.settings.update_frequency {
            UpdateFrequency::OnStartup => self.trigger_refresh(),
            UpdateFrequency::Weekly => {
                if self.settings.should_refresh_weekly() {
                    self.trigger_refresh();
                }
            }
            UpdateFrequency::Monthly => {
                if self.settings.should_refresh_monthly() {
                    self.trigger_refresh();
                }
            }
        }
    }

    /// Try loading system emoji fonts (e.g. Segoe UI Emoji on Windows) as fallback fonts in egui.
    fn configure_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // Try standard Windows Segoe UI Emoji font path
        #[cfg(target_os = "windows")]
        {
            let emoji_font_path = r"C:\Windows\Fonts\seguiemj.ttf";
            if let Ok(font_data) = std::fs::read(emoji_font_path) {
                fonts.font_data.insert(
                    "seguiemj".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                // Add to Proportional family fallbacks
                if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                    family.push("seguiemj".to_owned());
                }
                // Add to Monospace family fallbacks
                if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                    family.push("seguiemj".to_owned());
                }
            }
        }

        ctx.set_fonts(fonts);
    }
}

impl eframe::App for AstroFeedApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        main_window::show(self, ctx, frame);
    }
}
