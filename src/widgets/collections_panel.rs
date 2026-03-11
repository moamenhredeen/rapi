use crate::http::method::HttpMethod;
use crate::widgets::key_value_editor::KeyValueEditor;
use iced::widget::{
    button, column, container, horizontal_rule, row, scrollable, text, text_input, Column,
};
use iced::{border, Element, Length, Padding};

#[derive(Debug, Clone)]
pub struct SavedRequest {
    pub name: String,
    pub url: String,
    pub method: HttpMethod,
    pub headers: Vec<(String, String)>,
    pub params: Vec<(String, String)>,
    pub body: Option<String>,
}

impl SavedRequest {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            url: String::new(),
            method: HttpMethod::default(),
            headers: Vec::new(),
            params: Vec::new(),
            body: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<SavedRequest>,
    pub expanded: bool,
}

impl Collection {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            requests: Vec::new(),
            expanded: true,
        }
    }
}

pub struct CollectionsPanel {
    pub collections: Vec<Collection>,
    pub selected: Option<(usize, usize)>,
    new_collection_name: String,
    new_request_name: String,
    adding_request_to: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleCollection(usize),
    SelectRequest(usize, usize),
    UpdateNewCollectionName(String),
    AddCollection,
    RemoveCollection(usize),
    StartAddRequest(usize),
    UpdateNewRequestName(String),
    ConfirmAddRequest,
    CancelAddRequest,
    RemoveRequest(usize, usize),
    SaveCurrent,
}

impl Default for CollectionsPanel {
    fn default() -> Self {
        Self {
            collections: vec![Collection::new("Default")],
            selected: None,
            new_collection_name: String::new(),
            new_request_name: String::new(),
            adding_request_to: None,
        }
    }
}

impl CollectionsPanel {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleCollection(idx) => {
                if let Some(col) = self.collections.get_mut(idx) {
                    col.expanded = !col.expanded;
                }
            }
            Message::SelectRequest(col_idx, req_idx) => {
                self.selected = Some((col_idx, req_idx));
            }
            Message::UpdateNewCollectionName(name) => {
                self.new_collection_name = name;
            }
            Message::AddCollection => {
                let name = self.new_collection_name.trim().to_string();
                if !name.is_empty() {
                    self.collections.push(Collection::new(&name));
                    self.new_collection_name.clear();
                }
            }
            Message::RemoveCollection(idx) => {
                if idx < self.collections.len() {
                    self.collections.remove(idx);
                    if let Some((col, _)) = self.selected {
                        if col == idx {
                            self.selected = None;
                        } else if col > idx {
                            self.selected = Some((col - 1, self.selected.unwrap().1));
                        }
                    }
                }
            }
            Message::StartAddRequest(col_idx) => {
                self.adding_request_to = Some(col_idx);
                self.new_request_name.clear();
            }
            Message::UpdateNewRequestName(name) => {
                self.new_request_name = name;
            }
            Message::ConfirmAddRequest => {
                if let Some(col_idx) = self.adding_request_to {
                    let name = self.new_request_name.trim().to_string();
                    if !name.is_empty() {
                        if let Some(col) = self.collections.get_mut(col_idx) {
                            col.requests.push(SavedRequest::new(&name));
                            col.expanded = true;
                        }
                    }
                }
                self.adding_request_to = None;
                self.new_request_name.clear();
            }
            Message::CancelAddRequest => {
                self.adding_request_to = None;
                self.new_request_name.clear();
            }
            Message::RemoveRequest(col_idx, req_idx) => {
                if let Some(col) = self.collections.get_mut(col_idx) {
                    if req_idx < col.requests.len() {
                        col.requests.remove(req_idx);
                        if let Some((c, r)) = self.selected {
                            if c == col_idx {
                                if r == req_idx {
                                    self.selected = None;
                                } else if r > req_idx {
                                    self.selected = Some((c, r - 1));
                                }
                            }
                        }
                    }
                }
            }
            Message::SaveCurrent => {}
        }
    }

    pub fn save_to_selected(
        &mut self,
        url: &str,
        method: HttpMethod,
        headers: &KeyValueEditor,
        params: &KeyValueEditor,
        body: Option<String>,
    ) {
        if let Some((col_idx, req_idx)) = self.selected {
            if let Some(col) = self.collections.get_mut(col_idx) {
                if let Some(req) = col.requests.get_mut(req_idx) {
                    req.url = url.to_string();
                    req.method = method;
                    req.headers = headers.to_pairs();
                    req.params = params.to_pairs();
                    req.body = body;
                }
            }
        }
    }

    pub fn selected_request(&self) -> Option<&SavedRequest> {
        let (col_idx, req_idx) = self.selected?;
        self.collections.get(col_idx)?.requests.get(req_idx)
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = row![
            text("Collections").size(13).width(Length::Fill),
            button(
                crate::icon::plus()
                    .size(11)
                    .center()
                    .width(Length::Fill)
            )
            .width(24)
            .height(24)
            .on_press(Message::SaveCurrent)
            .style(|theme: &iced::Theme, status| {
                button::Style {
                    background: None,
                    text_color: theme.extended_palette().background.base.text,
                    border: border::rounded(4),
                    ..button::text(theme, status)
                }
            }),
        ]
        .align_y(iced::Center)
        .padding([0, 4]);

        let new_collection = row![
            text_input("New collection...", &self.new_collection_name)
                .on_input(Message::UpdateNewCollectionName)
                .on_submit(Message::AddCollection)
                .size(12)
                .width(Length::Fill),
            button(
                crate::icon::plus()
                    .size(10)
                    .center()
                    .width(Length::Fill)
            )
            .width(24)
            .height(24)
            .on_press(Message::AddCollection)
            .style(|theme: &iced::Theme, status| {
                let palette = theme.extended_palette();
                button::Style {
                    background: Some(palette.success.base.color.into()),
                    text_color: palette.success.base.text,
                    border: border::rounded(4),
                    ..button::primary(theme, status)
                }
            }),
        ]
        .spacing(4)
        .align_y(iced::Center)
        .padding([0, 4]);

        let mut list = Column::new().spacing(2);

        for (col_idx, collection) in self.collections.iter().enumerate() {
            let toggle_icon = if collection.expanded {
                crate::icon::down_open()
            } else {
                crate::icon::right_open()
            };

            let collection_header = button(
                row![
                    toggle_icon.size(10),
                    crate::icon::folder().size(12),
                    text(&collection.name).size(12).width(Length::Fill),
                ]
                .spacing(6)
                .align_y(iced::Center),
            )
            .width(Length::Fill)
            .padding([6, 8])
            .on_press(Message::ToggleCollection(col_idx))
            .style(|theme: &iced::Theme, status| {
                let palette = theme.extended_palette();
                button::Style {
                    background: None,
                    text_color: palette.background.base.text,
                    border: border::rounded(4),
                    ..button::text(theme, status)
                }
            });

            let collection_actions = row![
                button(
                    crate::icon::plus()
                        .size(9)
                        .center()
                        .width(Length::Fill)
                )
                .width(20)
                .height(20)
                .on_press(Message::StartAddRequest(col_idx))
                .style(|theme: &iced::Theme, status| {
                    button::Style {
                        background: None,
                        text_color: theme.extended_palette().background.base.text,
                        border: border::rounded(3),
                        ..button::text(theme, status)
                    }
                }),
                button(
                    crate::icon::trash()
                        .size(9)
                        .center()
                        .width(Length::Fill)
                )
                .width(20)
                .height(20)
                .on_press(Message::RemoveCollection(col_idx))
                .style(|theme: &iced::Theme, status| {
                    button::Style {
                        background: None,
                        text_color: theme.extended_palette().danger.base.color,
                        border: border::rounded(3),
                        ..button::text(theme, status)
                    }
                }),
            ]
            .spacing(2);

            let collection_row = row![collection_header, collection_actions]
                .align_y(iced::Center);

            list = list.push(collection_row);

            if collection.expanded {
                // Show add-request input if active for this collection
                if self.adding_request_to == Some(col_idx) {
                    let add_input = container(
                        row![
                            text_input("Request name...", &self.new_request_name)
                                .on_input(Message::UpdateNewRequestName)
                                .on_submit(Message::ConfirmAddRequest)
                                .size(11)
                                .width(Length::Fill),
                            button(
                                crate::icon::plus()
                                    .size(9)
                                    .center()
                                    .width(Length::Fill)
                            )
                            .width(20)
                            .height(20)
                            .on_press(Message::ConfirmAddRequest)
                            .style(|theme: &iced::Theme, status| {
                                let palette = theme.extended_palette();
                                button::Style {
                                    background: Some(palette.success.base.color.into()),
                                    text_color: palette.success.base.text,
                                    border: border::rounded(3),
                                    ..button::primary(theme, status)
                                }
                            }),
                            button(
                                crate::icon::cancel()
                                    .size(9)
                                    .center()
                                    .width(Length::Fill)
                            )
                            .width(20)
                            .height(20)
                            .on_press(Message::CancelAddRequest)
                            .style(|theme: &iced::Theme, status| {
                                button::Style {
                                    background: None,
                                    text_color: theme.extended_palette().danger.base.color,
                                    border: border::rounded(3),
                                    ..button::text(theme, status)
                                }
                            }),
                        ]
                        .spacing(4)
                        .align_y(iced::Center),
                    )
                    .padding(Padding::new(0.0).left(28.0).right(4.0));

                    list = list.push(add_input);
                }

                for (req_idx, request) in collection.requests.iter().enumerate() {
                    let is_selected = self.selected == Some((col_idx, req_idx));

                    let method_color = method_color(&request.method);

                    let req_button = button(
                        row![
                            text(format!("{}", request.method))
                                .size(9)
                                .color(method_color),
                            text(&request.name).size(12),
                        ]
                        .spacing(6)
                        .align_y(iced::Center),
                    )
                    .width(Length::Fill)
                    .padding([4, 8])
                    .on_press(Message::SelectRequest(col_idx, req_idx))
                    .style(move |theme: &iced::Theme, status| {
                        let palette = theme.extended_palette();
                        if is_selected {
                            button::Style {
                                background: Some(palette.primary.weak.color.into()),
                                text_color: palette.primary.weak.text,
                                border: border::rounded(4),
                                ..button::primary(theme, status)
                            }
                        } else {
                            button::Style {
                                background: None,
                                text_color: palette.background.base.text,
                                border: border::rounded(4),
                                ..button::text(theme, status)
                            }
                        }
                    });

                    let delete_btn = button(
                        crate::icon::cancel()
                            .size(8)
                            .center()
                            .width(Length::Fill),
                    )
                    .width(18)
                    .height(18)
                    .on_press(Message::RemoveRequest(col_idx, req_idx))
                    .style(|theme: &iced::Theme, status| {
                        button::Style {
                            background: None,
                            text_color: theme.extended_palette().danger.base.color,
                            border: border::rounded(3),
                            ..button::text(theme, status)
                        }
                    });

                    let req_row = container(
                        row![req_button, delete_btn].align_y(iced::Center),
                    )
                    .padding(Padding::new(0.0).left(20.0));

                    list = list.push(req_row);
                }
            }
        }

        container(
            column![
                header,
                container(horizontal_rule(1)).padding([4, 0]),
                new_collection,
                container(horizontal_rule(1)).padding([4, 0]),
                scrollable(list.padding([0, 4])).height(Length::Fill),
            ]
            .spacing(4)
            .padding([8, 4]),
        )
        .width(240)
        .height(Length::Fill)
        .style(|theme: &iced::Theme| {
            let palette = theme.extended_palette();
            iced::widget::container::Style {
                background: Some(palette.background.weak.color.into()),
                border: iced::Border {
                    width: 0.0,
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
    }
}

fn method_color(method: &HttpMethod) -> iced::Color {
    match method {
        HttpMethod::Get => iced::Color::from_rgb(0.0, 0.75, 0.0),
        HttpMethod::Post => iced::Color::from_rgb(1.0, 0.65, 0.0),
        HttpMethod::Put => iced::Color::from_rgb(0.0, 0.5, 1.0),
        HttpMethod::Delete => iced::Color::from_rgb(1.0, 0.2, 0.2),
    }
}
