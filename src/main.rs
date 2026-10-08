//! PrintCraft's design language, rebuilt as a minimal iced application.
//!
//! The window is the same workbench PrintCraft ships: a tab strip, a mode bar, a left tool panel,
//! a pasteboard, a right panel, a rail, and the ⌘K palette — all drawn from the same design tokens
//! (`theme.rs`) and the same catalogue (`catalog.rs`), only the toolkit differs.

mod app;
mod catalog;
mod icons;
mod theme;
mod ui;
mod view;
mod widgets;

fn main() -> iced::Result {
    env_logger::init();

    let mut application = iced::application::<app::App, app::Message, iced::Theme, iced::Renderer>(app::App::default, app::update, view::view)
        .title("PrintCraft — iced design demo")
        .theme(|app: &app::App| app.tokens().to_theme())
        .subscription(app::subscription)
        .window_size(iced::Size::new(1440.0, 900.0))
        .default_font(theme::regular())
        .antialiasing(true);

    // The same OFL faces PrintCraft bundles: Inter (400/500/600) and JetBrains Mono.
    for font in theme::fonts() {
        application = application.font(font);
    }

    application.run()
}
