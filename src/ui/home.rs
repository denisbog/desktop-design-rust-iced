//! The Home tab: the community card, recommended tools and recent files.
//!
//! Shown when no document tab is active; the recommended tools come straight from the catalogue.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::app::{App, Dialog, LeftPanel, Message};
use crate::catalog;
use crate::icons;
use crate::theme::{self, Tokens};
use crate::widgets;

/// The five tool families featured on the home screen, as in PrintCraft.
const RECOMMENDED: [&str; 5] = ["organize", "comment", "fill_sign", "edit", "compress"];

pub fn view(app: &App, t: Tokens) -> Element<'_, Message> {
    let mut page = column![].spacing(0.0).width(Length::Fill);

    page = page.push(text("Welcome to PrintCraft").font(theme::semibold()).size(24.0).color(t.text));
    page = page.push(
        text("An open-source PDF workbench — local, private, and scriptable.")
            .font(theme::regular())
            .size(14.0)
            .color(t.text_muted),
    );
    page = page.push(widgets::vspace(14.0));

    // Community card.
    page = page.push(
        container(
            column![
                row![
                    mark(t),
                    column![
                        text("Join the ArtCraft community").font(theme::semibold()).size(15.0).color(t.text),
                        text("Get help, share feedback and follow development on Discord.")
                            .font(theme::regular())
                            .size(12.5)
                            .color(t.text_muted),
                    ]
                    .spacing(2.0),
                ]
                .spacing(10.0)
                .align_y(Alignment::Center),
                widgets::vspace(10.0),
                row![
                    widgets::icon_pill("messages-square", "Join our Discord", true, t, Message::Notify("Opening discord.gg/artcraft".into())),
                    widgets::icon_pill("book-open", "Documentation", false, t, Message::Notify("Opening the handbook".into())),
                    widgets::icon_pill("circle-help", "Keyboard shortcuts", false, t, Message::OpenDialog(Dialog::Shortcuts)),
                ]
                .spacing(8.0)
                .wrap(),
            ]
            .spacing(4.0),
        )
        .width(Length::Fill)
        .padding(14)
        .style(move |_| widgets::card_style(t)),
    );

    page = page.push(widgets::vspace(20.0));

    // Recommended tools.
    let mut cards = row![].spacing(14.0);
    for id in RECOMMENDED {
        if let Some(group) = catalog::ToolGroup::find(id) {
            cards = cards.push(tool_card(group, t));
        }
    }
    cards = cards.push(open_card(t));

    page = page.push(
        container(
            column![
                text("Recommended tools").font(theme::semibold()).size(15.0).color(t.text),
                widgets::vspace(12.0),
                cards.wrap(),
            ]
            .spacing(0.0),
        )
        .width(Length::Fill)
        .padding(18)
        .style(move |_| widgets::card_style(t)),
    );

    page = page.push(widgets::vspace(24.0));

    // Recent files.
    page = page.push(text("Recent").font(theme::semibold()).size(17.0).color(t.text));
    page = page.push(widgets::vspace(8.0));
    if app.recent.is_empty() {
        page = page.push(
            text("Files you open in PrintCraft appear here. Drop a PDF anywhere to open it.")
                .font(theme::regular())
                .size(13.0)
                .color(t.text_muted),
        );
    }
    for recent in &app.recent {
        page = page.push(recent_row(recent.name.as_str(), recent.path.as_str(), recent.pages, recent.size.as_str(), t));
    }

    page = page.push(widgets::vspace(20.0));
    page = page.push(widgets::section_title("Privacy", t));
    page = page.push(
        text("PrintCraft works offline. No telemetry, no account, and no cloud processing unless you add a provider.")
            .font(theme::regular())
            .size(13.0)
            .color(t.text_muted),
    );

    widgets::scrolled(container(page).padding(Padding { top: 28.0, right: 36.0, bottom: 28.0, left: 36.0 }), t)
        .height(Length::Fill)
        .into()
}

/// The ArtCraft-style app mark: an accent square with a white glyph.
fn mark<'a>(t: Tokens) -> Element<'a, Message> {
    container(icons::icon::<Message>("layers", 15.0, Color::WHITE))
        .width(28.0)
        .height(28.0)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_| container::Style {
            background: Some(t.accent.into()),
            border: Border { radius: 8.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

/// A recommended-tool card: colour-coded icon, label, a blurb and a "Use now" action.
fn tool_card<'a>(group: &'static catalog::ToolGroup, t: Tokens) -> Element<'a, Message> {
    let hue = Color::from_rgb8(group.hue[0], group.hue[1], group.hue[2]);

    button(
        column![
            icons::icon::<Message>(group.icon, 19.0, hue),
            widgets::vspace(6.0),
            text(group.label).font(theme::semibold()).size(13.5).color(t.text),
            text(group.blurb()).font(theme::regular()).size(11.5).color(t.text_muted),
            iced::widget::space::vertical(),
            text("Use now").font(theme::medium()).size(12.0).color(t.accent_text),
        ]
        .spacing(0.0)
        .width(Length::Fill),
    )
    .width(Length::Fixed(196.0))
    .height(Length::Fixed(116.0))
    .padding(14)
    .on_press(Message::OpenLeft(LeftPanel::Tool(group.id)))
    .style(move |_theme, status| button::Style {
        background: Some((if matches!(status, button::Status::Hovered) { t.hover } else { t.card }).into()),
        text_color: t.text,
        border: Border { color: t.divider, width: 1.0, radius: 10.0.into() },
        ..Default::default()
    })
    .into()
}

/// The "Open file" card, sized like the tool cards.
fn open_card<'a>(t: Tokens) -> Element<'a, Message> {
    button(
        column![
            icons::icon::<Message>("folder-open", 22.0, t.icon),
            widgets::vspace(8.0),
            text("Open file").font(theme::semibold()).size(13.0).color(t.text),
        ]
        .align_x(Alignment::Center)
        .width(Length::Fill),
    )
    .width(Length::Fixed(170.0))
    .height(Length::Fixed(116.0))
    .padding(14)
    .on_press(Message::OpenRequested)
    .style(move |_theme, status| button::Style {
        background: Some((if matches!(status, button::Status::Hovered) { t.hover } else { t.pasteboard }).into()),
        text_color: t.text,
        border: Border { color: t.divider, width: 1.0, radius: 10.0.into() },
        ..Default::default()
    })
    .into()
}

/// One recent file: a red PDF glyph, name and path, then the page count and size.
fn recent_row<'a>(name: &'a str, path: &'a str, pages: usize, size: &str, t: Tokens) -> Element<'a, Message> {
    button(
        row![
            icons::icon::<Message>("file-text", 19.0, Color::from_rgb8(0xE0, 0x3E, 0x3E)),
            column![
                text(name).font(theme::medium()).size(13.5).color(t.text),
                text(path).font(theme::regular()).size(11.0).color(t.text_faint),
            ]
            .spacing(1.0)
            .width(Length::Fill),
            text(format!("{pages} pages  ·  {size}")).font(theme::regular()).size(12.0).color(t.text_muted),
        ]
        .spacing(14.0)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding(Padding { top: 8.0, right: 10.0, bottom: 8.0, left: 10.0 })
    .on_press(Message::Notify(format!("Opening {name}")))
    .style(move |_theme, status| button::Style {
        background: Some((if matches!(status, button::Status::Hovered) { t.hover } else { Color::TRANSPARENT }).into()),
        text_color: t.text,
        border: Border { radius: 8.0.into(), ..Default::default() },
        ..Default::default()
    })
    .into()
}
