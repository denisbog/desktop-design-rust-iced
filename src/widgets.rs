//! The demo's custom widgets, built from the design tokens — the iced counterpart of PrintCraft's
//! `crates/ui-egui/src/widgets.rs`.
//!
//! Each constructor is a plain function returning an [`Element`], so callers compose them exactly
//! like the built-in widgets. Hover and press feedback comes from iced's `button` status, which
//! keeps every widget keyboard- and accessibility-aware for free.

use iced::widget::{button, column, container, row, scrollable, space, text, tooltip, Scrollable};
use iced::{Alignment, Border, Color, Element, Length, Padding, Shadow};

use crate::icons;
use crate::theme::{self, Tokens};

/// A mode-bar tab: text with an underline when active.
pub fn mode_tab<'a, Message: Clone + 'a>(
    label: &'a str,
    active: bool,
    t: Tokens,
    on_press: Message,
) -> Element<'a, Message> {
    let font = if active { theme::semibold() } else { theme::medium() };
    let fg = if active { t.text } else { t.text_muted };
    let underline = if active { t.text } else { Color::TRANSPARENT };

    let content = column![
        space::vertical().height(Length::Fixed(3.0)),
        space::vertical(),
        text(label).font(font).size(13.5).line_height(1.2).color(fg),
        space::vertical(),
        container(space())
            .height(Length::Fixed(3.0))
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(underline.into()),
                ..Default::default()
            }),
    ]
    .width(Length::Shrink)
    .align_x(Alignment::Center);

    button(content)
        .padding(Padding { top: 0.0, right: 11.0, bottom: 0.0, left: 11.0 })
        .height(Length::Fixed(48.0))
        .on_press(on_press)
        .style(move |_theme, status| {
            let bg = match status {
                button::Status::Hovered if !active => t.hover,
                _ => Color::TRANSPARENT,
            };
            button::Style { background: Some(bg.into()), text_color: fg, ..Default::default() }
        })
        .into()
}

/// A compact tab for the menu dropdown's group row (File / Edit / View / Help / Tools).
pub fn mode_tab_small<'a, Message: Clone + 'a>(
    label: &'a str,
    active: bool,
    t: Tokens,
    on_press: Message,
) -> Element<'a, Message> {
    let font = if active { theme::semibold() } else { theme::medium() };
    let fg = if active { t.text } else { t.text_muted };
    let fill = if active { t.accent_soft } else { Color::TRANSPARENT };
    button(text(label).font(font).size(12.0).color(fg))
        .padding(Padding { top: 4.0, right: 9.0, bottom: 4.0, left: 9.0 })
        .on_press(on_press)
        .style(move |_theme, status| button::Style {
            background: Some(
                (match status {
                    button::Status::Hovered if !active => t.hover,
                    _ => fill,
                })
                .into(),
            ),
            text_color: fg,
            border: Border { radius: 5.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

/// A rounded pill button; `primary` fills with the accent.
pub fn pill_button<'a, Message: Clone + 'a>(
    label: &'a str,
    primary: bool,
    t: Tokens,
    on_press: Message,
) -> Element<'a, Message> {
    button(text(label).font(theme::medium()).size(12.5))
        .padding(Padding { top: 5.0, right: 13.0, bottom: 5.0, left: 13.0 })
        .on_press(on_press)
        .style(move |_theme, status| {
            let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
            let (fill, border, fg) = if primary {
                (if hovered { t.accent_text } else { t.accent }, Border::default(), Color::WHITE)
            } else {
                (
                    if hovered { t.hover } else { t.card },
                    Border { color: t.text_muted, width: 1.2, radius: 14.0.into() },
                    t.text,
                )
            };
            button::Style { background: Some(fill.into()), text_color: fg, border, ..Default::default() }
        })
        .into()
}

/// Icon + label; transparent until hovered.
pub fn ghost_button<'a, Message: Clone + 'a>(
    icon: &str,
    label: &'a str,
    t: Tokens,
    on_press: Message,
) -> Element<'a, Message> {
    let content = row![
        icons::icon::<Message>(icon, 15.0, t.icon),
        text(label).font(theme::medium()).size(13.0).line_height(1.2).color(t.text),
    ]
    .spacing(6.0)
    .align_y(Alignment::Center);
    button(content)
        .padding(Padding { top: 5.0, right: 10.0, bottom: 5.0, left: 6.0 })
        .height(Length::Fixed(30.0))
        .on_press(on_press)
        .style(move |_theme, status| button::Style {
            background: Some(
                (if matches!(status, button::Status::Hovered | button::Status::Pressed) { t.hover } else { Color::TRANSPARENT }).into(),
            ),
            text_color: t.text,
            border: Border { radius: t.radius.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

/// A square icon button: transparent until hovered; `selected` gets the accent treatment.
pub fn icon_button<'a, Message: Clone + 'a>(
    name: &str,
    box_size: f32,
    selected: bool,
    enabled: bool,
    t: Tokens,
    on_press: Message,
) -> Element<'a, Message> {
    let tint = if !enabled {
        t.text_faint
    } else if selected {
        t.accent_text
    } else {
        t.icon
    };
    let fill = if selected { t.accent_soft } else { Color::TRANSPARENT };

    // iced's `Button` pins its content limits to its own size (`min = max`) and does not loosen
    // them, so a `Length::Fixed` glyph inside a `Length::Fixed` button gets clamped *up* to the
    // button size. Deriving the box from the padding instead keeps the glyph at `glyph_size`.
    let glyph_size = (box_size * 0.42).max(11.0).round();
    let inset = ((box_size - glyph_size) / 2.0).max(0.0);

    button(icons::icon::<Message>(name, glyph_size, tint))
        .padding(inset)
        .on_press_maybe(enabled.then_some(on_press))
        .style(move |_theme, status| button::Style {
            background: Some(
                match status {
                    button::Status::Hovered | button::Status::Pressed if enabled => t.hover,
                    _ => fill,
                }
                .into(),
            ),
            text_color: tint,
            border: Border { radius: t.radius.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

/// A search-field lookalike (click it to open the command palette) with a `⌘K` hint.
pub fn search_box<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    width: f32,
    t: Tokens,
    on_press: Message,
) -> Element<'a, Message> {
    let content = row![
        icons::icon::<Message>("search", 14.0, t.text_muted),
        text(placeholder).font(theme::regular()).size(13.0).line_height(1.2).color(t.text_faint),
        space::horizontal(),
        text("⌘K").font(theme::regular()).size(11.5).color(t.text_faint),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);
    button(content)
        .padding(Padding { top: 0.0, right: 12.0, bottom: 0.0, left: 10.0 })
        .width(Length::Fixed(width))
        .height(Length::Fixed(32.0))
        .on_press(on_press)
        .style(move |_theme, status| {
            let fill = if matches!(status, button::Status::Hovered | button::Status::Pressed) { t.hover } else { t.field };
            button::Style {
                background: Some(fill.into()),
                text_color: t.text,
                border: Border { color: t.border, width: 1.0, radius: 16.0.into() },
                ..Default::default()
            }
        })
        .into()
}

/// A menu row: label on the left, shortcut on the right, disabled when `enabled` is false.
pub fn menu_item<'a, Message: Clone + 'a>(
    label: &'a str,
    shortcut: &'a str,
    enabled: bool,
    t: Tokens,
    on_press: Message,
) -> Element<'a, Message> {
    let fg = if enabled { t.text } else { t.text_faint };
    let content = row![
        text(label).font(theme::regular()).size(13.0).line_height(1.2).color(fg).width(Length::Fill),
        text(shortcut).font(theme::regular()).size(11.5).line_height(1.2).color(t.text_faint),
    ]
    .spacing(24.0)
    .align_y(Alignment::Center);
    button(content)
        .width(Length::Fill)
        .padding(Padding { top: 6.0, right: 10.0, bottom: 6.0, left: 10.0 })
        .on_press_maybe(enabled.then_some(on_press))
        .style(move |_theme, status| button::Style {
            background: Some(
                (if matches!(status, button::Status::Hovered | button::Status::Pressed) && enabled { t.hover } else { Color::TRANSPARENT })
                    .into(),
            ),
            text_color: fg,
            border: Border { radius: 4.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

/// An all-caps, letter-spaced section heading for panel bodies.
pub fn section_title<'a, Message: 'a>(title: &'a str, t: Tokens) -> Element<'a, Message> {
    text(title.to_uppercase())
        .font(theme::semibold())
        .size(10.5)
        .color(t.text_faint)
        .into()
}

/// A pill button with a leading icon (primary = filled accent).
pub fn icon_pill<'a, Message: Clone + 'a>(
    icon: &str,
    label: &'a str,
    primary: bool,
    t: Tokens,
    on_press: Message,
) -> Element<'a, Message> {
    let fg = if primary { Color::WHITE } else { t.text };
    let content = row![
        icons::icon::<Message>(icon, 14.0, fg),
        text(label).font(theme::medium()).size(12.5).line_height(1.2).color(fg),
    ]
    .spacing(6.0)
    .align_y(Alignment::Center);
    button(content)
        .padding(Padding { top: 5.0, right: 13.0, bottom: 5.0, left: 11.0 })
        .on_press(on_press)
        .style(move |_theme, status| {
            let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
            let (fill, border) = if primary {
                (if hovered { t.accent_text } else { t.accent }, Border::default())
            } else {
                (
                    if hovered { t.hover } else { t.card },
                    Border { color: t.text_muted, width: 1.2, radius: 15.0.into() },
                )
            };
            button::Style { background: Some(fill.into()), text_color: fg, border, ..Default::default() }
        })
        .into()
}

/// A small status pill ("M7", "Ready") used by catalogue rows.
pub fn badge<'a, Message: 'a>(label: &'a str, fill: Color, fg: Color) -> Element<'a, Message> {
    container(text(label).font(theme::medium()).size(10.0).line_height(1.2).color(fg))
        .padding(Padding { top: 2.0, right: 6.0, bottom: 2.0, left: 6.0 })
        .style(move |_| container::Style {
            background: Some(fill.into()),
            border: Border { radius: 8.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

/// A 1-point vertical divider that fills the available height.
pub fn v_divider<'a, Message: 'a>(t: Tokens) -> Element<'a, Message> {
    container(space())
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .style(move |_| container::Style { background: Some(t.divider.into()), ..Default::default() })
        .into()
}

/// A 1-point horizontal divider that fills the available width.
pub fn h_divider<'a, Message: 'a>(t: Tokens) -> Element<'a, Message> {
    container(space())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(move |_| container::Style { background: Some(t.divider.into()), ..Default::default() })
        .into()
}

/// A square spacer.
pub fn hspace<'a, Message: 'a>(px: f32) -> Element<'a, Message> {
    space::horizontal().width(Length::Fixed(px)).into()
}

/// A vertical spacer.
pub fn vspace<'a, Message: 'a>(px: f32) -> Element<'a, Message> {
    space::vertical().height(Length::Fixed(px)).into()
}

/// Wrap `content` in a delayed tooltip styled like the rest of the chrome.
pub fn tip<'a, Message: 'a>(content: Element<'a, Message>, label: &'a str, t: Tokens) -> Element<'a, Message> {
    let body = container(text(label).font(theme::regular()).size(12.0).line_height(1.2).color(if t.dark() { t.text } else { Color::WHITE }))
        .padding(Padding { top: 5.0, right: 9.0, bottom: 5.0, left: 9.0 })
        .style(move |_| container::Style {
            background: Some(if t.dark() { t.card } else { Color::from_rgb8(0x2A, 0x2A, 0x2F) }.into()),
            border: Border { radius: 5.0.into(), ..Default::default() },
            ..Default::default()
        });
    tooltip(content, body, tooltip::Position::Bottom).gap(6.0).into()
}

/// A card surface used by the home screen and the panels.
pub fn card_style(t: Tokens) -> container::Style {
    container::Style {
        background: Some(t.card.into()),
        border: Border { color: t.border, width: 1.0, radius: 12.0.into() },
        ..Default::default()
    }
}


/// A thin, floating scrollbar: a 7 px lane with a 5 px rounded thumb, no rail background.
///
/// Every scrollable in the app goes through this helper, so the workspace, the panels and the
/// overlays share one scrollbar look instead of iced's 10 px default.
pub fn scrolled<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    t: Tokens,
) -> Scrollable<'a, Message> {
    scrollable(content)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new().width(7.0).scroller_width(5.0).margin(1.0),
        ))
        .style(move |_theme, status| {
            let (hovered, dragged) = match status {
                scrollable::Status::Hovered { is_vertical_scrollbar_hovered, .. } => (is_vertical_scrollbar_hovered, false),
                scrollable::Status::Dragged { is_vertical_scrollbar_dragged, .. } => (true, is_vertical_scrollbar_dragged),
                _ => (false, false),
            };
            let thumb = if dragged { t.icon } else if hovered { t.text_muted } else { t.text_faint };
            let scroller = scrollable::Scroller {
                background: thumb.into(),
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 3.0.into() },
            };

            scrollable::Style {
                container: container::Style::default(),
                vertical_rail: scrollable::Rail { background: None, border: Border::default(), scroller },
                horizontal_rail: scrollable::Rail { background: None, border: Border::default(), scroller },
                gap: None,
                auto_scroll: scrollable::AutoScroll {
                    background: t.card.into(),
                    border: Border { color: t.border, width: 1.0, radius: 8.0.into() },
                    shadow: Shadow::default(),
                    icon: t.icon,
                },
            }
        })
}
