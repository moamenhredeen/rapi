use iced::widget::{button, row, text};
use iced::{border, Element, Length};

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
            button(text(label).size(13).center().width(Length::Fill))
                .width(Length::FillPortion(1))
                .padding([6, 12])
                .style(move |theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();
                    if is_active {
                        button::Style {
                            background: Some(palette.primary.base.color.into()),
                            text_color: palette.primary.base.text,
                            border: border::rounded(6),
                            ..button::primary(theme, status)
                        }
                    } else {
                        button::Style {
                            background: None,
                            text_color: palette.background.base.text,
                            border: border::rounded(6),
                            ..button::Style::default()
                        }
                    }
                })
                .on_press(on_select(value))
                .into()
        })
        .collect();

    row(buttons).spacing(4).into()
}
