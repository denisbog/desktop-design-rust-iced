//! The document area: the find bar, the pasteboard with the open page, and Organize pages.
//!
//! Pages stay white in both themes — the "Dark Gray keeps pages white" rule from PrintCraft's
//! design notes — so the ink colours here are deliberately independent of the tokens.

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Padding, Shadow, Vector};

use crate::app::{App, Message, PageLayout};
use crate::icons;
use crate::theme::{self, Tokens};
use crate::widgets;

/// The id of the find field, so ⌘F can focus it.
pub const FIND_INPUT: &str = "find-input";

/// Ink colours used *on the page*, fixed in both themes.
const INK: Color = Color::from_rgb(0.13, 0.13, 0.15);
const INK_MUTED: Color = Color::from_rgb(0.42, 0.42, 0.45);
const INK_FAINT: Color = Color::from_rgb(0.62, 0.62, 0.65);
const PAPER: Color = Color::WHITE;
const ACCENT: Color = Color::from_rgb8(0x1B, 0x63, 0xE0);

/// US Letter, in points; multiplied by the zoom to get on-screen pixels.
const PAGE_W: f32 = 612.0;
const PAGE_H: f32 = 792.0;

pub fn view(app: &App, t: Tokens) -> Element<'_, Message> {
    let center: Element<'_, Message> = if app.active.is_none() {
        crate::ui::home::view(app, t)
    } else if app.organizing {
        organize(app, t)
    } else {
        reader(app, t)
    };

    container(center)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style { background: Some(t.pasteboard.into()), ..Default::default() })
        .into()
}

// ---------------------------------------------------------------------------------------------
// Reader
// ---------------------------------------------------------------------------------------------

fn reader(app: &App, t: Tokens) -> Element<'_, Message> {
    let find: Element<'_, Message> = if app.find_open { find_bar(app, t) } else { container(iced::widget::space()).height(0).into() };

    let zoom = app.zoom;
    let page = page_card(app.page, zoom, t);

    let surface = match app.layout {
        PageLayout::TwoUp => {
            let right = if app.page + 1 < app.doc().map(|d| d.pages).unwrap_or(1) {
                Some(page_card(app.page + 1, zoom, t))
            } else {
                None
            };
            let mut row = row![page].spacing(24.0).align_y(Alignment::Start);
            if let Some(second) = right {
                row = row.push(second);
            }
            row
        }
        _ => row![page],
    };

    let scrolled = widgets::scrolled(
        container(surface)
            .width(Length::Fill)
            .padding(Padding { top: 28.0, right: 28.0, bottom: 40.0, left: 28.0 })
            .center_x(Length::Fill)
            .center_y(Length::Fill),
        t,
    )
    .height(Length::Fill);

    column![find, scrolled].height(Length::Fill).into()
}

/// The white page with its shadow and a sample of typographic content.
fn page_card<'a>(index: usize, zoom: f32, t: Tokens) -> Element<'a, Message> {
    let width = PAGE_W * zoom;
    let height = PAGE_H * zoom;

    container(page_body(index, zoom))
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .padding(Padding {
            top: 52.0 * zoom,
            right: 54.0 * zoom,
            bottom: 48.0 * zoom,
            left: 54.0 * zoom,
        })
        .style(move |_| container::Style {
            background: Some(PAPER.into()),
            shadow: Shadow { color: t.page_shadow, offset: Vector::new(0.0, 4.0), blur_radius: 16.0 },
            ..Default::default()
        })
        .into()
}

/// A sample of the "PrintCraft Showcase": headings, body text, a pull quote and a figure.
fn page_body<'a>(index: usize, zoom: f32) -> Element<'a, Message> {
    let s = |v: f32| v * zoom;
    let samples: [(&str, &str, &str, &str); 5] = [
        (
            "PrintCraft Showcase",
            "A 13-page specimen",
            "This document exercises real-world typography: world scripts, vertical Japanese, colour emoji, gradients, soft masks and transparency. Everything renders the way the author intended.",
            "Every save appends your changes and leaves the original bytes untouched.",
        ),
        (
            "Foreword",
            "Why a PDF workbench",
            "Read, organize, combine, split and secure PDFs in a fast, native app written in Rust from the ground up. No account, no telemetry, no cloud.",
            "The engine, the CLI and the app are all open source.",
        ),
        (
            "Setting Text",
            "Kerning and ligatures",
            "Text is set with care for the details that make a page feel right: kerning and ligatures, right-to-left and complex scripts, vertical CJK and colour emoji.",
            "Deep zoom stays sharp: large pages render in tiles.",
        ),
        (
            "Code and Images",
            "Syntax listings",
            "A syntax-coloured listing sits beside a fractal image. Blend modes, shadings and soft masks are all rendered from the document's own content streams.",
            "Built to survive bad files: every page renders in isolation.",
        ),
        (
            "Scripts of the World",
            "Twelve writing systems",
            "Arabic, Hebrew, Devanagari, Thai, Greek, Cyrillic, Chinese, Korean, IPA, Armenian, Georgian and Tamil on one page.",
            "Selection comes out in the right reading order, for columns and CJK too.",
        ),
    ];
    let (title, subtitle, body, quote) = samples[index % samples.len()];

    let mut content = column![].spacing(s(10.0)).width(Length::Fill);

    content = content.push(
        row![
            text("PRINTCRAFT · SHOWCASE").font(theme::semibold()).size(s(8.5)).color(ACCENT),
            iced::widget::space::horizontal(),
            text(format!("Page {}", index + 1)).font(theme::regular()).size(s(8.5)).color(INK_FAINT),
        ]
        .align_y(Alignment::Center),
    );

    content = content.push(container(iced::widget::space()).height(Length::Fixed(1.0)).width(Length::Fill).style(move |_| container::Style {
        background: Some(INK_FAINT.into()),
        ..Default::default()
    }));

    content = content.push(text(title.to_string()).font(theme::semibold()).size(s(34.0)).color(INK));
    content = content.push(text(subtitle.to_string()).font(theme::regular()).size(s(13.0)).color(INK_MUTED));
    content = content.push(widgets::vspace(s(10.0)));
    content = content.push(text(body.to_string()).font(theme::regular()).size(s(11.5)).color(INK));

    // Pull quote with an accent rule.
    content = content.push(widgets::vspace(s(8.0)));
    content = content.push(
        row![
            container(iced::widget::space()).width(Length::Fixed(s(3.0))).height(Length::Fixed(s(38.0))).style(move |_| container::Style {
                background: Some(ACCENT.into()),
                ..Default::default()
            }),
            text(quote.to_string()).font(theme::medium()).size(s(12.5)).color(INK_MUTED).width(Length::Fill),
        ]
        .spacing(s(12.0))
        .align_y(Alignment::Center),
    );

    // A figure block, coloured like an image placeholder.
    content = content.push(widgets::vspace(s(8.0)));
    content = content.push(
        container(
            row![
                icons::icon::<Message>("image", s(20.0), Color::WHITE),
                text("Fractal, 2048 × 2048, device RGB").font(theme::regular()).size(s(10.0)).color(Color::WHITE),
            ]
            .spacing(s(8.0))
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fixed(PAGE_H * zoom * 0.22))
        .padding(Padding { top: 0.0, right: 0.0, bottom: 0.0, left: s(14.0) })
        .align_y(Alignment::Center)
        .style(move |_| container::Style {
            background: Some(Color::from_rgb8(0x1E, 0x3A, 0x6B).into()),
            border: Border { radius: 4.0.into(), ..Default::default() },
            ..Default::default()
        }),
    );

    content = content.push(iced::widget::space::vertical());

    content = content.push(
        row![
            text("The PrintCraft Showcase").font(theme::regular()).size(s(8.5)).color(INK_FAINT),
            iced::widget::space::horizontal(),
            text("getartcraft.com").font(theme::regular()).size(s(8.5)).color(INK_FAINT),
        ]
        .align_y(Alignment::Center),
    );

    content.into()
}

// ---------------------------------------------------------------------------------------------
// Find bar
// ---------------------------------------------------------------------------------------------

fn find_bar(app: &App, t: Tokens) -> Element<'_, Message> {
    let total = app.match_count();
    let count = if total == 0 { "No matches".to_string() } else { format!("{} of {}", total.min(9), total) };

    let field = text_input("Find in document", &app.find_query)
        .id(FIND_INPUT)
        .on_input(Message::FindQuery)
        .on_submit(Message::FindNext)
        .font(theme::regular())
        .size(13.0)
        .padding(7)
        .width(Length::Fixed(260.0))
        .style(move |_theme, status| text_input::Style {
            background: t.field.into(),
            border: Border {
                color: if matches!(status, text_input::Status::Focused { .. }) { t.accent } else { t.border },
                width: 1.0,
                radius: 8.0.into()
            },
            icon: t.text_muted,
            placeholder: t.text_faint,
            value: t.text,
            selection: t.accent_soft,
        });

    container(
        row![
            field,
            text(count).font(theme::regular()).size(12.0).color(t.text_muted),
            widgets::icon_button("chevron-up", 28.0, false, true, t, Message::FindPrev),
            widgets::icon_button("chevron-down", 28.0, false, true, t, Message::FindNext),
            iced::widget::space::horizontal(),
            text("⌘G next · ⇧⌘G previous").font(theme::regular()).size(11.0).color(t.text_faint),
            widgets::icon_button("x", 28.0, false, true, t, Message::FindToggle),
        ]
        .spacing(6.0)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .padding(Padding { top: 6.0, right: 12.0, bottom: 6.0, left: 12.0 })
    .style(move |_| container::Style {
        background: Some(t.chrome.into()),
        border: Border { color: t.divider, width: 0.0, radius: 0.0.into() },
        ..Default::default()
    })
    .into()
}

// ---------------------------------------------------------------------------------------------
// Organize pages
// ---------------------------------------------------------------------------------------------

fn organize(app: &App, t: Tokens) -> Element<'_, Message> {
    let Some(doc) = app.doc() else {
        return container(iced::widget::space()).into();
    };
    let selected = app.selected_pages.len();
    let count = if selected == 0 { "No pages selected".to_string() } else { format!("{selected} of {} pages selected", doc.pages) };

    let toolbar = container(
        row![
            text("Organize pages").font(theme::semibold()).size(14.0).color(t.text),
            text(count).font(theme::regular()).size(12.0).color(t.text_muted),
            iced::widget::space::horizontal(),
            widgets::pill_button("Select all", false, t, Message::SelectAllPages),
            widgets::pill_button("Clear", false, t, Message::ClearSelection),
            widgets::pill_button("Rotate", false, t, Message::RotateSelected),
            widgets::pill_button("Extract", false, t, Message::Notify("Extract pages".into())),
            widgets::pill_button("Insert", false, t, Message::Notify("Insert pages".into())),
            widgets::pill_button("Delete", true, t, Message::DeleteSelected),
            widgets::pill_button("Done", false, t, Message::ToggleOrganize),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .padding(Padding { top: 10.0, right: 16.0, bottom: 10.0, left: 16.0 })
    .style(move |_| container::Style {
        background: Some(t.chrome.into()),
        border: Border { color: t.divider, width: 0.0, radius: 0.0.into() },
        ..Default::default()
    });

    // A wrapped grid: rows of five thumbnails.
    const PER_ROW: usize = 5;
    let mut grid = column![].spacing(16.0);
    let mut index = 0;
    while index < doc.pages {
        let mut line = row![].spacing(16.0);
        for _ in 0..PER_ROW {
            if index >= doc.pages {
                break;
            }
            line = line.push(organize_cell(index, app.selected_pages.contains(&index), t));
            index += 1;
        }
        grid = grid.push(line);
    }

    column![
        toolbar,
        widgets::scrolled(container(grid).padding(Padding { top: 20.0, right: 24.0, bottom: 40.0, left: 24.0 }), t).height(Length::Fill),
    ]
    .height(Length::Fill)
    .into()
}

/// One selectable thumbnail in the Organize grid.
fn organize_cell<'a>(index: usize, selected: bool, t: Tokens) -> Element<'a, Message> {
    let mut card = column![
        crate::ui::canvas::page_thumbnail(index, 120.0, 156.0, selected, t),
        row![
            text(format!("{}", index + 1)).font(theme::medium()).size(11.5).color(if selected { t.accent_text } else { t.text_muted }),
            iced::widget::space::horizontal(),
            if selected {
                icons::icon::<Message>("check-circle-2", 15.0, t.accent)
            } else {
                icons::icon::<Message>("circle", 15.0, t.text_faint)
            },
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill),
    ]
    .spacing(6.0)
    .width(Length::Fixed(120.0))
    .align_x(Alignment::Center);

    if selected {
        card = card.push(widgets::badge("Selected", t.accent_soft, t.accent_text));
    }

    button(card)
        .padding(8)
        .on_press(Message::TogglePageSelected(index))
        .style(move |_theme, status| button::Style {
            background: Some(
                (if selected {
                    t.selected
                } else if matches!(status, button::Status::Hovered) {
                    t.hover
                } else {
                    Color::TRANSPARENT
                })
                .into(),
            ),
            text_color: t.text,
            border: Border {
                color: if selected { t.accent } else { Color::TRANSPARENT },
                width: if selected { 1.5 } else { 0.0 },
                radius: 10.0.into(),
            },
            ..Default::default()
        })
        .into()
}

// ---------------------------------------------------------------------------------------------
// Shared page thumbnail
// ---------------------------------------------------------------------------------------------

/// A miniature page used by Organize pages and the Pages panel: white paper with grey text bars.
pub fn page_thumbnail<'a>(index: usize, width: f32, height: f32, active: bool, t: Tokens) -> Element<'a, Message> {
    let scale = width / PAGE_W;
    let bar = |w: f32, h: f32, color: Color| -> Element<'a, Message> {
        container(iced::widget::space())
            .width(Length::Fixed(w * scale))
            .height(Length::Fixed(h * scale))
            .style(move |_| container::Style {
                background: Some(color.into()),
                border: Border { radius: (h * scale / 2.0).into(), ..Default::default() },
                ..Default::default()
            })
            .into()
    };

    let accent_bar: Element<'a, Message> = container(iced::widget::space())
        .width(Length::Fixed(70.0 * scale))
        .height(Length::Fixed(9.0 * scale))
        .style(move |_| container::Style {
            background: Some(ACCENT.into()),
            ..Default::default()
        })
        .into();

    let figure: Element<'a, Message> = container(iced::widget::space())
        .width(Length::Fill)
        .height(Length::Fixed(height * 0.30))
        .style(move |_| container::Style {
            background: Some(Color::from_rgb8(0x1E, 0x3A, 0x6B).into()),
            border: Border { radius: 2.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into();

    let body = column![
        text(format!("Page {}", index + 1)).font(theme::semibold()).size(5.0 + scale * 4.0).color(INK_FAINT),
        widgets::vspace(6.0 * scale),
        accent_bar,
        widgets::vspace(6.0 * scale),
        bar(300.0, 13.0, INK),
        widgets::vspace(5.0 * scale),
        bar(330.0, 9.0, INK_MUTED),
        bar(340.0, 9.0, INK_MUTED),
        bar(320.0, 9.0, INK_MUTED),
        bar(210.0, 9.0, INK_MUTED),
        widgets::vspace(8.0 * scale),
        figure,
    ]
    .spacing(3.0 * scale)
    .width(Length::Fill);

    container(body)
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .padding(Padding { top: 12.0 * scale * 2.0, right: 14.0 * scale * 2.0, bottom: 12.0 * scale * 2.0, left: 14.0 * scale * 2.0 })
        .style(move |_| container::Style {
            background: Some(PAPER.into()),
            border: Border {
                color: if active { ACCENT } else { t.border },
                width: if active { 2.0 } else { 1.0 },
                radius: 3.0.into(),
            },
            shadow: Shadow { color: t.page_shadow, offset: Vector::new(0.0, 2.0), blur_radius: 6.0 },
            ..Default::default()
        })
        .into()
}
