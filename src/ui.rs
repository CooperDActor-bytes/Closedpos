use iced::widget::{Button, Column, Container, Text};
use iced::{Element, Command};

#[derive(Debug, Clone)]
pub enum Message {
    Orders,
    Search,
    Inventory,
    Reports,
    Settings,
}

pub struct MainMenu {
    orders_button: iced::widget::button::State,
    search_button: iced::widget::button::State,
    inventory_button: iced::widget::button::State,
    reports_button: iced::widget::button::State,
    settings_button: iced::widget::button::State,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            orders_button: iced::widget::button::State::new(),
            search_button: iced::widget::button::State::new(),
            inventory_button: iced::widget::button::State::new(),
            reports_button: iced::widget::button::State::new(),
            settings_button: iced::widget::button::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Orders => println!("Navigating to Orders"),
            Message::Search => println!("Navigating to Search"),
            Message::Inventory => println!("Navigating to Inventory"),
            Message::Reports => println!("Navigating to Reports"),
            Message::Settings => println!("Navigating to Settings"),
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        Column::new()
            .push(Button::new(&self.orders_button, Text::new("Orders")).on_press(Message::Orders))
            .push(Button::new(&self.search_button, Text::new("Search")).on_press(Message::Search))
            .push(Button::new(&self.inventory_button, Text::new("Inventory")).on_press(Message::Inventory))
            .push(Button::new(&self.reports_button, Text::new("Reports")).on_press(Message::Reports))
            .push(Button::new(&self.settings_button, Text::new("Settings")).on_press(Message::Settings))
            .into()
    }
}