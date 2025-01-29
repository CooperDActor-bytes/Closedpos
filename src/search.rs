use iced::{Element, Sandbox, Text};

pub struct SearchOrders;

impl Sandbox for SearchOrders {
    type Message = ();

    fn new() -> Self {
        SearchOrders
    }

    fn title(&self) -> String {
        "Search Orders".to_string()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Text::new("This is the Search Orders View").into()
    }
}