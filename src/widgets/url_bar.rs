use iced::widget::{button, pick_list, row, text_input};
use iced::Element;

use crate::http::method::HttpMethod;

pub struct UrlBar {
    pub url: String,
    pub method: HttpMethod,
    pub loading: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateUrl(String),
    UpdateMethod(HttpMethod),
    Send,
}

impl Default for UrlBar {
    fn default() -> Self {
        Self {
            url: String::new(),
            method: HttpMethod::default(),
            loading: false,
        }
    }
}

impl UrlBar {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateUrl(url) => self.url = url,
            Message::UpdateMethod(method) => self.method = method,
            Message::Send => {} // handled by parent
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let send_button = if self.loading {
            button("Loading ...").width(100)
        } else {
            button("Send").width(100).on_press(Message::Send)
        };

        row![
            pick_list(
                HttpMethod::ALL,
                Some(self.method),
                Message::UpdateMethod,
            ),
            text_input("Enter URL...", &self.url)
                .on_input(Message::UpdateUrl),
            send_button,
        ]
        .spacing(10)
        .into()
    }
}
