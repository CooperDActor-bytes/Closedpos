use iced::{Element, Sandbox, Text};

pub struct Settings;

impl Sandbox for Settings {
    type Message = ();

    fn new() -> Self {
        Settings
    }

    fn title(&self) -> String {
        "Settings".to_string()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Text::new("This is the Settings View").into()
    }
}