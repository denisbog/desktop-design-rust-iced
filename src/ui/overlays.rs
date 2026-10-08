//! Transient surfaces: the ⌘K command palette, the modal dialogs and the toast.
//!
//! Each one is a full-window layer pushed onto the root [`iced::widget::Stack`], so it floats over
//! the chrome without disturbing the layout underneath.

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::app::{App, Dialog, Message, PropsTab};
use crate::catalog;
use crate::icons;
use crate::theme::{self, Tokens};
use crate::widgets;

/// The id of the palette field, so ⌘K can focus it.
pub const PALETTE_INPUT: &str = "palette-input";

// ---------------------------------------------------------------------------------------------
// Command palette
// ---------------------------------------------------------------------------------------------

/// One palette result: a command, a tool family, or a tool item.
#[derive(Clone, Debug)]
pub struct Hit {
    pub group: Option<&'static str>,
    pub command: Option<String>,
    pub label: String,
    pub detail: String,
    pub icon: &'static str,
    pub ready: bool,
}

/// Substring first, then a subsequence fallback — the same forgiving match PrintCraft uses.
fn score(hay: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    let hay = hay.to_lowercase();
    if let Some(pos) = hay.find(needle) {
        return Some(pos);
    }
    let mut chars = hay.chars();
    needle.chars().all(|c| chars.any(|x| x == c)).then_some(100)
}

/// Every palette result for `query`, best first (commands before tools on a tie).
pub fn hits(query: &str) -> Vec<Hit> {
    let needle = query.trim().to_lowercase();
    let mut hits: Vec<(usize, Hit)> = Vec::new();

    for command in catalog::COMMANDS {
        if let Some(s) = score(command.label, &needle).or_else(|| score(command.id, &needle).map(|s| s + 50)) {
            hits.push((
                s,
                Hit {
                    group: None,
                    command: Some(command.id.to_string()),
                    label: command.label.to_string(),
                    detail: command.shortcut.unwrap_or(command.menu).to_string(),
                    icon: command.icon,
                    ready: true,
                },
            ));
        }
    }

    for group in catalog::TOOL_GROUPS {
        if let Some(s) = score(group.label, &needle) {
            hits.push((
                s + 20,
                Hit {
                    group: Some(group.id),
                    command: None,
                    label: group.label.to_string(),
                    detail: "Tool".to_string(),
                    icon: group.icon,
                    ready: group.availability.is_ready(),
                },
            ));
        }
        for section in group.sections {
            for item in section.items {
                if let Some(s) = score(item.label, &needle).or_else(|| score(item.command, &needle).map(|s| s + 50)) {
                    hits.push((
                        s + 21,
                        Hit {
                            group: Some(group.id),
                            command: Some(item.command.to_string()),
                            label: item.label.to_string(),
                            detail: group.label.to_string(),
                            icon: item.icon,
                            ready: item.availability.is_ready(),
                        },
                    ));
                }
            }
        }
    }

    hits.sort_by_key(|(s, hit)| (*s, !hit.ready));
    hits.into_iter().take(12).map(|(_, hit)| hit).collect()
}

/// The message a hit dispatches when chosen.
pub fn hit_message(hit: &Hit) -> Message {
    if let Some(command) = &hit.command {
        Message::PalettePick(command.clone())
    } else if let Some(group) = hit.group {
        Message::OpenLeft(crate::app::LeftPanel::Tool(group))
    } else {
        Message::PaletteClose
    }
}


pub fn palette(app: &App, t: Tokens) -> Element<'_, Message> {
    let results = hits(&app.palette_query);

    let field = text_input("Search tools and commands…", &app.palette_query)
        .id(PALETTE_INPUT)
        .on_input(Message::PaletteQuery)
        .on_submit(Message::PaletteSubmit)
        .font(theme::regular())
        .size(15.0)
        .padding(0)
        .width(Length::Fill)
        .style(move |_theme, _status| text_input::Style {
            background: Color::TRANSPARENT.into(),
            border: Border::default(),
            icon: t.text_muted,
            placeholder: t.text_muted,
            value: t.text,
            selection: t.accent_soft,
        });

    let mut list = column![].spacing(1.0).width(Length::Fill);
    for hit in &results {
        let fg = if hit.ready { t.text } else { t.text_faint };
        list = list.push(
            button(
                row![
                    icons::icon::<Message>(hit.icon, 17.0, if hit.ready { t.icon } else { t.text_faint }),
                    text(hit.label.clone()).font(theme::regular()).size(13.5).color(fg).width(Length::Fill),
                    text(hit.detail.clone()).font(theme::regular()).size(12.0).color(t.text_faint),
                ]
                .spacing(12.0)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding(Padding { top: 7.0, right: 8.0, bottom: 7.0, left: 8.0 })
            .on_press(hit_message(hit))
            .style(move |_theme, status| button::Style {
                background: Some((if matches!(status, button::Status::Hovered) { t.hover } else { Color::TRANSPARENT }).into()),
                text_color: fg,
                border: Border { radius: 6.0.into(), ..Default::default() },
                ..Default::default()
            }),
        );
    }
    if results.is_empty() {
        list = list.push(container(text("No matching tools").font(theme::regular()).size(13.0).color(t.text_muted)).padding(8));
    }

    let panel = container(
        column![
            row![
                icons::icon::<Message>("search", 18.0, t.text_muted),
                field,
                widgets::badge("Esc", t.hover, t.text_faint),
            ]
            .spacing(10.0)
            .align_y(Alignment::Center),
            widgets::h_divider(t),
            widgets::vspace(4.0),
            widgets::scrolled(list, t).height(Length::Fixed(360.0)),
            widgets::vspace(4.0),
        ]
        .spacing(8.0),
    )
    .width(Length::Fixed(560.0))
    .padding(10)
    .style(move |_| container::Style {
        background: Some(t.card.into()),
        border: Border { color: t.border, width: 1.0, radius: 12.0.into() },
        shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.22), offset: iced::Vector::new(0.0, 10.0), blur_radius: 30.0 },
        ..Default::default()
    });

    // A scrim under the panel: clicking anywhere else closes the palette.
    let scrim = button(iced::widget::space())
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::PaletteClose)
        .style(|_theme, _status| button::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.08).into()),
            ..Default::default()
        });

    let positioned = container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding { top: 96.0, right: 0.0, bottom: 0.0, left: 0.0 })
        .align_x(Alignment::Center)
        .align_y(Alignment::Start);

    iced::widget::stack![scrim, positioned].into()
}

// ---------------------------------------------------------------------------------------------
// Dialogs
// ---------------------------------------------------------------------------------------------

pub fn dialog(app: &App, t: Tokens) -> Element<'_, Message> {
    let Some(dialog) = app.dialog else {
        return container(iced::widget::space()).into();
    };

    let body: Element<'_, Message> = match dialog {
        Dialog::Properties(tab) => properties(app, tab, t),
        Dialog::Shortcuts => shortcuts(t),
        Dialog::About => about(t),
    };

    let title = match dialog {
        Dialog::Properties(tab) => format!("Document properties — {}", tab.label()),
        Dialog::Shortcuts => "Keyboard shortcuts".to_string(),
        Dialog::About => "About PrintCraft".to_string(),
    };

    let card = container(
        column![
            row![
                text(title).font(theme::semibold()).size(15.0).color(t.text),
                iced::widget::space::horizontal(),
                widgets::icon_button("x", 26.0, false, true, t, Message::CloseDialog),
            ]
            .align_y(Alignment::Center),
            widgets::h_divider(t),
            widgets::scrolled(container(body).padding(Padding { top: 14.0, right: 0.0, bottom: 14.0, left: 0.0 }), t)
                .height(Length::Fixed(400.0)),
            widgets::h_divider(t),
            row![
                iced::widget::space::horizontal(),
                widgets::pill_button("Close", false, t, Message::CloseDialog),
            ]
            .padding(Padding { top: 8.0, right: 0.0, bottom: 0.0, left: 0.0 }),
        ]
        .spacing(10.0),
    )
    .width(Length::Fixed(560.0))
    .padding(16)
    .style(move |_| container::Style {
        background: Some(t.card.into()),
        border: Border { color: t.border, width: 1.0, radius: 12.0.into() },
        shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.24), offset: iced::Vector::new(0.0, 12.0), blur_radius: 34.0 },
        ..Default::default()
    });

    // Scrim + centered card.
    container(
        container(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_| container::Style { background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.28).into()), ..Default::default() })
    .into()
}

/// The Document properties dialog, with its tab strip and per-tab content.
fn properties(app: &App, tab: PropsTab, t: Tokens) -> Element<'_, Message> {
    let mut tabs = row![].spacing(4.0);
    for candidate in PropsTab::ALL {
        tabs = tabs.push(widgets::mode_tab_small(candidate.label(), candidate == tab, t, Message::SetPropsTab(candidate)));
    }

    let doc = app.doc();
    let rows: Vec<(&str, String)> = match tab {
        PropsTab::Description => vec![
            ("File", doc.map(|d| d.name.clone()).unwrap_or_else(|| "—".into())),
            ("Title", "PrintCraft Showcase".into()),
            ("Location", doc.and_then(|d| d.path.clone()).unwrap_or_else(|| "Not saved yet".into())),
            ("Author", "The ArtCraft Team".into()),
            ("Subject", "A 13-page specimen of PrintCraft's typography".into()),
            ("Keywords", "pdf, typography, specimen, rust".into()),
            ("Page count", doc.map(|d| d.pages.to_string()).unwrap_or_else(|| "—".into())),
            ("Page size", "612 × 792 pt (US Letter)".into()),
            ("File size", doc.map(|d| d.size.clone()).unwrap_or_else(|| "—".into())),
            ("Fast web view", "Yes".into()),
        ],
        PropsTab::InitialView => vec![
            ("Navigation tab", "Bookmarks panel and page".into()),
            ("Page layout", "Single page, continuous".into()),
            ("Magnification", "Fit width".into()),
            ("Open in full screen", "No".into()),
        ],
        PropsTab::Security => vec![
            ("Security method", "No security".into()),
            ("Document restriction summary", "All permissions allowed".into()),
            ("Signatures", if doc.map(|d| d.signed).unwrap_or(false) { "1 signature".into() } else { "No signatures".into() }),
        ],
        PropsTab::Fonts => vec![
            ("Inter", "TrueType, embedded subset".into()),
            ("JetBrains Mono", "TrueType, embedded subset".into()),
            ("Noto Sans CJK JP", "CID font, embedded".into()),
        ],
    };

    let mut table = column![].spacing(0.0).width(Length::Fill);
    for (i, (key, value)) in rows.iter().enumerate() {
        table = table.push(
            container(
                row![
                    text(*key).font(theme::medium()).size(12.5).color(t.text_muted).width(Length::Fixed(180.0)),
                    text(value.clone()).font(theme::regular()).size(12.5).color(t.text).width(Length::Fill),
                ]
                .spacing(8.0),
            )
            .width(Length::Fill)
            .padding(Padding { top: 7.0, right: 8.0, bottom: 7.0, left: 8.0 })
            .style(move |_| container::Style {
                background: Some((if i % 2 == 0 { t.panel } else { t.field }).into()),
                ..Default::default()
            }),
        );
    }

    column![tabs, widgets::vspace(10.0), table].spacing(2.0).into()
}

/// The shortcuts dialog: two columns of key/description rows.
fn shortcuts(t: Tokens) -> Element<'static, Message> {
    const SHORTCUTS: [(&str, &str); 14] = [
        ("⌘K", "Search tools and commands"),
        ("⌘O", "Open a PDF"),
        ("⌘S", "Save"),
        ("⌘P", "Print"),
        ("⌘D", "Document properties"),
        ("⌘F", "Find in document"),
        ("⌘G", "Find next"),
        ("⇧⌘G", "Find previous"),
        ("⌘1", "Actual size"),
        ("⌘0", "Zoom to page level"),
        ("⌘2", "Fit to width"),
        ("⇧⌘+", "Rotate view clockwise"),
        ("⌘[", "Previous view"),
        ("⌘]", "Next view"),
    ];

    let mut grid = column![].spacing(2.0);
    for chunk in SHORTCUTS.chunks(2) {
        let mut line = row![].spacing(20.0);
        for (keys, description) in chunk {
            line = line.push(
                row![
                    widgets::badge(keys, t.hover, t.text),
                    text(*description).font(theme::regular()).size(12.5).color(t.text),
                ]
                .spacing(10.0)
                .align_y(Alignment::Center)
                .width(Length::Fill),
            );
        }
        grid = grid.push(line);
    }
    grid.into()
}

/// The About dialog: version, blurb and the community action.
fn about(t: Tokens) -> Element<'static, Message> {
    column![
        text("PrintCraft").font(theme::semibold()).size(20.0).color(t.text),
        text("The PDF workbench — an open-source, clean-room reimplementation of Adobe Acrobat, rebuilt in pure Rust.")
            .font(theme::regular())
            .size(13.0)
            .color(t.text_muted),
        widgets::vspace(8.0),
        text("Version 0.2.1 · MIT OR Apache-2.0 · no account, no telemetry").font(theme::regular()).size(12.0).color(t.text_faint),
        widgets::vspace(12.0),
        row![
            widgets::icon_pill("messages-square", "Join our Discord", true, t, Message::Notify("Opening discord.gg/artcraft".into())),
            widgets::icon_pill("book-open", "Read the handbook", false, t, Message::Notify("Opening the handbook".into())),
        ]
        .spacing(8.0),
    ]
    .spacing(6.0)
    .into()
}

// ---------------------------------------------------------------------------------------------
// Toast
// ---------------------------------------------------------------------------------------------

/// A transient message at the bottom centre; it fades away after 3.5 seconds.
pub fn toast(app: &App, t: Tokens) -> Element<'_, Message> {
    let message = app.toast.as_ref().map(|(message, _)| message.clone()).unwrap_or_default();

    container(
        container(text(message).font(theme::regular()).size(13.0).color(if t.dark() { Color::from_rgb8(0x22, 0x22, 0x26) } else { Color::WHITE }))
            .padding(Padding { top: 10.0, right: 16.0, bottom: 10.0, left: 16.0 })
            .style(move |_| container::Style {
                background: Some(if t.dark() { Color::from_rgb8(0xEC, 0xEC, 0xEF) } else { Color::from_rgb8(0x2A, 0x2A, 0x2F) }.into()),
                border: Border { radius: 8.0.into(), ..Default::default() },
                shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.28), offset: iced::Vector::new(0.0, 6.0), blur_radius: 20.0 },
                ..Default::default()
            }),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(Padding { top: 0.0, right: 0.0, bottom: 28.0, left: 0.0 })
    .align_x(Alignment::Center)
    .align_y(Alignment::End)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{LeftPanel, Message};

    #[test]
    fn score_prefers_substrings_and_falls_back_to_subsequences() {
        assert_eq!(score("Organize pages", "org"), Some(0));
        assert!(score("Organize pages", "zpa").is_some(), "subsequence should match");
        assert!(score("Organize pages", "qqq").is_none());
        assert_eq!(score("anything", ""), Some(0));
    }

    #[test]
    fn palette_lists_and_filters() {
        let all = hits("");
        assert!(!all.is_empty());
        assert!(all.len() <= 12, "the palette caps its result list");

        let organize = hits("organize");
        assert!(!organize.is_empty());
        assert!(
            organize[0].label.to_lowercase().contains("organize"),
            "best hit should be the organize entry, got {:?}",
            organize[0].label
        );
        assert!(organize.iter().any(|hit| hit.ready));
    }

    #[test]
    fn hits_map_to_dispatchable_messages() {
        let group = Hit {
            group: Some("organize"),
            command: None,
            label: "Organize pages".into(),
            detail: "Tool".into(),
            icon: "layout-grid",
            ready: true,
        };
        assert!(matches!(hit_message(&group), Message::OpenLeft(LeftPanel::Tool("organize"))));

        let command = Hit { group: Some("organize"), command: Some("page.rotate".into()), ..group.clone() };
        assert!(matches!(hit_message(&command), Message::PalettePick(id) if id == "page.rotate"));
    }
}
