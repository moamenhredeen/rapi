use iced::widget::{button, column, container, tooltip, Text};
use iced::{border, Center, Element, Length};

pub struct ActivityBarItem<Route> {
    pub route: Route,
    pub icon: fn() -> Text<'static>,
    pub label: &'static str,
}

pub fn activity_bar<'a, Route, M>(
    items: Vec<ActivityBarItem<Route>>,
    active: &Route,
    on_select: impl Fn(Route) -> M + 'a,
) -> Element<'a, M>
where
    Route: Clone + PartialEq + 'a,
    M: Clone + 'static,
{
    let buttons: Vec<Element<'a, M>> = items
        .into_iter()
        .map(|item| {
            let is_active = &item.route == active;
            let route = item.route;

            tooltip(
                button(
                    (item.icon)()
                        .size(20)
                        .center()
                        .width(Length::Fill),
                )
                .width(44)
                .height(44)
                .on_press(on_select(route))
                .style(move |theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();
                    if is_active {
                        button::Style {
                            background: Some(palette.background.weak.color.into()),
                            text_color: palette.primary.strong.color,
                            border: border::rounded(8),
                            ..button::primary(theme, status)
                        }
                    } else {
                        button::Style {
                            background: None,
                            text_color: palette.background.base.text,
                            border: border::rounded(8),
                            ..button::Style::default()
                        }
                    }
                }),
                item.label,
                tooltip::Position::Right,
            )
            .gap(8)
            .style(|theme: &iced::Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(palette.background.weak.color.into()),
                    border: border::rounded(4),
                    text_color: Some(palette.background.base.text),
                    ..container::Style::default()
                }
            })
            .padding(6)
            .into()
        })
        .collect();

    container(
        column(buttons)
            .spacing(4)
            .align_x(Center)
            .padding([12, 4]),
    )
    .width(52)
    .height(Length::Fill)
    .style(|theme: &iced::Theme| {
        let palette = theme.extended_palette();
        container::background(palette.background.strong.color)
    })
    .into()
}
