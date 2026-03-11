use iced::widget::{column, container, row, text, text_editor};
use iced::{border, Element, Length, Padding};

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
            Message::EditorAction(_) => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let status_line: Element<'_, Message> = if let Some(ref resp) = self.response {
            let (status_color, bg_color) = if resp.status_code < 300 {
                (
                    iced::Color::from_rgb(0.15, 0.75, 0.35),
                    iced::Color::from_rgba(0.15, 0.75, 0.35, 0.15),
                )
            } else if resp.status_code < 400 {
                (
                    iced::Color::from_rgb(0.85, 0.65, 0.1),
                    iced::Color::from_rgba(0.85, 0.65, 0.1, 0.15),
                )
            } else {
                (
                    iced::Color::from_rgb(0.85, 0.25, 0.25),
                    iced::Color::from_rgba(0.85, 0.25, 0.25, 0.15),
                )
            };

            row![
                container(text(format!("{}", resp.status_code)).size(13).color(status_color))
                    .padding([3, 8])
                    .style(move |_theme: &iced::Theme| container::Style {
                        background: Some(bg_color.into()),
                        border: border::rounded(4),
                        ..container::Style::default()
                    }),
                text(format!("{}ms", resp.duration.as_millis())).size(13),
                text(format!(
                    "{} bytes",
                    resp.body.len()
                ))
                .size(13),
            ]
            .spacing(12)
            .align_y(iced::Center)
            .into()
        } else {
            container(text("No response yet").size(13))
                .padding([3, 0])
                .into()
        };

        column![
            container(text("Response").size(12)).padding(Padding::new(0.0).bottom(2.0)),
            status_line,
            text_editor(&self.content)
                .placeholder("Response will appear here...")
                .height(Length::Fill)
                .highlight("json", iced::highlighter::Theme::SolarizedDark)
                .on_action(Message::EditorAction),
        ]
        .spacing(6)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
