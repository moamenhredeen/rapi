use iced::widget::{column, container, row, text, text_editor};
use iced::{Element, Length};

use crate::http::response::Response;

pub struct ResponseViewer {
    pub response: Option<Response>,
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    #[allow(dead_code)]
    EditorAction(text_editor::Action),
}

impl Default for ResponseViewer {
    fn default() -> Self {
        Self {
            response: None,
            content: text_editor::Content::new(),
        }
    }
}

impl ResponseViewer {
    pub fn set_response(&mut self, response: Response) {
        self.content = text_editor::Content::with_text(&response.body);
        self.response = Some(response);
    }

    pub fn set_error(&mut self, error: &str) {
        self.content = text_editor::Content::with_text(error);
        self.response = None;
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.content = text_editor::Content::new();
        self.response = None;
    }

    pub fn update(&mut self, message: Message) {
        match message {
            // Read-only: ignore edit actions
            Message::EditorAction(_) => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let status_line: Element<'_, Message> = if let Some(ref resp) = self.response {
            let status_color = if resp.status_code < 300 {
                iced::Color::from_rgb(0.2, 0.8, 0.2)
            } else if resp.status_code < 400 {
                iced::Color::from_rgb(0.8, 0.8, 0.2)
            } else {
                iced::Color::from_rgb(0.8, 0.2, 0.2)
            };

            row![
                text(format!("{}", resp.status_code)).color(status_color),
                text(format!("  {}ms", resp.duration.as_millis())),
            ]
            .spacing(10)
            .into()
        } else {
            container("").into()
        };

        column![
            status_line,
            text_editor(&self.content)
                .placeholder("Response will appear here...")
                .height(Length::Fill)
                .highlight("json", iced::highlighter::Theme::SolarizedDark)
                .on_action(Message::EditorAction),
        ]
        .spacing(5)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
