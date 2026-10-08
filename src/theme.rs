//! Design tokens and the iced theme — the same language as PrintCraft's `crates/ui-egui/src/theme.rs`:
//! neutral chrome, white panels, one blue accent, and a Dark Gray theme that keeps pages white.
//!
//! Every custom widget in this demo reads [`Tokens::for_kind`]; nothing hard-codes a colour.

use iced::font::{Family, Stretch, Style as FontStyle, Weight};
use iced::theme::Palette;
use iced::{Color, Font, Theme};

/// Which of the two designed themes is active.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ThemeKind {
    #[default]
    Light,
    Dark,
}

/// The full token set. Field names and values mirror PrintCraft's `Tokens`.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub kind: ThemeKind,
    /// Title/tab strip.
    pub titlebar: Color,
    /// Mode bar, rails.
    pub chrome: Color,
    /// Panels and their headers.
    pub panel: Color,
    /// Workspace behind pages.
    pub pasteboard: Color,
    /// Cards, popups, the page itself.
    pub card: Color,
    pub border: Color,
    pub divider: Color,
    pub text: Color,
    pub text_muted: Color,
    pub text_faint: Color,
    pub icon: Color,
    pub hover: Color,
    pub pressed: Color,
    pub selected: Color,
    pub accent: Color,
    pub accent_text: Color,
    pub accent_soft: Color,
    pub field: Color,
    /// Shadow cast by a page on the pasteboard.
    pub page_shadow: Color,
    pub radius: f32,
}

impl Tokens {
    pub fn for_kind(kind: ThemeKind) -> Self {
        match kind {
            ThemeKind::Light => Self {
                kind,
                titlebar: Color::from_rgb8(0xE9, 0xE9, 0xEB),
                chrome: Color::from_rgb8(0xFF, 0xFF, 0xFF),
                panel: Color::from_rgb8(0xFF, 0xFF, 0xFF),
                pasteboard: Color::from_rgb8(0xF1, 0xF1, 0xF3),
                card: Color::from_rgb8(0xFF, 0xFF, 0xFF),
                border: Color::from_rgb8(0xDA, 0xDA, 0xDE),
                divider: Color::from_rgb8(0xE8, 0xE8, 0xEB),
                text: Color::from_rgb8(0x22, 0x22, 0x26),
                text_muted: Color::from_rgb8(0x5E, 0x5E, 0x66),
                text_faint: Color::from_rgb8(0x6B, 0x6B, 0x73),
                icon: Color::from_rgb8(0x44, 0x44, 0x4B),
                hover: Color::from_rgb8(0xF0, 0xF0, 0xF3),
                pressed: Color::from_rgb8(0xE4, 0xE4, 0xE9),
                selected: Color::from_rgb8(0xE6, 0xEE, 0xFD),
                accent: Color::from_rgb8(0x1B, 0x63, 0xE0),
                accent_text: Color::from_rgb8(0x17, 0x55, 0xC4),
                accent_soft: Color::from_rgb8(0xE3, 0xEC, 0xFD),
                field: Color::from_rgb8(0xFF, 0xFF, 0xFF),
                page_shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.13),
                radius: 6.0,
            },
            ThemeKind::Dark => Self {
                kind,
                titlebar: Color::from_rgb8(0x1B, 0x1B, 0x1E),
                chrome: Color::from_rgb8(0x26, 0x26, 0x2A),
                panel: Color::from_rgb8(0x26, 0x26, 0x2A),
                pasteboard: Color::from_rgb8(0x19, 0x19, 0x1C),
                card: Color::from_rgb8(0x2E, 0x2E, 0x33),
                border: Color::from_rgb8(0x3C, 0x3C, 0x43),
                divider: Color::from_rgb8(0x33, 0x33, 0x39),
                text: Color::from_rgb8(0xEC, 0xEC, 0xEF),
                text_muted: Color::from_rgb8(0xAE, 0xAE, 0xB6),
                text_faint: Color::from_rgb8(0x97, 0x97, 0x9E),
                icon: Color::from_rgb8(0xD4, 0xD4, 0xDA),
                hover: Color::from_rgb8(0x34, 0x34, 0x3A),
                pressed: Color::from_rgb8(0x3E, 0x3E, 0x45),
                selected: Color::from_rgb8(0x23, 0x3A, 0x63),
                accent: Color::from_rgb8(0x4B, 0x8B, 0xF5),
                accent_text: Color::from_rgb8(0x8C, 0xB6, 0xFA),
                accent_soft: Color::from_rgb8(0x24, 0x36, 0x57),
                field: Color::from_rgb8(0x1E, 0x1E, 0x22),
                page_shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.47),
                radius: 6.0,
            },
        }
    }

    pub fn dark(&self) -> bool {
        self.kind == ThemeKind::Dark
    }

    /// The base palette handed to iced so built-in widget defaults stay coherent.
    pub fn palette(&self) -> Palette {
        Palette {
            background: self.panel,
            text: self.text,
            primary: self.accent,
            success: Color::from_rgb8(0x2D, 0x9D, 0x5B),
            warning: Color::from_rgb8(0xE8, 0x8A, 0x1A),
            danger: Color::from_rgb8(0xE0, 0x3E, 0x3E),
        }
    }

    pub fn to_theme(self) -> Theme {
        Theme::custom(
            if self.dark() { "PrintCraft Dark Gray" } else { "PrintCraft Light Gray" },
            self.palette(),
        )
    }

}

/// The interface font family, as PrintCraft registers it: Inter first, JetBrains Mono for code.
pub const INTER: &str = "Inter";
pub const MONO: &str = "JetBrains Mono";

/// The static font files shipped with the app (the same OFL faces PrintCraft bundles).
pub fn fonts() -> [&'static [u8]; 4] {
    [
        include_bytes!("../assets/fonts/Inter-Regular.ttf").as_slice(),
        include_bytes!("../assets/fonts/Inter-Medium.ttf").as_slice(),
        include_bytes!("../assets/fonts/Inter-SemiBold.ttf").as_slice(),
        include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf").as_slice(),
    ]
}

/// Regular weight (400).
pub fn regular() -> Font {
    Font::with_name(INTER)
}

/// Medium weight (500) — labels, buttons, tab names.
pub fn medium() -> Font {
    Font { family: Family::Name(INTER), weight: Weight::Medium, stretch: Stretch::Normal, style: FontStyle::Normal }
}

/// Semi-bold weight (600) — section and page headings.
pub fn semibold() -> Font {
    Font { family: Family::Name(INTER), weight: Weight::Semibold, stretch: Stretch::Normal, style: FontStyle::Normal }
}

/// Monospaced face for document IDs, sizes and the code-like specimen pages.
pub fn mono() -> Font {
    Font::with_name(MONO)
}
