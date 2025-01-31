use iced::{Column, Element, Sandbox, Text};

pub struct Inventory;

impl Sandbox for Inventory {
    type Message = ();

    fn new() -> Self {
        Inventory
    }

    fn title(&self) -> String {
        String::from("Inventory")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("Inventory Management"))
            .into()
    }
}