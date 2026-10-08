//! The right panel: Comments, Bookmarks, Page thumbnails, Form fields, Layers and Attachments.
//!
//! Which panel is shown comes from [`App::right`]; every panel is a thin projection of the active
//! document's model, and each one has an explicit empty state.

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::app::{App, Comment, Field, Layer, Message, OutlineItem, RightPanel};
use crate::icons;
use crate::theme::{self, Tokens};
use crate::widgets;

/// Width of the right panel, in points (PrintCraft uses 300).
pub const WIDTH: f32 = 300.0;

pub fn view(app: &App, t: Tokens) -> Element<'_, Message> {
    let Some(panel) = app.right else {
        return container(iced::widget::space()).into();
    };
    let Some(doc) = app.doc() else {
        return shell(panel.title(), panel, empty(t, "file-text", "Open a document to see this panel"), t);
    };

    let body: Element<'_, Message> = match panel {
        RightPanel::Comments => comments(app, t),
        RightPanel::Bookmarks => bookmarks(doc.outline.as_slice(), t),
        RightPanel::Pages => pages(app, t),
        RightPanel::Fields => fields(doc.fields.as_slice(), t),
        RightPanel::Layers => layers(doc.layers.as_slice(), t),
        RightPanel::Attachments => attachments(app, t),
        RightPanel::Signatures => {
            if doc.signed {
                widgets::section_title("This document is signed", t)
            } else {
                empty(t, "signature", "Not signed. Use “Use a certificate” to add a signature.")
            }
        }
    };

    shell(panel.title(), panel, body, t)
}

/// The panel frame: header with a close button, then a scrolled body.
fn shell<'a>(title: &'a str, panel: RightPanel, body: Element<'a, Message>, t: Tokens) -> Element<'a, Message> {
    let header = container(
        row![
            text(title).font(theme::semibold()).size(13.5).line_height(1.2).color(t.text),
            iced::widget::space::horizontal(),
            widgets::tip(widgets::icon_button("x", 24.0, false, true, t, Message::ToggleRight(panel)), "Close panel", t),
        ]
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fixed(40.0))
    .padding(Padding { top: 0.0, right: 8.0, bottom: 0.0, left: 12.0 })
    .align_y(Alignment::Center);

    container(
        row![
            widgets::v_divider(t),
            column![
                header,
                widgets::h_divider(t),
                widgets::scrolled(container(body).padding(Padding { top: 10.0, right: 12.0, bottom: 16.0, left: 12.0 }), t).height(Length::Fill),
            ]
            .height(Length::Fill),
        ]
        .height(Length::Fill),
    )
    .width(Length::Fixed(WIDTH))
    .height(Length::Fill)
    .style(move |_| container::Style { background: Some(t.panel.into()), ..Default::default() })
    .into()
}

/// An empty state: a muted icon over an explanation.
fn empty<'a>(t: Tokens, icon: &str, message: &'a str) -> Element<'a, Message> {
    container(
        column![
            icons::icon::<Message>(icon, 30.0, t.text_faint),
            text(message).font(theme::regular()).size(12.5).color(t.text_muted).center(),
        ]
        .spacing(8.0)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .padding(Padding { top: 32.0, right: 0.0, bottom: 0.0, left: 0.0 })
    .center_x(Length::Fill)
    .into()
}

// ---------------------------------------------------------------------------------------------
// Comments
// ---------------------------------------------------------------------------------------------

fn comments(app: &App, t: Tokens) -> Element<'_, Message> {
    let composer = column![
        text("Threaded comments").font(theme::semibold()).size(11.0).color(t.text_faint),
        text_input("Add a comment…", &app.comment_draft)
            .on_input(Message::CommentDraft)
            .on_submit(Message::AddComment)
            .font(theme::regular())
            .size(13.0)
            .padding(8)
            .style(move |_theme, status| text_input::Style {
                background: t.field.into(),
                border: Border {
                    color: if matches!(status, text_input::Status::Focused { .. }) { t.accent } else { t.border },
                    width: 1.0,
                    radius: 6.0.into()
                },
                icon: t.text_muted,
                placeholder: t.text_faint,
                value: t.text,
                selection: t.accent_soft,
            }),
        row![
            iced::widget::space::horizontal(),
            widgets::pill_button("Comment", true, t, Message::AddComment)
        ],
    ]
    .spacing(6.0);

    let Some(doc) = app.doc() else {
        return composer.into();
    };

    let mut list = column![].spacing(10.0);
    for (index, comment) in doc.comments.iter().enumerate() {
        list = list.push(comment_card(index, comment, t));
    }

    column![composer, widgets::vspace(6.0), widgets::h_divider(t), widgets::vspace(6.0), list]
        .spacing(2.0)
        .width(Length::Fill)
        .into()
}

/// One comment thread: avatar, author, page/time, body, replies and a resolve toggle.
fn comment_card<'a>(index: usize, comment: &'a Comment, t: Tokens) -> Element<'a, Message> {
    let mut thread = column![].spacing(6.0);

    for (body, author) in &comment.replies {
        thread = thread.push(
            row![
                container(container(iced::widget::space()).width(3.0).height(Length::Fill).style(move |_| container::Style {
                    background: Some(t.border.into()),
                    ..Default::default()
                }))
                .width(Length::Fixed(3.0))
                .height(Length::Fill),
                column![
                    text(*author).font(theme::medium()).size(11.5).color(t.text_muted),
                    text(body).font(theme::regular()).size(12.0).color(t.text),
                ]
                .spacing(1.0),
            ]
            .spacing(8.0),
        );
    }

    let header = row![
        avatar(comment.initials, t),
        column![
            text(comment.author).font(theme::semibold()).size(12.5).color(t.text),
            text(format!("Page {} · {}", comment.page, comment.time)).font(theme::regular()).size(11.0).color(t.text_faint),
        ]
        .spacing(1.0),
        iced::widget::space::horizontal(),
        widgets::badge(
            if comment.resolved { "Resolved" } else { "Open" },
            if comment.resolved { t.hover } else { t.accent_soft },
            if comment.resolved { t.text_muted } else { t.accent_text },
        ),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    let actions = row![
        button(text(if comment.resolved { "Reopen" } else { "Resolve" }).font(theme::medium()).size(11.5).color(t.accent_text))
            .padding(Padding { top: 3.0, right: 8.0, bottom: 3.0, left: 8.0 })
            .on_press(Message::ToggleResolve(index))
            .style(move |_theme, status| button::Style {
                background: Some(
                    (if matches!(status, button::Status::Hovered) { t.hover } else { t.accent_soft }).into(),
                ),
                text_color: t.accent_text,
                border: Border { radius: 5.0.into(), ..Default::default() },
                ..Default::default()
            }),
        text("Reply").font(theme::medium()).size(11.5).color(t.text_muted),
    ]
    .spacing(10.0);

    container(
        column![
            header,
            text(&comment.body).font(theme::regular()).size(12.5).color(t.text),
            thread,
            actions,
        ]
        .spacing(7.0),
    )
    .width(Length::Fill)
    .padding(10)
    .style(move |_| container::Style {
        background: Some(t.card.into()),
        border: Border { color: t.border, width: 1.0, radius: 8.0.into() },
        ..Default::default()
    })
    .into()
}

/// A circular avatar with the author's initials.
fn avatar<'a>(initials: &'a str, t: Tokens) -> Element<'a, Message> {
    container(text(initials).font(theme::semibold()).size(11.0).color(t.accent_text))
        .width(28.0)
        .height(28.0)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_| container::Style {
            background: Some(t.accent_soft.into()),
            border: Border { radius: 14.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

// ---------------------------------------------------------------------------------------------
// Bookmarks
// ---------------------------------------------------------------------------------------------

fn bookmarks<'a>(items: &'a [OutlineItem], t: Tokens) -> Element<'a, Message> {
    if items.is_empty() {
        return empty(t, "bookmark", "This document has no bookmarks.");
    }
    let mut list = column![].spacing(1.0);
    for item in items {
        list = list.push(outline_item(item, 0, t));
    }
    list.width(Length::Fill).into()
}

/// One outline entry, recursing into its children with an extra indent level.
fn outline_item<'a>(item: &'a OutlineItem, depth: usize, t: Tokens) -> Element<'a, Message> {
    let mut block = column![].spacing(1.0);
    block = block.push(
        button(
            row![
                widgets::hspace(depth as f32 * 14.0),
                icons::icon::<Message>("file-text", 14.0, t.text_faint),
                text(item.title.as_str()).font(theme::regular()).size(12.5).color(t.text).width(Length::Fill),
                text(item.page.to_string()).font(theme::regular()).size(11.0).color(t.text_faint),
            ]
            .spacing(7.0)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding(Padding { top: 5.0, right: 6.0, bottom: 5.0, left: 6.0 })
        .on_press(Message::GoToPage(item.page.saturating_sub(1)))
        .style(move |_theme, status| button::Style {
            background: Some((if matches!(status, button::Status::Hovered) { t.hover } else { Color::TRANSPARENT }).into()),
            text_color: t.text,
            border: Border { radius: 5.0.into(), ..Default::default() },
            ..Default::default()
        }),
    );
    for child in &item.children {
        block = block.push(outline_item(child, depth + 1, t));
    }
    block.into()
}

// ---------------------------------------------------------------------------------------------
// Page thumbnails
// ---------------------------------------------------------------------------------------------

fn pages(app: &App, t: Tokens) -> Element<'_, Message> {
    let Some(doc) = app.doc() else {
        return empty(t, "files", "No pages to show.");
    };
    let mut list = column![].spacing(10.0);
    for index in 0..doc.pages {
        let active = app.page == index;
        list = list.push(
            button(
                column![
                    crate::ui::canvas::page_thumbnail(index, 108.0, 140.0, active, t),
                    text(format!("Page {}", index + 1)).font(theme::regular()).size(11.0).color(if active { t.accent_text } else { t.text_muted }),
                ]
                .spacing(5.0)
                .align_x(Alignment::Center),
            )
            .padding(6)
            .on_press(Message::GoToPage(index))
            .style(move |_theme, status| button::Style {
                background: Some((if active { t.selected } else if matches!(status, button::Status::Hovered) { t.hover } else { Color::TRANSPARENT }).into()),
                text_color: t.text,
                border: Border { radius: 8.0.into(), ..Default::default() },
                ..Default::default()
            }),
        );
    }
    list.width(Length::Fill).into()
}

// ---------------------------------------------------------------------------------------------
// Form fields
// ---------------------------------------------------------------------------------------------

fn fields<'a>(items: &'a [Field], t: Tokens) -> Element<'a, Message> {
    if items.is_empty() {
        return empty(t, "text-cursor-input", "This document has no form fields.");
    }
    let mut list = column![].spacing(2.0);
    for field in items {
        list = list.push(
            container(
                row![
                    icons::icon::<Message>("text-cursor-input", 15.0, t.icon),
                    column![
                        text(field.name).font(theme::medium()).size(12.5).color(t.text),
                        text(format!("{} · page {}", field.kind, field.page)).font(theme::regular()).size(11.0).color(t.text_faint),
                    ]
                    .spacing(1.0)
                    .width(Length::Fill),
                    widgets::badge(field.value.as_str(), t.hover, t.text_muted),
                ]
                .spacing(8.0)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding(8)
            .style(move |_| container::Style {
                background: Some(t.card.into()),
                border: Border { color: t.border, width: 1.0, radius: 7.0.into() },
                ..Default::default()
            }),
        );
    }
    list.width(Length::Fill).into()
}

// ---------------------------------------------------------------------------------------------
// Layers
// ---------------------------------------------------------------------------------------------

fn layers<'a>(items: &'a [Layer], t: Tokens) -> Element<'a, Message> {
    if items.is_empty() {
        return empty(t, "layers", "This document has no optional content layers.");
    }
    let mut list = column![].spacing(2.0);
    for (index, layer) in items.iter().enumerate() {
        list = list.push(
            row![
                widgets::icon_button(if layer.visible { "eye" } else { "eye-off" }, 26.0, false, true, t, Message::ToggleLayer(index)),
                text(layer.name).font(theme::regular()).size(12.5).color(if layer.visible { t.text } else { t.text_faint }).width(Length::Fill),
                if layer.locked { icons::icon::<Message>("lock", 14.0, t.text_faint) } else { icons::icon::<Message>("lock-open", 14.0, t.text_faint) },
            ]
            .spacing(6.0)
            .align_y(Alignment::Center),
        );
    }
    list.width(Length::Fill).into()
}

// ---------------------------------------------------------------------------------------------
// Attachments
// ---------------------------------------------------------------------------------------------

fn attachments(app: &App, t: Tokens) -> Element<'_, Message> {
    let Some(doc) = app.doc() else {
        return empty(t, "paperclip", "No attachments.");
    };
    if doc.attachments.is_empty() {
        return empty(t, "paperclip", "This document has no attachments.");
    }
    let mut list = column![].spacing(2.0);
    for attachment in &doc.attachments {
        list = list.push(
            container(
                row![
                    icons::icon::<Message>("paperclip", 15.0, t.icon),
                    column![
                        text(attachment.name).font(theme::medium()).size(12.5).color(t.text),
                        text(attachment.size).font(theme::regular()).size(11.0).color(t.text_faint),
                    ]
                    .spacing(1.0)
                    .width(Length::Fill),
                    widgets::icon_button("file-down", 26.0, false, true, t, Message::Notify(format!("Saved {}", attachment.name))),
                ]
                .spacing(8.0)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding(8)
            .style(move |_| container::Style {
                background: Some(t.card.into()),
                border: Border { color: t.border, width: 1.0, radius: 7.0.into() },
                ..Default::default()
            }),
        );
    }
    list.width(Length::Fill).into()
}
