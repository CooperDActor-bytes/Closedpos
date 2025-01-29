use iced::{Element, Sandbox, Text};

pub struct Reports;

impl Sandbox for Reports {
    type Message = ();

    fn new() -> Self {
        Reports
    }

    fn title(&self) -> String {
        "Reports".to_string()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Text::new("This is the Reports View").into()
    }
}