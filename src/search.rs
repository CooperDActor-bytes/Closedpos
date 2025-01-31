use iced::widget::{Column, Text};
use iced::{Element, Sandbox};

pub struct Search;

impl Sandbox for Search {
    type Message = ();

    fn new() -> Self {
        Search
    }

    fn title(&self) -> String {
        String::from("Search")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("Search Screen"))
            .into()
    }
}