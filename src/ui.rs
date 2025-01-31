use iced::{button, Button, Column, Container, Element, Length, Sandbox, Text};

#[derive(Debug, Clone)]
pub enum Message {
    Orders,
    Search,
    Inventory,
    Reports,
    Settings,
}

pub struct MainMenu {
    orders_button: button::State,
    search_button: button::State,
    inventory_button: button::State,
    reports_button: button::State,
    settings_button: button::State,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            orders_button: button::State::new(),
            search_button: button::State::new(),
            inventory_button: button::State::new(),
            reports_button: button::State::new(),
            settings_button: button::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Orders => println!("Orders selected"),
            Message::Search => println!("Search selected"),
            Message::Inventory => println!("Inventory selected"),
            Message::Reports => println!("Reports selected"),
            Message::Settings => println!("Settings selected"),
        }
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