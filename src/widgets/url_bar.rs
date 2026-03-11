use iced::widget::{button, pick_list, row, text_input};
use iced::{border, Element};

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
            Message::Send => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let send_button = if self.loading {
            button("Sending...")
                .width(100)
                .style(|theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();
                    button::Style {
                        background: Some(palette.primary.weak.color.into()),
                        text_color: palette.primary.weak.text,
                        border: border::rounded(6),
                        ..button::primary(theme, status)
                    }
                })
        } else {
            button("Send")
                .width(100)
                .on_press(Message::Send)
                .style(|theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();
                    button::Style {
                        background: Some(palette.success.base.color.into()),
                        text_color: palette.success.base.text,
                        border: border::rounded(6),
                        ..button::primary(theme, status)
                    }
                })
        };

        row![
            pick_list(
                HttpMethod::ALL,
                Some(self.method),
                Message::UpdateMethod,
            )
            .width(100),
            text_input("Enter URL...", &self.url)
                .on_input(Message::UpdateUrl)
                .on_submit(Message::Send),
            send_button,
        ]
        .spacing(8)
        .into()
    }
}
