use std::fmt::Debug;

pub trait Screen<M: Debug + Clone> : Default {
    fn update(&mut self, message: M) -> iced::Task<M>;
    fn view(&self) -> iced::Element<'_, M>;
}
