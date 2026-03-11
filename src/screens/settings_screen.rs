use iced::widget::{column, container, horizontal_rule, text};
use iced::{Element, Padding};

pub struct SettingsScreen;

#[derive(Debug, Clone)]
pub enum Message {
    #[allow(dead_code)]
    UpdateTheme,
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self
    }
}

impl SettingsScreen {
    pub fn update(&mut self, event: Message) {
        match event {
            Message::UpdateTheme => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            container(text("Settings").size(18))
                .padding(Padding::new(0.0).bottom(8.0)),
            horizontal_rule(1),
            container(
                column![
                    text("General").size(15),
                    text("Theme, font, and editor preferences will be configurable here.")
                        .size(13),
                ]
                .spacing(8),
            )
            .padding([16, 0]),
        ]
        .spacing(8)
        .padding(16)
        .into()
    }
}