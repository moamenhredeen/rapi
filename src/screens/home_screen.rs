use std::fmt::Display;
use std::str::FromStr;
use iced::{Task, Length};
use iced::widget::{text_editor, button, column, row, progress_bar, text_input, pick_list, Button, TextInput, Text, ProgressBar, PickList};
use reqwest::header::HeaderValue;
use crate::screens::contracts::Screen;
use crate::screens::route::Route;

pub struct HomeScreen {
    url: String,
    loading: bool,
    http_method: HttpMethod,
    response: text_editor::Content,
    request: text_editor::Content,
    selected_request_tab: RequestTab,
}


#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Route),
    Request,
    UpdateUrl(String),
    UpdateHttpMethod(HttpMethod),
    Done(Result<String, String>),
    RequestUpdate(text_editor::Action),
    ResponseAction(text_editor::Action),
    SelectRequestTab(RequestTab),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}


#[derive(Debug, Clone, Eq, PartialEq)]
enum RequestTab {
    Params,
    Headers,
    Body,
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


impl Default for HomeScreen {
    fn default() -> Self {
        Self {
            url: "".to_owned(),
            http_method: HttpMethod::Get,
            loading: false,
            response: text_editor::Content::new(),
            request: text_editor::Content::new(),
            selected_request_tab: RequestTab::Params,
        }
    }
}

impl Screen<Message> for HomeScreen {
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
            Message::SelectRequestTab(tab) => self.selected_request_tab = tab,
            Message::Navigate(_) => todo!(),
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
            ].spacing(10),

            row![
                column![
                    row![
                        button("Params")
                            .width(Length::FillPortion(1))
                            .style(button::primary)
                            .on_press(Message::SelectRequestTab(RequestTab::Params)),
                        button("Headers")
                            .width(Length::FillPortion(1))
                            .style(button::text)
                            .on_press(Message::SelectRequestTab(RequestTab::Headers)),
                        button("Body")
                            .width(Length::FillPortion(1))
                            .style(button::text)
                            .on_press(Message::SelectRequestTab(RequestTab::Body))
                    ],
                    // text_editor(&self.request)
                    //     .placeholder("reqeust body")
                    //     .height(Length::Fill)
                    //     .on_action(|action| HomeEvent::RequestUpdate(action))
                ],
                text_editor(&self.response)
                    .placeholder("reqeust body")
                    .height(Length::Fill)
                    .highlight("json", iced::highlighter::Theme::SolarizedDark)
                    .on_action(Message::ResponseAction),
            ]
            .spacing(10)
        ]
            .spacing(10)
            .padding(10)
            .into()
    }
}