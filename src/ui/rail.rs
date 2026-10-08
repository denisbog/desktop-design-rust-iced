//! The right rail: panel toggles down the side, and the page/zoom cluster at the bottom.
//!
//! This is the counterpart of PrintCraft's `chrome::right_rail`.

use iced::widget::{column, container, row, stack, text, text_input};
use iced::{Alignment, Border, Element, Length, Padding};

use crate::app::{App, Fit, Message, RightPanel};
use crate::theme::{self, Tokens};
use crate::widgets;

/// Width of the rail, in points (PrintCraft uses 48).
pub const WIDTH: f32 = 48.0;

pub fn view(app: &App, t: Tokens) -> Element<'_, Message> {
    let Some(doc) = app.doc() else {
        return rail_shell(column![].into(), t);
    };

    let has = |count: usize| count > 0;

    let mut panels = column![].spacing(4.0).align_x(Alignment::Center);
    for (panel, present) in [
        (RightPanel::Comments, has(doc.comments.len())),
        (RightPanel::Bookmarks, has(doc.outline.len())),
        (RightPanel::Pages, false),
        (RightPanel::Fields, has(doc.fields.len())),
        (RightPanel::Layers, has(doc.layers.len())),
        (RightPanel::Attachments, has(doc.attachments.len())),
        (RightPanel::Signatures, doc.signed),
    ] {
        panels = panels.push(rail_button(app, panel, present, t));
    }

    // The page cluster, in the order PrintCraft lays it out bottom-up.
    let pages = doc.pages;
    let zoom_pct = format!("{:.0}%", app.zoom * 100.0);
    let fit_icon = if app.fit == Fit::Width { "maximize" } else { "columns-2" };
    let fit_tip = if app.fit == Fit::Width { "Fit to page" } else { "Fit to width" };

    let cluster = column![
        container(
            text_input("", &app.page_input)
                .on_input(Message::PageInput)
                .on_submit(Message::PageSubmit)
                .font(theme::medium())
                .size(12.0)
                .align_x(Alignment::Center)
                .padding(4)
                .width(Length::Fixed(38.0))
                .style(move |_theme, status| text_input::Style {
                    background: t.field.into(),
                    border: Border {
                        color: if matches!(status, text_input::Status::Focused { .. }) { t.accent } else { t.border },
                        width: 1.0,
                        radius: 5.0.into()
                    },
                    icon: t.text_muted,
                    placeholder: t.text_faint,
                    value: t.text,
                    selection: t.accent_soft,
                }),
        )
        .padding(Padding { top: 6.0, right: 0.0, bottom: 2.0, left: 0.0 }),
        text(pages.to_string()).font(theme::regular()).size(11.0).color(t.text_muted),
        widgets::tip(
            widgets::icon_button("chevron-up", 30.0, false, true, t, Message::PrevPage),
            "Previous page",
            t,
        ),
        widgets::tip(
            widgets::icon_button("chevron-down", 30.0, false, true, t, Message::NextPage),
            "Next page",
            t,
        ),
        widgets::vspace(4.0),
        text(zoom_pct).font(theme::mono()).size(10.5).color(t.text_faint),
        widgets::tip(
            widgets::icon_button(fit_icon, 32.0, false, true, t, Message::SetFit(if app.fit == Fit::Width { Fit::Page } else { Fit::Width })),
            fit_tip,
            t,
        ),
        widgets::tip(
            widgets::icon_button("rotate-cw", 32.0, false, true, t, Message::Notify("Rotated the view clockwise".into())),
            "Rotate view clockwise",
            t,
        ),
        widgets::tip(widgets::icon_button("zoom-in", 32.0, false, true, t, Message::ZoomIn), "Zoom in (⌘+)", t),
        widgets::tip(widgets::icon_button("zoom-out", 32.0, false, true, t, Message::ZoomOut), "Zoom out (⌘−)", t),
    ]
    .spacing(2.0)
    .align_x(Alignment::Center);

    let body = column![panels, iced::widget::space::vertical(), cluster].align_x(Alignment::Center);
    rail_shell(body.into(), t)
}

/// The rail shell: a fixed-width, full-height chrome column with a divider on its left edge.
fn rail_shell<'a>(body: Element<'a, Message>, t: Tokens) -> Element<'a, Message> {
    container(
        row![
            widgets::v_divider(t),
            container(body)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .padding(Padding { top: 8.0, right: 0.0, bottom: 8.0, left: 0.0 }),
        ]
        .height(Length::Fill)
        .align_y(Alignment::Center),
    )
    .width(Length::Fixed(WIDTH))
    .height(Length::Fill)
    .style(move |_| container::Style { background: Some(t.chrome.into()), ..Default::default() })
    .into()
}

/// A panel toggle: selected gets the accent treatment, and a small dot marks content present.
fn rail_button<'a>(app: &App, panel: RightPanel, present: bool, t: Tokens) -> Element<'a, Message> {
    let selected = app.right == Some(panel);
    let button = widgets::icon_button(panel.icon(), 34.0, selected, true, t, Message::ToggleRight(panel));

    if !present || selected {
        return widgets::tip(button, panel.title(), t);
    }

    let dot = container(container(iced::widget::space()).width(6.0).height(6.0).style(move |_| container::Style {
        background: Some(t.accent.into()),
        border: Border { radius: 3.0.into(), ..Default::default() },
        ..Default::default()
    }))
    .width(Length::Fixed(34.0))
    .height(Length::Fixed(34.0))
    .align_x(Alignment::End)
    .align_y(Alignment::Start)
    .padding(Padding { top: 5.0, right: 4.0, bottom: 0.0, left: 0.0 });

    widgets::tip(stack![button, dot].into(), panel.title(), t)
}

