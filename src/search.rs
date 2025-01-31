use iced::{Column, Element, Sandbox, Text};

pub struct Search;

impl Sandbox for Search {
    type Message = ();

    fn new() -> Self {
        Search
    }

    fn title(&self) -> String {
        String::from("Search Orders")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("Search Orders"))
            .into()
    }
}