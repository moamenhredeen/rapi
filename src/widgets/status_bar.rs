use iced::widget::{container, row, text};
use iced::{Element, Length};

pub fn status_bar<'a, Message: Clone + 'a>(
    message: impl text::IntoFragment<'a>,
) -> Element<'a, Message> {
    container(
        row![text("Message"), text(message)]
            .spacing(5)
            .padding(5),
    )
    .style(|theme: &iced::Theme| {
        let palette = theme.extended_palette();
        container::background(palette.background.strong.color)
    })
    .width(Length::Fill)
    .into()
}
