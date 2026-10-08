//! Root composition: the chrome column, the three-column workspace, and the floating layers.

use iced::widget::{button, column, container, row, stack, Stack};
use iced::{Alignment, Color, Element, Length, Padding};

use crate::app::{App, Message};
use crate::theme::Tokens;
use crate::ui::{canvas, chrome, left, overlays, rail, right};
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let t = app.tokens();

    // The workspace: left panel | document area | right panel | rail.
    let mut workspace = row![].width(Length::Fill).height(Length::Fill);
    if app.left_open {
        workspace = workspace.push(left::view(app, t));
    }
    workspace = workspace.push(canvas::view(app, t));
    if app.right.is_some() {
        workspace = workspace.push(right::view(app, t));
    }
    workspace = workspace.push(rail::view(app, t));

    let root = column![
        chrome::tab_strip(app, t),
        chrome::mode_bar(app, t),
        widgets::h_divider(t),
        workspace,
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    let mut layers: Stack<'_, Message> = Stack::new().push(root);

    if app.menu.is_some() {
        layers = layers.push(menu_layer(app, t));
    }
    if app.palette_open {
        layers = layers.push(overlays::palette(app, t));
    }
    if app.dialog.is_some() {
        layers = layers.push(overlays::dialog(app, t));
    }
    if app.toast.is_some() {
        layers = layers.push(overlays::toast(app, t));
    }

    layers.into()
}

/// The Menu dropdown layer: a click-away scrim under the dropdown panel, anchored below the chrome.
fn menu_layer(app: &App, t: Tokens) -> Element<'_, Message> {
    let scrim = button(iced::widget::space())
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::CloseMenu)
        .style(|_theme, _status| button::Style { background: Some(Color::TRANSPARENT.into()), ..Default::default() });

    let anchored = container(chrome::menu_dropdown(app, t))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding {
            top: chrome::TAB_STRIP_HEIGHT + chrome::MODE_BAR_HEIGHT + 1.0,
            right: 0.0,
            bottom: 0.0,
            left: 10.0,
        })
        .align_x(Alignment::Start)
        .align_y(Alignment::Start);

    stack![scrim, anchored].into()
}

