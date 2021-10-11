use orbtk::prelude::themes::THEME_DEFAULT_FONTS;
use orbtk::prelude::themes::THEME_FLUENT;
use orbtk::prelude::themes::THEME_FLUENT_COLORS_LIGHT;
use orbtk::prelude::*;

pub use self::main_state::*;
pub use self::main_view::*;

mod main_state;
mod main_view;

static DARK_EXT: &str = include_str!("assets/calculator.ron");

fn theme() -> Theme {
    orbtk::widgets::themes::theme_orbtk::register_default_fonts(Theme::from_config(
        ThemeConfig::from(DARK_EXT)
            .extend(ThemeConfig::from(THEME_FLUENT))
            .extend(ThemeConfig::from(THEME_FLUENT_COLORS_LIGHT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

fn main() {
    Application::from_name("calculator")
        .theme(theme())
        .window(move |ctx| {
            Window::new()
                .title("Calculator")
                .position((100.0, 100.0))
                .size(310.0, 400.0)
                .resizeable(false)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
