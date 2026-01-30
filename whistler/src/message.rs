use iced::widget::text_editor::Action;

#[derive(Debug, Clone)]
pub enum Message {
    EditorAction(Action),
}