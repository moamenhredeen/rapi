use std::{fmt::Display, time::Duration};

use iced::{widget::{ button, column, pick_list, row, text_editor::{Content}, text_input, Button, Row, TextEditor, Theme}, Task};
use iced::widget::text_editor;

pub fn main() -> iced::Result {
    iced::application("APIKIT", AppState::update, AppState::view)
        .theme(|_| Theme::TokyoNight)
        .run()
}

struct AppState {
    url: String,
    loading: bool,
    screen: Screen,
    http_method: HttpMethod,
    response: text_editor::Content
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            url: "".to_owned(),
            screen: Screen::Main,
            http_method: HttpMethod::Get,
            loading: false,
            response: Content::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Main,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Request,
    Navigate(Screen),
    UpdateUrl(String),
    UpdateHttpMethod(HttpMethod),
    Done,
    DoNothing
}

impl AppState {
    fn update(&mut self, event: Message) -> Task<Message> {
        match event {
            Message::Request => {
                self.loading = true;
                return Task::perform(tokio::time::sleep(Duration::from_secs(2)), |_| {
                    Message::Done
                });
            }
            Message::Navigate(screen) => self.screen = screen,
            Message::UpdateUrl(url) => self.url = url,
            Message::UpdateHttpMethod(http_method) => self.http_method = http_method,
            Message::Done => self.loading = false,
            Message::DoNothing => {}
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let methods = vec![
            HttpMethod::Get,
            HttpMethod::Post,
            HttpMethod::Put,
            HttpMethod::Delete,
        ];
        
        let send_button: Button<Message> = if self.loading {
            button("Loading ...").width(100)
        } else {
            button("Send").width(100).on_press(Message::Request)
        };

        column![
            row![
                pick_list(methods, Some(self.http_method), |method| {
                    Message::UpdateHttpMethod(method)
                }),
                text_input("enter url", self.url.as_ref()).on_input(|s| Message::UpdateUrl(s)),
                send_button
            ].spacing(10).padding(10).into(),

            text_editor(&self.response)
            .placeholder("reqeust body")
            .into()
        ]
        .into()
    }
}
