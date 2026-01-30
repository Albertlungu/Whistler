mod app;
mod message;

fn main () {
    iced::run(app::App::update, app::App::view);
}