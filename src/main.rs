use iced::{Application, Command, Element, Settings, Theme};
use iced::widget::{button, column, container, text};

mod orders;
mod search;
mod inventory;
mod reports;
mod settings;
mod utils;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenOrders,
    OpenSearch,
    OpenInventory,
    OpenReports,
    OpenSettings,
}

struct RangerPOS {
    theme: Theme,
}

impl Application for RangerPOS {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_: ()) -> (Self, Command<Message>) {
        (Self { theme: Theme::Light }, Command::none())
    }

    fn title(&self) -> String {
        "Ranger POS".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OpenOrders => println!("Orders opened"),
            Message::OpenSearch => println!("Search opened"),
            Message::OpenInventory => println!("Inventory opened"),
            Message::OpenReports => println!("Reports opened"),
            Message::OpenSettings => println!("Settings opened"),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            button("Orders").on_press(Message::OpenOrders),
            button("Search").on_press(Message::OpenSearch),
            button("Inventory").on_press(Message::OpenInventory),
            button("Reports").on_press(Message::OpenReports),
            button("Settings").on_press(Message::OpenSettings)
        ];
        container(content).center_x().center_y().into()
    }
}

fn main() -> iced::Result {
    RangerPOS::run(Settings::default())
}