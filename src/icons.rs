//! Vector icons: the embedded Lucide SVGs, tinted per use and drawn through iced's `svg` widget.
//!
//! PrintCraft's rule — never use Unicode glyphs as icons — holds here too. The table is generated
//! from `assets/icons/` by `build.rs`; [`handle`] falls back to a neutral square so a typo in a
//! catalogue row cannot panic at runtime.

use iced::widget::svg;
use iced::{Color, Element, Length};

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

/// The tinted vector handle for `name`, or a neutral square when the name is unknown.
pub fn handle(name: &str) -> svg::Handle {
    let bytes = ICONS
        .iter()
        .find(|(n, _)| *n == name)
        .or_else(|| ICONS.iter().find(|(n, _)| *n == "square"))
        .map(|(_, b)| *b)
        .unwrap_or(b"<svg xmlns='http://www.w3.org/2000/svg'/>".as_slice());
    svg::Handle::from_memory(bytes)
}

/// Whether an icon with this name is embedded (asserted by the catalogue test below).
#[cfg_attr(not(test), allow(dead_code))]
pub fn exists(name: &str) -> bool {
    ICONS.iter().any(|(n, _)| *n == name)
}

/// A tinted icon, exactly `size` points square.
pub fn icon<'a, Message: 'a>(name: &str, size: f32, tint: Color) -> Element<'a, Message> {
    svg(handle(name))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |_, _| svg::Style { color: Some(tint) })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every icon the catalogue names must be embedded, or a row would silently fall back.
    #[test]
    fn every_catalog_icon_exists() {
        for group in crate::catalog::TOOL_GROUPS {
            assert!(exists(group.icon), "missing group icon {}", group.icon);
            for section in group.sections {
                for item in section.items {
                    assert!(exists(item.icon), "missing item icon {}", item.icon);
                }
            }
        }
        for command in crate::catalog::COMMANDS {
            assert!(exists(command.icon), "missing command icon {}", command.icon);
        }
        for panel in [
            crate::app::RightPanel::Comments,
            crate::app::RightPanel::Bookmarks,
            crate::app::RightPanel::Pages,
            crate::app::RightPanel::Fields,
            crate::app::RightPanel::Layers,
            crate::app::RightPanel::Attachments,
            crate::app::RightPanel::Signatures,
        ] {
            assert!(exists(panel.icon()), "missing rail icon {}", panel.icon());
        }
    }
}
