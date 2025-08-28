use iced::{Element, Task};
use crate::screens::contracts::Screen;
use crate::screens::route::Route;

pub struct SettingsScreen {

}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateTheme,
    Navigate(Route)
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self {}
    }
}

impl Screen<Message> for SettingsScreen {

    fn update(&mut self, event: Message) -> Task<Message> {
        match event {
            Message::UpdateTheme => Task::none(),
            Message::Navigate(screen) => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        "hello world".into()
    }
}