use iced::widget::{button, row};
use iced::{Element, Length};

pub fn tab_bar<'a, T, M>(
    tabs: Vec<(T, &'a str)>,
    selected: &T,
    on_select: impl Fn(T) -> M + 'a,
) -> Element<'a, M>
where
    T: Clone + PartialEq + 'a,
    M: Clone + 'a,
{
    let buttons: Vec<Element<'a, M>> = tabs
        .into_iter()
        .map(|(value, label)| {
            let is_active = &value == selected;
            let style = if is_active {
                button::primary
            } else {
                button::text
            };
            let val = value;
            button(label)
                .width(Length::FillPortion(1))
                .style(style)
                .on_press(on_select(val))
                .into()
        })
        .collect();

    row(buttons).into()
}
