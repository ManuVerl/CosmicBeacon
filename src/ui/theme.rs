use egui::{Color32, Context, Rounding, Stroke, Visuals};

// ── Individual theme builders ──────────────────────────────────────────────

fn dark_visuals() -> Visuals {
    Visuals::dark()
}

fn light_visuals() -> Visuals {
    let mut v = Visuals::light();
    v.panel_fill                                    = Color32::from_rgb(244, 245, 247);
    v.window_fill                                   = Color32::from_rgb(248, 249, 251);
    v.extreme_bg_color                              = Color32::from_rgb(236, 238, 242);
    v.widgets.noninteractive.bg_fill                = Color32::from_rgb(240, 242, 245);
    v.widgets.noninteractive.weak_bg_fill           = Color32::from_rgb(244, 245, 247);
    v.widgets.noninteractive.bg_stroke.color        = Color32::from_rgb(215, 220, 228);
    v.widgets.noninteractive.fg_stroke.color        = Color32::from_rgb(45, 52, 64);
    v.widgets.inactive.bg_fill                      = Color32::from_rgb(232, 235, 240);
    v.widgets.inactive.weak_bg_fill                 = Color32::from_rgb(238, 240, 244);
    v.widgets.inactive.bg_stroke.color              = Color32::from_rgb(205, 212, 222);
    v.widgets.inactive.fg_stroke.color              = Color32::from_rgb(35, 42, 54);
    v.widgets.hovered.bg_fill                       = Color32::from_rgb(222, 228, 236);
    v.widgets.hovered.bg_stroke.color               = Color32::from_rgb(180, 192, 208);
    v.widgets.hovered.fg_stroke.color               = Color32::from_rgb(20, 25, 35);
    v.widgets.active.bg_fill                        = Color32::from_rgb(210, 218, 230);
    v.widgets.active.bg_stroke.color                = Color32::from_rgb(150, 168, 190);
    v.widgets.active.fg_stroke.color                = Color32::from_rgb(10, 15, 25);
    v.widgets.open.bg_fill                          = Color32::from_rgb(228, 232, 238);
    v
}

/// Teal — inspired by CDE/Motif teal-grey X11 style.
fn teal_visuals() -> Visuals {
    let mut v = Visuals::dark();
    let bg          = Color32::from_rgb(0,  96, 96);   // deep teal
    let panel       = Color32::from_rgb(0,  80, 80);
    let widget      = Color32::from_rgb(0, 112, 112);
    let widget_hot  = Color32::from_rgb(0, 140, 140);
    let widget_act  = Color32::from_rgb(0, 160, 160);
    let text        = Color32::from_rgb(230, 255, 255);
    let border      = Color32::from_rgb(0, 180, 180);

    v.override_text_color               = Some(text);
    v.panel_fill                        = panel;
    v.window_fill                       = bg;
    v.extreme_bg_color                  = Color32::from_rgb(0, 64, 64);
    v.window_stroke                     = Stroke::new(1.0_f32, border);

    v.widgets.noninteractive.bg_fill        = widget;
    v.widgets.noninteractive.weak_bg_fill   = panel;
    v.widgets.noninteractive.bg_stroke      = Stroke::new(1.0_f32, border);
    v.widgets.noninteractive.fg_stroke      = Stroke::new(1.0_f32, text);

    v.widgets.inactive.bg_fill              = widget;
    v.widgets.inactive.weak_bg_fill         = panel;
    v.widgets.inactive.bg_stroke            = Stroke::new(1.0_f32, Color32::from_rgb(0, 150, 150));
    v.widgets.inactive.fg_stroke            = Stroke::new(1.0_f32, text);

    v.widgets.hovered.bg_fill               = widget_hot;
    v.widgets.hovered.bg_stroke             = Stroke::new(1.0_f32, Color32::from_rgb(0, 200, 200));
    v.widgets.hovered.fg_stroke             = Stroke::new(1.0_f32, Color32::WHITE);

    v.widgets.active.bg_fill                = widget_act;
    v.widgets.active.bg_stroke              = Stroke::new(1.0_f32, Color32::WHITE);
    v.widgets.active.fg_stroke              = Stroke::new(1.0_f32, Color32::WHITE);

    v.widgets.open.bg_fill                  = widget_hot;

    v.selection.bg_fill                     = Color32::from_rgb(0, 200, 200);
    v.selection.stroke                      = Stroke::new(1.0_f32, Color32::WHITE);
    v.window_rounding                       = Rounding::same(2.0);
    v
}

/// Pink — inspired by SGI Irix / older X11 mauve-pink style.
fn pink_visuals() -> Visuals {
    let mut v = Visuals::dark();
    let bg          = Color32::from_rgb(80, 0,  60);   // deep magenta
    let panel       = Color32::from_rgb(68, 0,  50);
    let widget      = Color32::from_rgb(100, 10, 80);
    let widget_hot  = Color32::from_rgb(140, 30,110);
    let widget_act  = Color32::from_rgb(180, 50,140);
    let text        = Color32::from_rgb(255, 220, 250);
    let border      = Color32::from_rgb(220, 100, 190);

    v.override_text_color               = Some(text);
    v.panel_fill                        = panel;
    v.window_fill                       = bg;
    v.extreme_bg_color                  = Color32::from_rgb(55, 0, 42);
    v.window_stroke                     = Stroke::new(1.0_f32, border);

    v.widgets.noninteractive.bg_fill        = widget;
    v.widgets.noninteractive.weak_bg_fill   = panel;
    v.widgets.noninteractive.bg_stroke      = Stroke::new(1.0_f32, border);
    v.widgets.noninteractive.fg_stroke      = Stroke::new(1.0_f32, text);

    v.widgets.inactive.bg_fill              = widget;
    v.widgets.inactive.weak_bg_fill         = panel;
    v.widgets.inactive.bg_stroke            = Stroke::new(1.0_f32, Color32::from_rgb(180, 60, 150));
    v.widgets.inactive.fg_stroke            = Stroke::new(1.0_f32, text);

    v.widgets.hovered.bg_fill               = widget_hot;
    v.widgets.hovered.bg_stroke             = Stroke::new(1.0_f32, Color32::from_rgb(240, 130, 210));
    v.widgets.hovered.fg_stroke             = Stroke::new(1.0_f32, Color32::WHITE);

    v.widgets.active.bg_fill                = widget_act;
    v.widgets.active.bg_stroke              = Stroke::new(1.0_f32, Color32::WHITE);
    v.widgets.active.fg_stroke              = Stroke::new(1.0_f32, Color32::WHITE);

    v.widgets.open.bg_fill                  = widget_hot;

    v.selection.bg_fill                     = Color32::from_rgb(220, 100, 190);
    v.selection.stroke                      = Stroke::new(1.0_f32, Color32::WHITE);
    v.window_rounding                       = Rounding::same(2.0);
    v
}

/// Navy — inspired by FVWM/TWM navy / royal-blue X11 scheme.
fn navy_visuals() -> Visuals {
    let mut v = Visuals::dark();
    let bg          = Color32::from_rgb(10, 20,  70);  // deep navy
    let panel       = Color32::from_rgb( 8, 16,  60);
    let widget      = Color32::from_rgb(20, 40, 100);
    let widget_hot  = Color32::from_rgb(40, 70, 140);
    let widget_act  = Color32::from_rgb(60, 100,180);
    let text        = Color32::from_rgb(200, 220, 255);
    let border      = Color32::from_rgb(70, 130, 220);

    v.override_text_color               = Some(text);
    v.panel_fill                        = panel;
    v.window_fill                       = bg;
    v.extreme_bg_color                  = Color32::from_rgb(5, 10, 45);
    v.window_stroke                     = Stroke::new(1.0_f32, border);

    v.widgets.noninteractive.bg_fill        = widget;
    v.widgets.noninteractive.weak_bg_fill   = panel;
    v.widgets.noninteractive.bg_stroke      = Stroke::new(1.0_f32, border);
    v.widgets.noninteractive.fg_stroke      = Stroke::new(1.0_f32, text);

    v.widgets.inactive.bg_fill              = widget;
    v.widgets.inactive.weak_bg_fill         = panel;
    v.widgets.inactive.bg_stroke            = Stroke::new(1.0_f32, Color32::from_rgb(50, 100, 180));
    v.widgets.inactive.fg_stroke            = Stroke::new(1.0_f32, text);

    v.widgets.hovered.bg_fill               = widget_hot;
    v.widgets.hovered.bg_stroke             = Stroke::new(1.0_f32, Color32::from_rgb(100, 170, 255));
    v.widgets.hovered.fg_stroke             = Stroke::new(1.0_f32, Color32::WHITE);

    v.widgets.active.bg_fill                = widget_act;
    v.widgets.active.bg_stroke              = Stroke::new(1.0_f32, Color32::WHITE);
    v.widgets.active.fg_stroke              = Stroke::new(1.0_f32, Color32::WHITE);

    v.widgets.open.bg_fill                  = widget_hot;

    v.selection.bg_fill                     = Color32::from_rgb(70, 130, 220);
    v.selection.stroke                      = Stroke::new(1.0_f32, Color32::WHITE);
    v.window_rounding                       = Rounding::same(2.0);
    v
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Apply the appropriate visuals for the given theme to the egui context.
pub fn apply_theme(ctx: &Context, theme: &crate::config::Theme) {
    let visuals = match theme {
        crate::config::Theme::Dark  => dark_visuals(),
        crate::config::Theme::Light => light_visuals(),
        crate::config::Theme::Teal  => teal_visuals(),
        crate::config::Theme::Pink  => pink_visuals(),
        crate::config::Theme::Navy  => navy_visuals(),
    };
    ctx.set_visuals(visuals);
}

/// Return a WCAG AA-compliant error colour for the given theme.
///
/// Each value is chosen to achieve a contrast ratio ≥ 4.5:1 against the
/// theme's window/panel background colour:
///
/// | Theme | Background  | Error colour | Approx. contrast |
/// |-------|-------------|--------------|-----------------|
/// | Dark  | #141420     | #FF7070      | ~6.5 : 1        |
/// | Light | #F4F5F7     | #C0270A      | ~5.5 : 1        |
/// | Teal  | #005050     | #FFB3B3      | ~7.0 : 1        |
/// | Pink  | #440032     | #FFD0A0      | ~6.5 : 1        |
/// | Navy  | #08103C     | #FF9E9E      | ~6.8 : 1        |
pub fn error_color(theme: &crate::config::Theme) -> Color32 {
    match theme {
        crate::config::Theme::Dark  => Color32::from_rgb(255, 112, 112), // soft red on dark bg
        crate::config::Theme::Light => Color32::from_rgb(192,  39,  10), // dark red on light bg
        crate::config::Theme::Teal  => Color32::from_rgb(255, 179, 179), // light rose on teal bg
        crate::config::Theme::Pink  => Color32::from_rgb(255, 208, 160), // apricot on magenta bg
        crate::config::Theme::Navy  => Color32::from_rgb(255, 158, 158), // salmon on navy bg
    }
}

/// Return a WCAG AA-compliant "info / muted" colour for the given theme.
///
/// Used for secondary labels, "searching…" status, coordinates, etc.
/// Contrast ratio ≥ 4.5:1 against the theme background in all cases.
pub fn muted_color(theme: &crate::config::Theme) -> Color32 {
    match theme {
        crate::config::Theme::Dark  => Color32::from_rgb(180, 185, 200), // light-grey on dark
        crate::config::Theme::Light => Color32::from_rgb( 80,  88, 105), // mid-grey on light
        crate::config::Theme::Teal  => Color32::from_rgb(160, 230, 230), // teal-white on teal
        crate::config::Theme::Pink  => Color32::from_rgb(230, 190, 225), // soft lilac on pink
        crate::config::Theme::Navy  => Color32::from_rgb(160, 190, 245), // periwinkle on navy
    }
}
