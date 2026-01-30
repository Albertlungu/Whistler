mod app;
mod message;

fn main() -> iced::Result {
    iced::run(app::App::update, app::App::view)
}