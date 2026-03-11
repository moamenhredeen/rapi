use iced::widget::{column, container, horizontal_rule, pick_list, row, text};
use iced::{Element, Padding};

pub struct SettingsScreen {
    pub theme: iced::Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(iced::Theme),
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self {
            theme: iced::Theme::GruvboxDark,
        }
    }
}

impl SettingsScreen {
    pub fn new(theme: iced::Theme) -> Self {
        Self { theme }
    }
    pub fn update(&mut self, event: Message) {
        match event {
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            container(text("Settings").size(18))
                .padding(Padding::new(0.0).bottom(8.0)),
            horizontal_rule(1),
            container(
                column![
                    text("Appearance").size(15),
                    row![
                        text("Theme").size(13),
                        pick_list(
                            iced::Theme::ALL,
                            Some(&self.theme),
                            |t| Message::ThemeSelected(t),
                        )
                        .width(200),
                    ]
                    .spacing(12)
                    .align_y(iced::Center),
                ]
                .spacing(12),
            )
            .padding([16, 0]),
        ]
        .spacing(8)
        .padding(16)
        .into()
    }
}