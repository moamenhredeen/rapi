mod config;
mod http;
mod icon;
mod screens;
mod widgets;

use crate::screens::app_screen::AppScreen;
use iced::window::Settings;
use iced::{Font, Size};

pub fn main() -> iced::Result {
    iced::application("APIKIT", AppScreen::update, AppScreen::view)
        .theme(AppScreen::theme)
        .font(include_bytes!("JetBrainsMono-Regular.ttf"))
        .font(icon::FONT)
        .default_font(Font::with_name("JetBrains Mono"))
        .window(Settings {
            min_size: Some(Size::new(1024.0, 768.0)),
            ..Settings::default()
        })
        .run()
}
