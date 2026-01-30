use iced::widget::{column, text};
use iced::Element;
use crate::message::Message;

#[derive(Default)]
pub struct App {
    // I need to put my application state fields here
}

impl App {
    pub fn update(&mut self, message: Message) {
        // Here ima handle the messages
        match message {
            // Add more cases here as I define more messages
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            text("Whistler IDE")
        ].into()
    }
}