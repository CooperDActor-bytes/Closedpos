mod orders;
mod search;
mod inventory;
mod reports;
mod settings;

use iced::{Application, Command, Element, Settings, Theme, Text};

fn main() -> iced::Result {
    RangerPOS::run(Settings::default())
}

struct RangerPOS;

impl Application for RangerPOS {
    type Executor = iced::executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (RangerPOS, Command::none())
    }

    fn title(&self) -> String {
        "Ranger POS".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Text::new("Welcome to Ranger POS!").size(40).into()
    }
}