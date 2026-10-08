//! Window chrome: the tab strip and the mode bar, with its menus.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::app::{App, LeftPanel, Message, Mode};
use crate::catalog;
use crate::icons;
use crate::theme::{self, Tokens};
use crate::widgets;

/// Height of the tab strip, in points (PrintCraft uses 38).
pub const TAB_STRIP_HEIGHT: f32 = 38.0;
/// Height of the mode bar, in points (PrintCraft uses 48).
pub const MODE_BAR_HEIGHT: f32 = 48.0;

/// The tab strip: the Home button, one tab per open document, the Open action, and the
/// window-level actions (theme, shortcuts, community).
pub fn tab_strip(app: &App, t: Tokens) -> Element<'_, Message> {
    let mut strip = row![].spacing(4.0).align_y(Alignment::Center);

    strip = strip.push(widgets::tip(
        widgets::icon_button("house", 28.0, app.active.is_none(), true, t, Message::SelectTab(None)),
        "Home",
        t,
    ));

    for (index, doc) in app.docs.iter().enumerate() {
        strip = strip.push(tab(app, t, index, doc));
    }

    strip = strip.push(widgets::tip(
        widgets::ghost_button("plus", "Open", t, Message::OpenRequested),
        "Open a PDF (⌘O)",
        t,
    ));

    let theme_icon = match app.theme {
        crate::theme::ThemeKind::Light => "moon",
        crate::theme::ThemeKind::Dark => "sun",
    };
    let theme_tip = match app.theme {
        crate::theme::ThemeKind::Light => "Dark gray theme",
        crate::theme::ThemeKind::Dark => "Light theme",
    };

    let right = row![
        widgets::tip(widgets::icon_button(theme_icon, 28.0, false, true, t, Message::ToggleTheme), theme_tip, t),
        widgets::tip(
            widgets::icon_button("circle-help", 28.0, false, true, t, Message::OpenDialog(crate::app::Dialog::Shortcuts)),
            "Keyboard shortcuts",
            t,
        ),
        widgets::tip(
            widgets::ghost_button("messages-square", "Discord", t, Message::Notify("Opening discord.gg/artcraft".into())),
            "Join the ArtCraft community",
            t,
        ),
    ]
    .spacing(4.0)
    .align_y(Alignment::Center);

    container(row![strip, iced::widget::space::horizontal(), right])
        .height(Length::Fixed(TAB_STRIP_HEIGHT))
        .width(Length::Fill)
        .padding(Padding { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .align_y(Alignment::Center)
        .style(move |_| container::Style { background: Some(t.titlebar.into()), ..Default::default() })
        .into()
}

/// One document tab: the file icon, its name, and either a dirty dot or a close button.
fn tab<'a>(app: &App, t: Tokens, index: usize, doc: &crate::app::Doc) -> Element<'a, Message> {
    let active = app.active == Some(index);
    let fg = if active { t.text } else { t.text_muted };
    let name: String = if doc.display_name().chars().count() > 28 {
        format!("{}…", doc.display_name().chars().take(27).collect::<String>())
    } else {
        doc.display_name().to_string()
    };

    let select = button(
        row![
            icons::icon::<Message>("file-text", 14.0, if active { t.accent } else { t.text_muted }),
            text(name).font(theme::regular()).size(13.0).line_height(1.2).color(fg),
        ]
        .spacing(7.0)
        .align_y(Alignment::Center),
    )
    .padding(Padding { top: 0.0, right: 2.0, bottom: 0.0, left: 8.0 })
    .height(Length::Fixed(30.0))
    .on_press(Message::SelectTab(Some(index)))
    .style(move |_theme, status| button::Style {
        background: Some(
            (match status {
                button::Status::Hovered if !active => t.hover,
                _ => Color::TRANSPARENT,
            })
            .into(),
        ),
        text_color: fg,
        border: Border { radius: 0.0.into(), ..Default::default() },
        ..Default::default()
    });

    // A dirty tab shows a dot where the close button sits; a clean tab shows the close button.
    let trailing: Element<'a, Message> = if doc.dirty {
        container(container(iced::widget::space()).width(8.0).height(8.0).style(move |_| container::Style {
            background: Some(if active { t.text } else { t.text_muted }.into()),
            border: Border { radius: 4.0.into(), ..Default::default() },
            ..Default::default()
        }))
        .width(24.0)
        .height(30.0)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
    } else {
        widgets::icon_button("x", 22.0, false, true, t, Message::CloseTab(index))
    };

    container(row![select, trailing].align_y(Alignment::Center))
        .height(Length::Fixed(30.0))
        .align_y(Alignment::Center)
        .style(move |_| container::Style {
            background: Some(if active { t.chrome } else { Color::TRANSPARENT }.into()),
            border: Border { radius: 7.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

/// The mode bar: the Menu button, the five workspace modes, and the document/search actions.
pub fn mode_bar(app: &App, t: Tokens) -> Element<'_, Message> {
    let mut modes = row![].spacing(2.0).align_y(Alignment::Center);
    for mode in Mode::ALL {
        modes = modes.push(widgets::mode_tab(mode.label(), app.mode == mode, t, Message::SetMode(mode)));
    }

    let has_doc = app.active.is_some();

    let actions = row![
        widgets::tip(
            widgets::icon_button("printer", 32.0, false, has_doc, t, Message::Notify("Print (⌘P)".into())),
            "Print (⌘P)",
            t,
        ),
        widgets::tip(
            widgets::icon_button("save", 32.0, false, has_doc, t, Message::Notify("Saved incrementally".into())),
            "Save (⌘S)",
            t,
        ),
        widgets::tip(
            widgets::icon_button(
                "info",
                32.0,
                false,
                has_doc,
                t,
                Message::OpenDialog(crate::app::Dialog::Properties(crate::app::PropsTab::Description)),
            ),
            "Document properties (⌘D)",
            t,
        ),
        widgets::hspace(8.0),
        widgets::search_box("Find tools and commands", 280.0, t, Message::PaletteOpen),
    ]
    .spacing(4.0)
    .align_y(Alignment::Center);

    let menu_message = if app.menu.is_some() { Message::CloseMenu } else { Message::ToggleMenu("File") };

    let body = row![
        widgets::ghost_button("panel-left", "Menu", t, menu_message),
        widgets::v_divider(t),
        modes,
        iced::widget::space::horizontal(),
        actions,
    ]
    .spacing(6.0)
    .align_y(Alignment::Center);

    container(body)
        .height(Length::Fixed(MODE_BAR_HEIGHT))
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .padding(Padding { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .style(move |_| container::Style { background: Some(t.chrome.into()), ..Default::default() })
        .into()
}

/// The dropdown opened by the Menu button: menu-group tabs over the group's commands.
pub fn menu_dropdown(app: &App, t: Tokens) -> Element<'_, Message> {
    let current = app.menu.filter(|m| !m.is_empty()).unwrap_or("File");

    let mut tabs = row![].spacing(2.0);
    for menu in catalog::MENUS {
        tabs = tabs.push(widgets::mode_tab_small(menu, current == menu, t, Message::ToggleMenu(menu)));
    }

    let mut items = column![].spacing(1.0).width(Length::Fill);
    let mut any = false;
    for command in catalog::COMMANDS.iter().filter(|c| c.menu == current) {
        any = true;
        items = items.push(widgets::menu_item(
            command.label,
            command.shortcut.unwrap_or(""),
            true,
            t,
            Message::PalettePick(command.id.to_string()),
        ));
    }
    if current == "Tools" {
        for group in catalog::TOOL_GROUPS {
            any = true;
            items = items.push(widgets::menu_item(
                group.label,
                "",
                group.availability.is_ready(),
                t,
                Message::OpenLeft(LeftPanel::Tool(group.id)),
            ));
        }
    }
    if !any {
        items = items.push(text("No commands in this menu").size(12.0).color(t.text_faint));
    }

    container(
        column![
            tabs,
            widgets::h_divider(t),
            widgets::vspace(4.0),
            widgets::scrolled(items, t).height(Length::Fixed(320.0)),
        ]
        .spacing(6.0),
    )
    .width(Length::Fixed(280.0))
    .padding(Padding { top: 8.0, right: 8.0, bottom: 8.0, left: 8.0 })
    .style(move |_| container::Style {
        background: Some(t.card.into()),
        border: Border { color: t.border, width: 1.0, radius: 10.0.into() },
        shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.18), offset: iced::Vector::new(0.0, 6.0), blur_radius: 20.0 },
        ..Default::default()
    })
    .into()
}
