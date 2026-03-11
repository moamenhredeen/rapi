use iced::widget::text;
use iced::Element;

pub struct SettingsScreen;

#[derive(Debug, Clone)]
pub enum Message {
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
        text("Settings — coming soon").into()
    }
}