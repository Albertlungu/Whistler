use iced::window;

mod app;
mod message;
mod file_tree;
mod icons;
mod syntax;
mod theme;
mod ui;

const FIRA_CODE: &[u8] = include_bytes!("../fonts/FiraCode/ttf/FiraCode-Regular.ttf");

fn main() -> iced::Result {
    let icon_data = include_bytes!("../../assets/icon.png");
    let icon = window::icon::from_file_data(
        icon_data, None)
        .expect("Failed to load icon.");

    iced::application(app::App::default, app::App::update, app::App::view)
        .title("Whistler")
        .subscription(|app| app.subscription())
        .font(FIRA_CODE)
        .default_font(iced::Font {
            family: iced::font::Family::Name("Fira Code"),
            ..iced::Font::DEFAULT
        })
        .window_size((1200.0, 800.0))
        .window(window::Settings{
            size: [1200.0, 800.0].into(),
            icon: Some(icon),
            ..Default::default()
        })
        .run()
}
