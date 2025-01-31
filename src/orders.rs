use iced::{Column, Element, Sandbox, Text};

pub struct Orders;

impl Sandbox for Orders {
    type Message = ();

    fn new() -> Self {
        Orders
    }

    fn title(&self) -> String {
        String::from("Orders")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("Order Management"))
            .into()
    }
}