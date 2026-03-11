use crate::screens::home_screen;
use crate::screens::home_screen::HomeScreen;
use crate::screens::route::Route;
use crate::screens::settings_screen;
use crate::screens::settings_screen::SettingsScreen;
use crate::widgets::side_bar;
use crate::widgets::status_bar;
use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

pub struct AppScreen {
    route: Route,
    home: HomeScreen,
    settings: SettingsScreen,
    status_message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Route),
    Home(home_screen::Message),
    Settings(settings_screen::Message),
}

impl Default for AppScreen {
    fn default() -> Self {
        Self {
            route: Route::Home,
            home: HomeScreen::default(),
            settings: SettingsScreen::default(),
            status_message: "Ready".to_string(),
        }
    }
}

impl AppScreen {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(route) => {
                self.route = route;
                Task::none()
            }
            Message::Home(msg) => self.home.update(msg).map(Message::Home),
            Message::Settings(msg) => {
                self.settings.update(msg);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar_items = vec![
            ("Home", Route::Home),
            ("Settings", Route::Settings),
        ];

        let sidebar: Element<'_, Message> = {
            let items: Vec<Element<'_, Message>> = sidebar_items
                .into_iter()
                .map(|(label, route)| {
                    let is_active = self.route == route;
                    side_bar::item(text(label), is_active, move || {
                        Message::Navigate(route.clone())
                    })
                })
                .collect();

            container(column(items).spacing(5).padding(10))
                .width(200)
                .height(Length::Fill)
                .style(|theme: &iced::Theme| {
                    let palette = theme.extended_palette();
                    container::background(palette.background.weak.color)
                })
                .into()
        };

        let content: Element<'_, Message> = match self.route {
            Route::Home => self.home.view().map(Message::Home),
            Route::Settings => self.settings.view().map(Message::Settings),
        };

        column![
            row![
                sidebar,
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(10),
            ]
            .height(Length::Fill),
            status_bar::status_bar(&self.status_message),
        ]
        .into()
    }
}
