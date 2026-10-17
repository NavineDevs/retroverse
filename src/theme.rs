//! The RetroVerse look: near-black surfaces with royal blue and royal purple accents.
//!
//! egui ships a dark theme, but its default greys are flat and its accent is a washed-out
//! lavender. Everything here is defined as one palette so the window reads as a single
//! designed surface rather than a set of defaults with a few coloured widgets.

use eframe::egui::{self, Color32, CornerRadius, RichText, Stroke, Visuals};

/// True black is harsh on an OLED and crushes the panel edges, so the base is a very dark
/// blue-black and the panels sit a few steps above it.
pub const BACKDROP: Color32 = Color32::from_rgb(6, 6, 10);
pub const SURFACE: Color32 = Color32::from_rgb(14, 14, 22);
pub const SURFACE_HIGH: Color32 = Color32::from_rgb(22, 22, 34);

/// Royal blue: the primary action colour.
pub const ROYAL_BLUE: Color32 = Color32::from_rgb(28, 60, 190);
/// Royal purple: the secondary accent, used for highlights and links.
pub const ROYAL_PURPLE: Color32 = Color32::from_rgb(104, 40, 190);
/// A brighter purple for text that needs to stand out without shouting.
pub const ROYAL_PURPLE_BRIGHT: Color32 = Color32::from_rgb(158, 96, 255);
/// Blue at full strength for focus rings and selection.
pub const ROYAL_BLUE_BRIGHT: Color32 = Color32::from_rgb(72, 130, 255);

pub const TEXT: Color32 = Color32::from_rgb(232, 234, 244);
pub const TEXT_DIM: Color32 = Color32::from_rgb(146, 150, 176);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(96, 100, 124);
pub const OUTLINE: Color32 = Color32::from_rgb(44, 46, 70);

pub const DANGER: Color32 = Color32::from_rgb(226, 74, 92);
pub const OK: Color32 = Color32::from_rgb(56, 200, 140);

/// The accent used for buttons the user is meant to press.
pub const ACCENT: Color32 = ROYAL_BLUE;

/// Applies the palette to an egui context.
pub fn apply(ctx: &egui::Context) {
    // `global_style`/`global_style_mut` rather than a `style()` accessor: egui 0.36 exposes
    // the active style this way, and it is what new popups and windows pick up.
    let mut style = (*ctx.global_style()).clone();

    let mut visuals = Visuals::dark();
    visuals.panel_fill = BACKDROP;
    visuals.window_fill = SURFACE;
    visuals.extreme_bg_color = BACKDROP;
    visuals.faint_bg_color = SURFACE_HIGH;
    visuals.code_bg_color = SURFACE_HIGH;

    visuals.override_text_color = Some(TEXT);
    visuals.weak_text_color = Some(TEXT_DIM);
    visuals.hyperlink_color = ROYAL_BLUE_BRIGHT;
    visuals.text_edit_bg_color = Some(SURFACE_HIGH);
    visuals.selection.bg_fill = ROYAL_BLUE;
    visuals.selection.stroke = Stroke::new(1.0, ROYAL_BLUE_BRIGHT);

    visuals.widgets.noninteractive.bg_fill = SURFACE;
    visuals.widgets.noninteractive.weak_bg_fill = SURFACE;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, OUTLINE);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_DIM);

    visuals.widgets.inactive.bg_fill = SURFACE_HIGH;
    visuals.widgets.inactive.weak_bg_fill = SURFACE;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, OUTLINE);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(32, 34, 54);
    visuals.widgets.hovered.weak_bg_fill = SURFACE_HIGH;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ROYAL_BLUE_BRIGHT);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT);

    visuals.widgets.active.bg_fill = ROYAL_BLUE;
    visuals.widgets.active.weak_bg_fill = ROYAL_PURPLE;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ROYAL_BLUE_BRIGHT);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);

    visuals.widgets.open.bg_fill = SURFACE_HIGH;
    visuals.widgets.open.weak_bg_fill = SURFACE;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, OUTLINE);

    visuals.error_fg_color = DANGER;
    visuals.warn_fg_color = Color32::from_rgb(232, 176, 60);

    visuals.window_stroke = Stroke::new(1.0, OUTLINE);
    visuals.window_corner_radius = CornerRadius::same(8);
    visuals.menu_corner_radius = CornerRadius::same(8);
    visuals.popup_shadow = egui::epaint::Shadow {
        offset: [0, 6],
        blur: 18,
        spread: 0,
        color: Color32::from_black_alpha(140),
    };
    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(14.0, 7.0);
    style.spacing.window_margin = egui::Margin::same(14);
    style.spacing.menu_margin = egui::Margin::same(10);
    style.spacing.slider_width = 180.0;
    style.spacing.interact_size.y = 26.0;
    // egui 0.36 dropped per-widget corner radii; `Frame::corner_radius` on individual
    // widgets is the supported way, and `primary`/`secondary`/`danger` below use it.

    ctx.global_style_mut(move |current| *current = style);
}

/// The wordmark, in the brighter purple so the title carries the secondary accent.
pub fn wordmark(text: &str) -> RichText {
    RichText::new(text).size(30.0).strong().color(ROYAL_PURPLE_BRIGHT)
}

/// The glyph colour: not pure white, so it sits on the badge rather than glaring off it.
const GLYPH: Color32 = Color32::from_rgb(242, 240, 255);
/// The small body on the orbit.
const MOON: Color32 = Color32::from_rgb(120, 226, 255);

/// Paints the RetroVerse mark: a ring with one small body on it, on a rounded badge.
///
/// Drawn from geometry rather than loaded as an image, so it is crisp at every scale and the
/// program needs no image decoder to show its own logo. [`icon_rgba`] draws the same shapes for
/// the window icon, from the same proportions, so the two cannot drift apart.
pub fn paint_mark(painter: &egui::Painter, rect: egui::Rect) {
    let size = rect.width().min(rect.height());
    let centre = rect.center();
    painter.rect_filled(rect, CornerRadius::same((size * 0.23).min(255.0) as u8), ROYAL_BLUE);
    let radius = size * 0.30;
    painter.circle_stroke(centre, radius, Stroke::new(size * 0.085, GLYPH));
    let angle = std::f32::consts::FRAC_PI_4;
    let moon = centre + egui::vec2(radius * angle.cos(), -radius * angle.sin());
    painter.circle_filled(moon, size * 0.085, MOON);
}

/// The mark as a widget, for the header.
pub fn mark(ui: &mut egui::Ui, size: f32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    paint_mark(ui.painter(), rect);
    response
}

/// The mark as RGBA pixels, for the window icon.
///
/// Computed from distance fields rather than shipped as a file: the edge of each shape is
/// softened over one pixel, which is all the anti-aliasing a small icon needs.
pub fn icon_rgba(size: u32) -> Vec<u8> {
    let s = size as f32;
    let centre = s / 2.0;
    let radius = s * 0.30;
    let angle = std::f32::consts::FRAC_PI_4;
    let moon = (centre + radius * angle.cos(), centre - radius * angle.sin());
    let mut out = vec![0u8; (size * size * 4) as usize];

    // How much of a pixel a shape covers, from its signed distance: inside is 1, outside is 0.
    let cover = |distance: f32| (0.5 - distance).clamp(0.0, 1.0);
    let mix = |from: [f32; 3], to: [f32; 3], t: f32| {
        [from[0] + (to[0] - from[0]) * t, from[1] + (to[1] - from[1]) * t, from[2] + (to[2] - from[2]) * t]
    };
    let rgb = |c: Color32| [c.r() as f32, c.g() as f32, c.b() as f32];

    for y in 0..size {
        for x in 0..size {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);

            // Rounded square.
            let corner = s * 0.23;
            let (dx, dy) = ((px - centre).abs() - (centre - corner), (py - centre).abs() - (centre - corner));
            let badge_distance = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt() - corner;
            let badge = cover(badge_distance);
            if badge <= 0.0 {
                continue;
            }

            // Blue at the top left running to purple at the bottom right.
            let along = ((px + py) / (2.0 * s)).clamp(0.0, 1.0);
            let mut colour = mix(rgb(ROYAL_BLUE), rgb(ROYAL_PURPLE), along);

            let from_centre = ((px - centre).powi(2) + (py - centre).powi(2)).sqrt();
            let ring = cover((from_centre - radius).abs() - s * 0.085 / 2.0);
            colour = mix(colour, rgb(GLYPH), ring);

            let from_moon = ((px - moon.0).powi(2) + (py - moon.1).powi(2)).sqrt();
            let body = cover(from_moon - s * 0.085);
            colour = mix(colour, rgb(MOON), body);

            let at = ((y * size + x) * 4) as usize;
            out[at] = colour[0].round() as u8;
            out[at + 1] = colour[1].round() as u8;
            out[at + 2] = colour[2].round() as u8;
            out[at + 3] = (badge * 255.0).round() as u8;
        }
    }
    out
}

/// A small pill for a count or a state, e.g. the number of builds on a tab.
pub fn pill(ui: &mut egui::Ui, text: impl Into<String>, accent: Color32) {
    egui::Frame::new()
        .fill(accent.gamma_multiply(0.22))
        .stroke(Stroke::new(1.0, accent.gamma_multiply(0.55)))
        .corner_radius(CornerRadius::same(9))
        .inner_margin(egui::Margin::symmetric(7, 2))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(11.0).strong().color(accent));
        });
}

/// A primary action button: royal blue, the colour the eye is drawn to first.
///
/// `'static` because the label is copied into the button, so the result does not borrow
/// from `text`.
pub fn primary(text: &str) -> egui::Button<'static> {
    egui::Button::new(RichText::new(text).strong().color(Color32::WHITE))
        .fill(ROYAL_BLUE)
        .corner_radius(CornerRadius::same(6))
        .min_size(egui::vec2(96.0, 28.0))
}

/// A secondary action, quieter than [`primary`].
pub fn secondary(text: &str) -> egui::Button<'static> {
    egui::Button::new(text.to_string())
        .fill(SURFACE_HIGH)
        .stroke(Stroke::new(1.0, OUTLINE))
        .corner_radius(CornerRadius::same(6))
        .min_size(egui::vec2(96.0, 28.0))
}

/// Adds a [`primary`] button, greyed out when `enabled` is false.
///
/// The button is still drawn and still takes its space, rather than disappearing: a control
/// that vanishes when its inputs are incomplete is harder to find than one that is visibly
/// unavailable, and the space it holds is where the reason would go.
pub fn add_enabled_primary(text: &str, enabled: bool, ui: &mut egui::Ui) -> egui::Response {
    if enabled {
        ui.add(primary(text))
    } else {
        ui.add(
            egui::Button::new(RichText::new(text).color(TEXT_FAINT))
                .fill(SURFACE_HIGH)
                .corner_radius(CornerRadius::same(6))
                .min_size(egui::vec2(96.0, 28.0)),
        )
    }
}

/// One option in a row of exclusive choices, such as the game switch.
///
/// The chosen one is filled in the primary colour and the rest stay quiet, so which is
/// active reads at a glance without comparing labels.
pub fn segment(text: &str, selected: bool) -> egui::Button<'static> {
    let label = if selected {
        RichText::new(text).strong().color(Color32::WHITE)
    } else {
        RichText::new(text).color(TEXT_DIM)
    };
    egui::Button::new(label)
        .fill(if selected { ROYAL_BLUE } else { SURFACE_HIGH })
        .stroke(Stroke::new(1.0, if selected { ROYAL_BLUE_BRIGHT } else { OUTLINE }))
        .corner_radius(CornerRadius::same(6))
        .min_size(egui::vec2(84.0, 28.0))
}

/// A short notice set apart from the page, with an accent bar down its left edge.
///
/// For something the person should read once and not have to hunt for, as opposed to a
/// status line that is replaced by the next thing that happens. The bar is painted, not a
/// stroke on one side of a `Frame`, because egui frames only take a stroke all the way round.
pub fn callout(ui: &mut egui::Ui, accent: Color32, text: &str) {
    let frame = egui::Frame::new()
        .fill(accent.gamma_multiply(0.12))
        .stroke(Stroke::new(1.0, accent.gamma_multiply(0.4)))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(egui::Margin { left: 16, right: 12, top: 9, bottom: 9 });
    let response = frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(RichText::new(text).color(TEXT).size(13.0));
    });
    let rect = response.response.rect;
    let bar = egui::Rect::from_min_size(rect.min, egui::vec2(4.0, rect.height()));
    ui.painter().rect_filled(bar, CornerRadius::same(2), accent);
}

/// A destructive action, so removing a build never looks like adding one.
pub fn danger(text: &str) -> egui::Button<'static> {
    egui::Button::new(text.to_string())
        .fill(Color32::from_rgb(58, 22, 30))
        .stroke(Stroke::new(1.0, Color32::from_rgb(120, 40, 56)))
        .corner_radius(CornerRadius::same(6))
        .min_size(egui::vec2(96.0, 28.0))
}

/// A card: the standard surface for a build row or a group of settings.
pub fn card() -> egui::Frame {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, OUTLINE))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(14.0)
}

/// A heading inside a page, in the secondary accent so it reads as a section.
pub fn heading(text: &str) -> RichText {
    RichText::new(text).size(19.0).strong().color(ROYAL_PURPLE_BRIGHT)
}

/// Text that should read as a section label rather than a value.
pub fn label(text: impl Into<String>) -> RichText {
    RichText::new(text).color(TEXT_FAINT).size(11.0)
}

/// Muted explanatory text.
pub fn muted(text: impl Into<String>) -> RichText {
    RichText::new(text).color(TEXT_DIM)
}

/// A sidebar navigation item.
///
/// The selected item is filled and accented rather than merely highlighted, so the current
/// page is obvious at a glance without reading the labels.
pub fn nav_item(text: &str, selected: bool, width: f32) -> egui::Button<'static> {
    let label = if selected {
        RichText::new(text).strong().color(Color32::WHITE)
    } else {
        RichText::new(text).color(TEXT_DIM)
    };
    egui::Button::new(label)
        .fill(if selected { ROYAL_BLUE } else { Color32::TRANSPARENT })
        .stroke(if selected {
            Stroke::NONE
        } else {
            Stroke::new(1.0, Color32::TRANSPARENT)
        })
        .corner_radius(CornerRadius::same(6))
        // A finite width: an infinite one produces a degenerate rect that paints the fill
        // but lays the label out nowhere, so the item reads as a blank bar.
        .min_size(egui::vec2(width.max(40.0), 30.0))
}

/// The accent bar to the left of the selected navigation item.
///
/// Painted rather than laid out as a widget: there is no fixed height to give it, and it
/// has to line up with whatever height the button beside it ends up.
pub fn nav_row(ui: &mut egui::Ui, text: &str, selected: bool) -> egui::Response {
    let mut clicked = None;
    // Measured before the marker is allocated, so the button takes what is left over and
    // the row comes out flush with the panel's inner edge.
    let width = ui.available_width() - 5.0;
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 20.0), egui::Sense::hover());
        ui.painter().rect_filled(
            rect,
            2.0,
            if selected { ROYAL_PURPLE_BRIGHT } else { Color32::TRANSPARENT },
        );
        clicked = Some(ui.add(nav_item(text, selected, width)));
    });
    // `horizontal` always runs its closure, so this is set.
    clicked.expect("the nav row always adds a button")
}

/// A page title.
pub fn title(text: &str) -> RichText {
    RichText::new(text).size(24.0).strong().color(TEXT)
}

/// A line under a page title explaining what the page is for.
pub fn subtitle(text: impl Into<String>) -> RichText {
    RichText::new(text).color(TEXT_DIM).size(12.5)
}

/// A section rule.
///
/// Painted rather than a `Frame` with a minimum height, because a `Frame` sizes itself from
/// its contents and a frame containing nothing has no height to draw a line in.
pub fn rule(ui: &mut egui::Ui) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 1.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, OUTLINE);
    ui.add_space(8.0);
}

/// A label over a value, for the figures along the top of a page.
///
/// Kept as a pair rather than one string so the number can be larger than its caption,
/// which is what makes a figure scannable.
pub fn stat_value(text: impl std::fmt::Display) -> RichText {
    RichText::new(text.to_string()).size(17.0).strong().color(TEXT)
}

/// A centred message for a page with nothing in it yet.
///
/// Without this, an empty list leaves a blank panel that reads as a broken window rather
/// than as an empty one.
pub fn empty_state(ui: &mut egui::Ui, message: &str, action: &str) {
    ui.add_space(48.0);
    // A minimum height rather than one that fills what is left: an empty state stretched
    // to the bottom of a maximised window looks like a panel waiting to load something,
    // which is the impression this is meant to avoid.
    card().inner_margin(egui::Margin::symmetric(28, 20)).show(ui, |ui| {
        ui.set_min_height(74.0);
        ui.vertical_centered(|ui| {
            ui.label(muted(message));
            ui.add_space(4.0);
            ui.label(label(action));
        });
    });
    ui.add_space(48.0);
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn the_icon_has_one_pixel_of_rgba_per_pixel() {
        for size in [16, 32, 64] {
            assert_eq!(icon_rgba(size).len(), (size * size * 4) as usize);
        }
    }

    /// The corner of a rounded badge is outside it, so it has to be see-through, and the middle
    /// of the ring's bottom edge is on the ring, so it has to be solid.
    #[test]
    fn the_icon_is_transparent_at_the_corner_and_solid_on_the_ring() {
        let size = 64u32;
        let pixels = icon_rgba(size);
        assert_eq!(pixels[3], 0, "the corner should be transparent");
        let ring_y = (size as f32 / 2.0 + size as f32 * 0.30) as u32;
        let at = ((ring_y * size + size / 2) * 4) as usize;
        assert_eq!(pixels[at + 3], 255, "the ring should be opaque");
        assert!(pixels[at] > 200, "the ring should be light, got {}", pixels[at]);
    }

    /// The corner radius is a `u8`, so a large badge could overflow the cast.
    #[test]
    fn the_mark_draws_at_every_size() {
        let ctx = egui::Context::default();
        apply(&ctx);
        let painter =
            egui::Painter::new(ctx.clone(), egui::LayerId::background(), egui::Rect::EVERYTHING);
        for size in [8.0, 16.0, 34.0, 96.0, 256.0, 2048.0] {
            let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(size, size));
            paint_mark(&painter, rect);
        }
    }

    #[test]
    fn the_palette_is_black_blue_and_purple() {
        // The backdrop must be genuinely dark, not a mid grey.
        assert!(BACKDROP.r() < 12 && BACKDROP.g() < 12 && BACKDROP.b() < 16, "{BACKDROP:?}");

        // Royal blue: blue channel clearly dominant.
        assert!(ROYAL_BLUE.b() > ROYAL_BLUE.r() * 3, "{ROYAL_BLUE:?}");
        assert!(ROYAL_BLUE.b() > ROYAL_BLUE.g() * 2, "{ROYAL_BLUE:?}");

        // Royal purple: red and blue both well above green.
        assert!(ROYAL_PURPLE.r() > ROYAL_PURPLE.g() * 2, "{ROYAL_PURPLE:?}");
        assert!(ROYAL_PURPLE.b() > ROYAL_PURPLE.g() * 2, "{ROYAL_PURPLE:?}");
    }

    /// Perceived brightness, used to assert the palette is ordered.
    ///
    /// `Color32` is deliberately not `Ord`, so comparing colours directly will not
    /// compile. Weighting green most closely matches how bright a colour looks.
    fn luma(c: Color32) -> f32 {
        0.2126 * c.r() as f32 + 0.7152 * c.g() as f32 + 0.0722 * c.b() as f32
    }

    #[test]
    fn panels_are_stepped_above_the_backdrop() {
        // Otherwise panels vanish into the window and the layout reads as one slab.
        assert!(luma(SURFACE) > luma(BACKDROP));
        assert!(luma(SURFACE_HIGH) > luma(SURFACE));
    }

    #[test]
    fn text_is_lighter_than_every_surface() {
        assert!(luma(TEXT) > luma(SURFACE_HIGH), "{TEXT:?} vs {SURFACE_HIGH:?}");
        assert!(luma(TEXT_DIM) < luma(TEXT));
        assert!(luma(TEXT_FAINT) < luma(TEXT_DIM));
    }

    #[test]
    fn apply_produces_a_usable_dark_visuals() {
        let ctx = egui::Context::default();
        apply(&ctx);
        let v = ctx.global_style().visuals.clone();
        assert_eq!(v.panel_fill, BACKDROP);
        assert_eq!(v.error_fg_color, DANGER);
        // Hover must be distinguishable from rest, or the UI looks inert.
        assert_ne!(v.widgets.hovered.bg_fill, v.widgets.inactive.bg_fill);
        assert_eq!(v.widgets.active.bg_fill, ROYAL_BLUE);
    }
}
