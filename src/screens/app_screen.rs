use crate::config::AppConfig;
use crate::screens::environments_screen;
use crate::screens::environments_screen::EnvironmentsScreen;
use crate::screens::home_screen;
use crate::screens::home_screen::HomeScreen;
use crate::screens::route::Route;
use crate::screens::settings_screen;
use crate::screens::settings_screen::SettingsScreen;
use crate::widgets::activity_bar::{self, ActivityBarItem};
use crate::widgets::status_bar;
use iced::widget::{column, container, row};
use iced::{Element, Length, Task};

pub struct AppScreen {
    route: Route,
    collections: HomeScreen,
    environments: EnvironmentsScreen,
    settings: SettingsScreen,
    config: AppConfig,
    status_message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Route),
    Collections(home_screen::Message),
    Environments(environments_screen::Message),
    Settings(settings_screen::Message),
}

impl Default for AppScreen {
    fn default() -> Self {
        let config = AppConfig::load();
        let settings = SettingsScreen::new(config.theme());
        Self {
            route: Route::Collections,
            collections: HomeScreen::default(),
            environments: EnvironmentsScreen::default(),
            settings,
            config,
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
            Message::Collections(msg) => {
                let task = self.collections.update(msg);
                task.map(Message::Collections)
            }
            Message::Environments(msg) => {
                self.environments.update(msg);
                Task::none()
            }
            Message::Settings(msg) => {
                self.settings.update(msg);
                self.config.set_theme(&self.settings.theme);
                self.config.save();
                Task::none()
            }
        }
    }

    pub fn theme(&self) -> iced::Theme {
        self.config.theme()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let nav_items = vec![
            ActivityBarItem {
                route: Route::Collections,
                icon: "📦",
                label: "Collections",
            },
            ActivityBarItem {
                route: Route::Environments,
                icon: "🌍",
                label: "Environments",
            },
            ActivityBarItem {
                route: Route::Settings,
                icon: "⚙",
                label: "Settings",
            },
        ];

        let content: Element<'_, Message> = match self.route {
            Route::Collections => self.collections.view().map(Message::Collections),
            Route::Environments => self.environments.view().map(Message::Environments),
            Route::Settings => self.settings.view().map(Message::Settings),
        };

        column![
            row![
                activity_bar::activity_bar(nav_items, &self.route, Message::Navigate),
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(0),
            ]
            .height(Length::Fill),
            status_bar::status_bar(&self.status_message),
        ]
        .into()
    }
}
