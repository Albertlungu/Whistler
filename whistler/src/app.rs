use iced::widget::{column, text};
use iced::widget::text_editor::{TextEditor, Content};
use iced::Element;
use crate::message::Message;

#[derive(Default)]
pub struct App {
    editor_content: Content,
}

// impl Default for App {
//     fn default() -> Self {
//         Self {
//             editor_content: Content::new(""), // Empty string for initialization of editor
//         }
//     }
// }

impl App {
    pub fn update(&mut self, message: Message) { // &mut self basically means this function is allowed to modify states in App
        // This is the only place that mutation should happen
        match message {
            Message::EditorAction(action) => { // "if the Message (which is an enum) is the TextChanged variant, take the String inside and call it new_value"
                self.editor_content.perform(action);
            } // Message::TextChanged(new_value) checks the type and unpacks the value. => is essentially an "if" statement in Python
        } // :: <=> Message.TextChanged in Python
    }
    // Element<Message> is essentially type annotation (like in Python, which would mean an Element that se3nds Message objects when interacted with)
    pub fn view(&self) -> Element<Message> { // This means the view is only allowed to read, not change states
        let editor = TextEditor::new(&self.editor_content)
            .on_action(Message::EditorAction);
        column![ // ! = macro: generates at compile time
            text("Whistler IDE"),
            editor
        ]
        .into()
    }
}