use iced::{Application, Command, Element, Settings};
use crate::ui::{MainMenu, Message};

mod ui;
mod orders;
mod search;
mod inventory;
mod reports;
mod settings;

pub struct RangerPOS {
    menu: MainMenu,
}

impl Application for RangerPOS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (RangerPOS { menu: MainMenu::new() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("RangerPOS")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.menu.update(message)
    }

    fn view(&self) -> Element<Self::Message> {
        self.menu.view()
    }
}

fn main() -> iced::Result {
    RangerPOS::run(Settings::default())
}