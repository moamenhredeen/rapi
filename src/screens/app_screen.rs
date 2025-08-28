use iced::{Background, Color, Element, Length, Task};
use crate::screens::contracts::Screen;
use crate::screens::route::Route;
use iced::widget::{row, column, container};
use crate::widgets::status_bar;

pub struct AppScreen;

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Route),
}


impl Default for AppScreen {
    fn default() -> Self {
        Self{}
    }
}

impl Screen<Message> for AppScreen {

    fn update(&mut self, message:  Message) -> Task<Message> {
        match message {
            Message::Navigate(screen) => {
                match screen {
                    Route::Home => {
                    },
                    Route::Settings => {},
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            row![
                container("side bar")
                    .width(250)
                    .style(|_| container::background(Background::Color(Color::new(0.0, 0.0, 0.8, 1.0))))
                    .height(Length::Fill),
                container("main content")
                    .width(Length::Fill)
                    .style(|_| container::background(Background::Color(Color::new(0.0, 0.0, 0.2, 1.0))))
                    .height(Length::Fill),
            ],
            status_bar::status_bar("some message"),
        ].into()
    }
}
