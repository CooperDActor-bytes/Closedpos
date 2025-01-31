use iced::widget::{Button, Column, Text};
use iced::{Element, Sandbox, Settings};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Orders,
    Search,
    Inventory,
    Reports,
    Settings,
}

pub struct UI;

impl Sandbox for UI {
    type Message = Message;

    fn new() -> Self {
        UI
    }

    fn title(&self) -> String {
        String::from("RangerPOS")
    }

    fn update(&mut self, _message: Message) {
        // Handle button clicks here
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(Button::new(Text::new("Orders")).on_press(Message::Orders))
            .push(Button::new(Text::new("Search")).on_press(Message::Search))
            .push(Button::new(Text::new("Inventory")).on_press(Message::Inventory))
            .push(Button::new(Text::new("Reports")).on_press(Message::Reports))
            .push(Button::new(Text::new("Settings")).on_press(Message::Settings))
            .into()
    }
}

fn main() {
    UI::run(Settings::default());
}