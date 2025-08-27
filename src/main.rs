use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use iced::widget::{
    Button, Theme, button, column, pick_list, progress_bar, row, text_editor, text_input,
};
use iced::{Length, Task};
use reqwest::{self, header::HeaderValue};

pub fn main() -> iced::Result {
    iced::application("APIKIT", AppState::update, AppState::view)
        .theme(|_| Theme::TokyoNight)
        .run()
}

struct AppState {
    url: String,
    loading: bool,
    http_method: HttpMethod,
    response: text_editor::Content,
    request: text_editor::Content,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            url: "".to_owned(),
            http_method: HttpMethod::Get,
            loading: false,
            response: text_editor::Content::new(),
            request: text_editor::Content::new(),
        }
    }
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
    UpdateUrl(String),
    UpdateHttpMethod(HttpMethod),
    Done(Result<String, String>),
    RequestUpdate(text_editor::Action),
    ResponseAction(text_editor::Action),
}

impl AppState {
    fn update(&mut self, event: Message) -> Task<Message> {
        match event {
            Message::Request => {
                self.loading = true;
                let url = self.url.clone();
                let method: reqwest::Method = match self.http_method {
                    HttpMethod::Get => reqwest::Method::GET,
                    HttpMethod::Post => reqwest::Method::POST,
                    HttpMethod::Put => reqwest::Method::PUT,
                    HttpMethod::Delete => reqwest::Method::DELETE,
                };
                return Task::perform(
                    async move {
                        let client = reqwest::Client::new();
                        let mut request =
                            reqwest::Request::new(method, reqwest::Url::from_str(&url).unwrap());
                        request.headers_mut().append(
                            "Content-Type",
                            HeaderValue::from_str("application/json").unwrap(),
                        );
                        match client.execute(request).await {
                            Ok(response) => match response.text().await {
                                Ok(text) => Message::Done(Ok(text)),
                                Err(err) => {
                                    Message::Done(Err(format!("Failed to read response: {}", err)))
                                }
                            },
                            Err(err) => Message::Done(Err(format!("Request failed: {}", err))),
                        }
                    },
                    |msg| msg,
                );
            }
            Message::UpdateUrl(url) => self.url = url,
            Message::UpdateHttpMethod(http_method) => self.http_method = http_method,
            Message::RequestUpdate(action) => {
                self.request.perform(action);
            }
            Message::ResponseAction(action) => {
                self.response.perform(action);
            }
            Message::Done(result) => {
                self.loading = false;
                match result {
                    Ok(text) => {
                        self.response = text_editor::Content::with_text(&text);
                    }
                    Err(error) => {
                        self.response = text_editor::Content::with_text(&error);
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
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
            progress_bar(0.0..=100.0, 20.0).height(4),
            row![
                pick_list(methods, Some(self.http_method), |method| {
                    Message::UpdateHttpMethod(method)
                }),
                text_input("enter url", self.url.as_ref()).on_input(|s| Message::UpdateUrl(s)),
                send_button
            ]
            .spacing(10),
            row![
                text_editor(&self.request)
                    .placeholder("reqeust body")
                    .height(Length::Fill)
                    .on_action(|action| Message::RequestUpdate(action)),
                text_editor(&self.response)
                    .placeholder("reqeust body")
                    .height(Length::Fill)
                    .on_action(|action| Message::ResponseAction(action)),
            ]
            .spacing(10)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
