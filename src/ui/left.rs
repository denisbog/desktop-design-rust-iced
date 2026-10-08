//! The left panel: "All tools" and a single tool's detail view.
//!
//! It is the primary navigation surface of the app and is driven entirely by [`catalog`].

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::app::{App, LeftPanel, Message};
use crate::catalog::{self, Availability, ToolGroup};
use crate::icons;
use crate::theme::{self, Tokens};
use crate::widgets;

/// Width of the left panel, in points (PrintCraft uses 264).
pub const WIDTH: f32 = 264.0;

pub fn view(app: &App, t: Tokens) -> Element<'_, Message> {
    let body: Element<'_, Message> = match app.left {
        LeftPanel::AllTools => all_tools(t),
        LeftPanel::Tool(id) => match ToolGroup::find(id) {
            Some(group) => tool_detail(group, t),
            None => all_tools(t),
        },
    };

    container(body)
        .width(Length::Fixed(WIDTH))
        .height(Length::Fill)
        .style(move |_| container::Style { background: Some(t.panel.into()), ..Default::default() })
        .into()
}

/// The header shared by both modes: a title, an optional back button and a close button.
fn panel_header<'a>(title: &'a str, back: Option<Message>, t: Tokens) -> Element<'a, Message> {
    let mut bar = row![].spacing(5.0).align_y(Alignment::Center);
    if let Some(message) = back {
        bar = bar.push(widgets::icon_button("chevron-left", 24.0, false, true, t, message));
    }
    bar = bar.push(text(title).font(theme::semibold()).size(13.5).line_height(1.2).color(t.text));

    container(
        row![bar, iced::widget::space::horizontal(), widgets::tip(
            widgets::icon_button("panel-left", 26.0, false, true, t, Message::ToggleLeft),
            "Hide panel",
            t,
        )]
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fixed(40.0))
    .padding(Padding { top: 0.0, right: 8.0, bottom: 0.0, left: 10.0 })
    .align_y(Alignment::Center)
    .into()
}

/// "All tools": every catalogue family as a tappable row.
fn all_tools(t: Tokens) -> Element<'static, Message> {
    let mut list = column![].spacing(2.0).width(Length::Fill);

    for group in catalog::TOOL_GROUPS {
        list = list.push(tool_row(group, t));
    }

    container(column![
        panel_header("All tools", None, t),
        widgets::h_divider(t),
        widgets::scrolled(container(list).padding(Padding { top: 8.0, right: 8.0, bottom: 16.0, left: 8.0 }), t)
            .height(Length::Fill),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// A row in "All tools": colour-coded icon, label, blurb and a chevron.
fn tool_row<'a>(group: &'static ToolGroup, t: Tokens) -> Element<'a, Message> {
    let hue = Color::from_rgb8(group.hue[0], group.hue[1], group.hue[2]);
    let ready = group.availability.is_ready();

    let labels = column![
        text(group.label).font(theme::medium()).size(13.0).line_height(1.2).color(t.text),
        text(group.blurb()).font(theme::regular()).size(11.0).line_height(1.25).color(t.text_faint).wrapping(iced::widget::text::Wrapping::Word),
    ]
    .spacing(1.0);

    let mut trailing = row![].spacing(4.0).align_y(Alignment::Center);
    if let Some(badge) = group.availability.badge() {
        trailing = trailing.push(widgets::badge(badge, if ready { t.accent_soft } else { t.hover }, t.text_muted));
    }
    trailing = trailing.push(icons::icon::<Message>("chevron-right", 15.0, t.text_faint));

    button(
        row![
            icons::icon::<Message>(group.icon, 18.0, hue),
            container(labels).width(Length::Fill).padding(Padding { top: 0.0, right: 4.0, bottom: 0.0, left: 0.0 }),
            trailing,
        ]
        .spacing(10.0)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding(Padding { top: 8.0, right: 8.0, bottom: 8.0, left: 8.0 })
    .on_press(Message::OpenLeft(LeftPanel::Tool(group.id)))
    .style(move |_theme, status| button::Style {
        background: Some(
            (match status {
                button::Status::Hovered => t.hover,
                button::Status::Pressed => t.pressed,
                _ => Color::TRANSPARENT,
            })
            .into(),
        ),
        text_color: t.text,
        border: Border { radius: 8.0.into(), ..Default::default() },
        ..Default::default()
    })
    .into()
}

/// A tool's detail view: coloured header, blurb, then its sections and items.
fn tool_detail(group: &'static ToolGroup, t: Tokens) -> Element<'static, Message> {
    let hue = Color::from_rgb8(group.hue[0], group.hue[1], group.hue[2]);

    let mut body = column![].spacing(2.0).width(Length::Fill);

    body = body.push(
        container(
            row![
                container(icons::icon::<Message>(group.icon, 22.0, hue))
                    .width(38.0)
                    .height(38.0)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(move |_| container::Style {
                        background: Some(t.hover.into()),
                        border: Border { radius: 9.0.into(), ..Default::default() },
                        ..Default::default()
                    }),
                column![
                    row![
                        text(group.label).font(theme::semibold()).size(14.0).line_height(1.2).color(t.text),
                        widgets::badge(
                            group.availability.badge().unwrap_or("Ready"),
                            if group.availability.is_ready() { t.accent_soft } else { t.hover },
                            if group.availability.is_ready() { t.accent_text } else { t.text_muted },
                        ),
                    ]
                    .spacing(6.0)
                    .align_y(Alignment::Center),
                    text(format!("{} · {} tools", group.availability.note(), group.item_count())).font(theme::regular()).size(11.0).color(t.text_faint),
                ]
                .spacing(2.0),
            ]
            .spacing(10.0)
            .align_y(Alignment::Center),
        )
        .padding(Padding { top: 10.0, right: 10.0, bottom: 10.0, left: 10.0 }),
    );

    for section in group.sections {
        body = body.push(widgets::vspace(6.0));
        body = body.push(container(widgets::section_title(section.title, t)).padding(Padding { top: 0.0, right: 10.0, bottom: 2.0, left: 10.0 }));
        for item in section.items {
            body = body.push(item_row(item.label, item.icon, item.availability, t, Message::PalettePick(item.command.to_string())));
        }
    }

    container(column![
        panel_header(group.label, Some(Message::OpenLeft(LeftPanel::AllTools)), t),
        widgets::h_divider(t),
        widgets::scrolled(container(body).padding(Padding { top: 4.0, right: 8.0, bottom: 16.0, left: 8.0 }), t).height(Length::Fill),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// One actionable row inside a tool: icon, label and an availability pill.
pub fn item_row<'a>(
    label: &'a str,
    icon: &str,
    availability: Availability,
    t: Tokens,
    on_press: Message,
) -> Element<'a, Message> {
    let ready = availability.is_ready();
    let fg = if ready { t.text } else { t.text_faint };

    let mut trailing = row![].spacing(4.0).align_y(Alignment::Center);
    if let Some(badge) = availability.badge() {
        trailing = trailing.push(widgets::badge(badge, t.hover, t.text_muted));
    }

    button(
        row![
            icons::icon::<Message>(icon, 15.0, if ready { t.icon } else { t.text_faint }),
            text(label).font(theme::regular()).size(12.5).line_height(1.2).color(fg).width(Length::Fill),
            trailing,
        ]
        .spacing(9.0)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding(Padding { top: 6.0, right: 8.0, bottom: 6.0, left: 8.0 })
    .on_press(on_press)
    .style(move |_theme, status| button::Style {
        background: Some(
            (match status {
                button::Status::Hovered => t.hover,
                button::Status::Pressed => t.pressed,
                _ => Color::TRANSPARENT,
            })
            .into(),
        ),
        text_color: fg,
        border: Border { radius: 6.0.into(), ..Default::default() },
        ..Default::default()
    })
    .into()
}
