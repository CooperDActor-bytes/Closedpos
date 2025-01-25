mod orders;
mod search;
mod inventory;
mod reports;
mod settings;

use iced::{Application, Settings};

fn main() -> iced::Result {
    RangerPOS::run(Settings::default())
}

struct RangerPOS;

impl Application for RangerPOS {
    type Executor = iced::executor::Default;
    type Message = ();
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (RangerPOS, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Ranger POS")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        iced::Text::new("Welcome to Ranger POS!").into()
    }
}