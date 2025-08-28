mod screens;
mod widgets;

use iced::{Font, Size};
use iced::widget::Theme;
use iced::window::Settings;
use crate::screens::app_screen::AppScreen;
use crate::screens::contracts::Screen;

pub fn main() -> iced::Result {
    iced::application("APIKIT", AppScreen::update, AppScreen::view)
        .theme(|_| Theme::TokyoNight)
        .font(include_bytes!("JetBrainsMono-Regular.ttf"))
        .default_font(Font::with_name("JetBrains Mono"))
        .window(Settings {
            min_size: Some(Size::new(1024.0, 768.0)),
            ..Settings::default()
        })
        .run()
}