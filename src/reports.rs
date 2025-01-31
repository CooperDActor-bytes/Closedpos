use iced::{Column, Element, Sandbox, Text};

pub struct Reports;

impl Sandbox for Reports {
    type Message = ();

    fn new() -> Self {
        Reports
    }

    fn title(&self) -> String {
        String::from("Reports")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("Reports & Analytics"))
            .into()
    }
}