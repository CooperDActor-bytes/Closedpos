use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

#[derive(Default)]
pub struct MainMenu {
    take_orders_button: button::State,
    order_search_button: button::State,
    inventory_button: button::State,
    reports_button: button::State,
    settings_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    TakeOrders,
    OrderSearch,
    ManageInventory,
    ViewReports,
    OpenSettings,
}

impl Sandbox for MainMenu {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Ice Cream POS Main Menu")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TakeOrders => println!("Navigating to 'Take Orders'..."),
            Message::OrderSearch => println!("Navigating to 'Order Search'..."),
            Message::ManageInventory => println!("Navigating to 'Inventory Management'..."),
            Message::ViewReports => println!("Navigating to 'Sales Reports'..."),
            Message::OpenSettings => println!("Navigating to 'Settings'..."),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let take_orders_button = Button::new(&mut self.take_orders_button, Text::new("Take Orders"))
            .on_press(Message::TakeOrders);

        let order_search_button = Button::new(&mut self.order_search_button, Text::new("Order Search"))
            .on_press(Message::OrderSearch);

        let inventory_button = Button::new(&mut self.inventory_button, Text::new("Manage Inventory"))
            .on_press(Message::ManageInventory);

        let reports_button = Button::new(&mut self.reports_button, Text::new("View Reports"))
            .on_press(Message::ViewReports);

        let settings_button = Button::new(&mut self.settings_button, Text::new("Settings"))
            .on_press(Message::OpenSettings);

        Column::new()
            .spacing(20)
            .push(Text::new("Ice Cream POS Main Menu").size(40))
            .push(take_orders_button)
            .push(order_search_button)
            .push(inventory_button)
            .push(reports_button)
            .push(settings_button)
            .into()
    }
}

pub fn run_menu() {
    MainMenu::run(Settings::default()).unwrap();
}