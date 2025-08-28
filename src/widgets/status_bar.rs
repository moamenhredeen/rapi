use iced::{Background, Element, Color, Length};
use iced::widget::{container, row, text};
use iced::widget::container::background;

pub fn status_bar<'a, Message: Clone + 'a>(
    message: impl text::IntoFragment<'a>,
) -> Element<'a, Message> {
    container(
        row![text("Message"), text(message)]
            .spacing(5)
            .padding(5)
    ).style(|_| background(Background::Color(Color::new(0.2, 0.0, 0.2, 1.0))))
        .width(Length::Fill)
        .into()
}
