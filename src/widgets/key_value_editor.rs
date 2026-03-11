use iced::widget::{button, column, row, text, text_input};
use iced::{Element, Length};

pub struct KeyValueEditor {
    pub entries: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateKey(usize, String),
    UpdateValue(usize, String),
    Add,
    Remove(usize),
}

impl Default for KeyValueEditor {
    fn default() -> Self {
        Self {
            entries: vec![(String::new(), String::new())],
        }
    }
}

impl KeyValueEditor {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateKey(idx, key) => {
                if let Some(entry) = self.entries.get_mut(idx) {
                    entry.0 = key;
                }
            }
            Message::UpdateValue(idx, value) => {
                if let Some(entry) = self.entries.get_mut(idx) {
                    entry.1 = value;
                }
            }
            Message::Add => {
                self.entries.push((String::new(), String::new()));
            }
            Message::Remove(idx) => {
                if self.entries.len() > 1 {
                    self.entries.remove(idx);
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let rows: Vec<Element<'_, Message>> = self
            .entries
            .iter()
            .enumerate()
            .map(|(idx, (key, value))| {
                row![
                    text_input("Key", key)
                        .on_input(move |k| Message::UpdateKey(idx, k))
                        .width(Length::FillPortion(1)),
                    text_input("Value", value)
                        .on_input(move |v| Message::UpdateValue(idx, v))
                        .width(Length::FillPortion(1)),
                    button(text("✕"))
                        .on_press(Message::Remove(idx))
                        .style(button::danger),
                ]
                .spacing(5)
                .into()
            })
            .collect();

        let mut col = column(rows).spacing(5);
        col = col.push(
            button("+ Add")
                .on_press(Message::Add)
                .style(button::secondary),
        );

        col.into()
    }

    pub fn to_pairs(&self) -> Vec<(String, String)> {
        self.entries
            .iter()
            .filter(|(k, _)| !k.is_empty())
            .cloned()
            .collect()
    }
}
