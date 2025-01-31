mod orders;
mod search;
mod inventory;
mod reports;
mod settings;
mod ui;

use iced::{Sandbox, Settings as IcedSettings, Application, Command, Element, Theme};
use ui::MainMenu;

fn main() -> iced::Result {
    RangerPOS::run(IcedSettings::default())
}

struct RangerPOS {
    menu: MainMenu,
}

impl Application for RangerPOS {
    type Executor = iced::executor::Default;
    type Message = ui::Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self { menu: MainMenu::new() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Ranger POS")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.menu.update(message)
    }

    fn view(&self) -> Element<Self::Message> {
        self.menu.view()
    }
}