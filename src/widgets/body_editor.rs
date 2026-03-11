use iced::widget::text_editor;
use iced::{Element, Length};

pub struct BodyEditor {
    pub content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
}

impl Default for BodyEditor {
    fn default() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }
}

impl BodyEditor {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        text_editor(&self.content)
            .placeholder("Request body...")
            .height(Length::Fill)
            .highlight("json", iced::highlighter::Theme::SolarizedDark)
            .on_action(Message::Edit)
            .into()
    }

    pub fn text(&self) -> String {
        self.content.text()
    }
}
