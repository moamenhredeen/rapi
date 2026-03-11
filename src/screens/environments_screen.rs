use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input};
use iced::{border, Element, Length, Padding};

use crate::widgets::key_value_editor::{self, KeyValueEditor};

#[derive(Debug, Clone)]
pub struct Environment {
    pub name: String,
    pub variables: KeyValueEditor,
}

impl Environment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            variables: KeyValueEditor::default(),
        }
    }
}

pub struct EnvironmentsScreen {
    environments: Vec<Environment>,
    selected_index: Option<usize>,
    new_env_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectEnvironment(String),
    Variables(key_value_editor::Message),
    UpdateNewEnvName(String),
    AddEnvironment,
    RemoveEnvironment,
}

impl Default for EnvironmentsScreen {
    fn default() -> Self {
        Self {
            environments: vec![Environment::new("Default")],
            selected_index: Some(0),
            new_env_name: String::new(),
        }
    }
}

impl EnvironmentsScreen {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectEnvironment(name) => {
                self.selected_index = self.environments.iter().position(|e| e.name == name);
            }
            Message::Variables(msg) => {
                if let Some(idx) = self.selected_index {
                    if let Some(env) = self.environments.get_mut(idx) {
                        env.variables.update(msg);
                    }
                }
            }
            Message::UpdateNewEnvName(name) => {
                self.new_env_name = name;
            }
            Message::AddEnvironment => {
                let name = self.new_env_name.trim().to_string();
                if !name.is_empty() && !self.environments.iter().any(|e| e.name == name) {
                    self.environments.push(Environment::new(&name));
                    self.selected_index = Some(self.environments.len() - 1);
                    self.new_env_name.clear();
                }
            }
            Message::RemoveEnvironment => {
                if let Some(idx) = self.selected_index {
                    if self.environments.len() > 1 {
                        self.environments.remove(idx);
                        self.selected_index =
                            Some(idx.min(self.environments.len().saturating_sub(1)));
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let env_names: Vec<String> = self.environments.iter().map(|e| e.name.clone()).collect();
        let selected_name = self
            .selected_index
            .and_then(|i| self.environments.get(i))
            .map(|e| e.name.clone());

        let env_selector = row![
            pick_list(env_names, selected_name, Message::SelectEnvironment)
                .width(Length::FillPortion(2))
                .placeholder("Select environment..."),
            text_input("New environment...", &self.new_env_name)
                .on_input(Message::UpdateNewEnvName)
                .on_submit(Message::AddEnvironment)
                .width(Length::FillPortion(2)),
            button(text("+").center().width(Length::Fill))
                .width(36)
                .style(|theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();
                    button::Style {
                        background: Some(palette.success.base.color.into()),
                        text_color: palette.success.base.text,
                        border: border::rounded(6),
                        ..button::primary(theme, status)
                    }
                })
                .on_press(Message::AddEnvironment),
            button(text("🗑").center().width(Length::Fill))
                .width(36)
                .style(button::danger)
                .on_press(Message::RemoveEnvironment),
        ]
        .spacing(8)
        .align_y(iced::Center);

        let variables_section: Element<'_, Message> =
            if let Some(idx) = self.selected_index {
                if let Some(env) = self.environments.get(idx) {
                    column![
                        container(
                            text(format!("Variables for: {}", env.name))
                                .size(14),
                        )
                        .padding([8, 0]),
                        container(
                            row![
                                text("Key").width(Length::FillPortion(1)).size(12),
                                text("Value").width(Length::FillPortion(1)).size(12),
                                container("").width(36),
                            ]
                            .spacing(5),
                        )
                        .padding([4, 0]),
                        scrollable(
                            env.variables.view().map(Message::Variables),
                        )
                        .height(Length::Fill),
                    ]
                    .spacing(4)
                    .into()
                } else {
                    text("Select an environment").into()
                }
            } else {
                text("Select an environment to edit variables").into()
            };

        column![
            container(
                text("Environments")
                    .size(18),
            )
            .padding(Padding::new(0.0).bottom(8.0)),
            env_selector,
            container(iced::widget::horizontal_rule(1)).padding([8, 0]),
            variables_section,
        ]
        .spacing(8)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
