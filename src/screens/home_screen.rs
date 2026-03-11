use crate::http;
use crate::widgets::body_editor::{self, BodyEditor};
use crate::widgets::key_value_editor::{self, KeyValueEditor};
use crate::widgets::response_viewer::{self, ResponseViewer};
use crate::widgets::tab_bar;
use crate::widgets::url_bar::{self, UrlBar};
use iced::widget::{column, progress_bar, row};
use iced::{Element, Length, Task};

pub struct HomeScreen {
    url_bar: UrlBar,
    params_editor: KeyValueEditor,
    headers_editor: KeyValueEditor,
    body_editor: BodyEditor,
    response_viewer: ResponseViewer,
    selected_request_tab: RequestTab,
    loading: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlBar(url_bar::Message),
    Params(key_value_editor::Message),
    Headers(key_value_editor::Message),
    Body(body_editor::Message),
    Response(response_viewer::Message),
    SelectRequestTab(RequestTab),
    RequestDone(Result<http::response::Response, String>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RequestTab {
    Params,
    Headers,
    Body,
}

impl Default for HomeScreen {
    fn default() -> Self {
        Self {
            url_bar: UrlBar::default(),
            params_editor: KeyValueEditor::default(),
            headers_editor: KeyValueEditor::default(),
            body_editor: BodyEditor::default(),
            response_viewer: ResponseViewer::default(),
            selected_request_tab: RequestTab::Params,
            loading: false,
        }
    }
}

impl HomeScreen {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UrlBar(msg) => {
                if matches!(msg, url_bar::Message::Send) {
                    return self.send_request();
                }
                self.url_bar.update(msg);
            }
            Message::Params(msg) => self.params_editor.update(msg),
            Message::Headers(msg) => self.headers_editor.update(msg),
            Message::Body(msg) => self.body_editor.update(msg),
            Message::Response(msg) => self.response_viewer.update(msg),
            Message::SelectRequestTab(tab) => self.selected_request_tab = tab,
            Message::RequestDone(result) => {
                self.loading = false;
                self.url_bar.loading = false;
                match result {
                    Ok(response) => self.response_viewer.set_response(response),
                    Err(error) => self.response_viewer.set_error(&error),
                }
            }
        }
        Task::none()
    }

    fn send_request(&mut self) -> Task<Message> {
        self.loading = true;
        self.url_bar.loading = true;

        let request = http::request::Request {
            url: self.url_bar.url.clone(),
            method: self.url_bar.method,
            headers: self.headers_editor.to_pairs(),
            params: self.params_editor.to_pairs(),
            body: {
                let text = self.body_editor.text();
                if text.trim().is_empty() {
                    None
                } else {
                    Some(text)
                }
            },
        };

        Task::perform(
            async move { http::client::execute(request).await },
            Message::RequestDone,
        )
    }

    pub fn view(&self) -> Element<'_, Message> {
        let tabs = vec![
            (RequestTab::Params, "Params"),
            (RequestTab::Headers, "Headers"),
            (RequestTab::Body, "Body"),
        ];

        let request_tab_content: Element<'_, Message> = match self.selected_request_tab {
            RequestTab::Params => self.params_editor.view().map(Message::Params),
            RequestTab::Headers => self.headers_editor.view().map(Message::Headers),
            RequestTab::Body => self.body_editor.view().map(Message::Body),
        };

        let progress = if self.loading {
            progress_bar(0.0..=100.0, 100.0).height(3)
        } else {
            progress_bar(0.0..=100.0, 0.0).height(3)
        };

        column![
            progress,
            self.url_bar.view().map(Message::UrlBar),
            row![
                column![
                    tab_bar::tab_bar(tabs, &self.selected_request_tab, |t| {
                        Message::SelectRequestTab(t)
                    }),
                    request_tab_content,
                ]
                .width(Length::FillPortion(1))
                .height(Length::Fill),
                self.response_viewer.view().map(Message::Response),
            ]
            .spacing(10)
            .height(Length::Fill),
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
