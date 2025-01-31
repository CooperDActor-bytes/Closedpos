use iced::{Column, Element, Sandbox, Text};

pub struct Settings;

impl Sandbox for Settings {
    type Message = ();

    fn new() -> Self {
        Settings
    }

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("Settings"))
            .into()
    }
}