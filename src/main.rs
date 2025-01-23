use iced::{
    button, Button, Column, Container, Element, Length, Sandbox, Settings, Text,
};
use iced::theme::{Theme, ThemeMessage};

mod orders;
mod search;
mod inventory;
mod reports;
mod settings;
mod theme;

#[derive(Default)]
struct POSApp {
    theme: Theme,                         // Current theme (light/dark)
    orders_button: button::State,         // Button state for "Take Orders"
    search_button: button::State,         // Button state for "Search Orders"
    inventory_button: button::State,      // Button state for "Inventory"
    reports_button: button::State,        // Button state for "Reports"
    settings_button: button::State,       // Button state for "Settings"
}

#[derive(Debug, Clone)]
enum Message {
    NavigateTo(&'static str),             // Navigation message
    ToggleTheme,                          // Toggle light/dark theme
}

impl Sandbox for POSApp {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("POS System")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(screen) => {
                // Navigate to the selected screen (To be implemented)
                println!("Navigating to {}", screen);
            }
            Message::ToggleTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                };
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let header = Text::new("Point of Sale System")
            .size(30)
            .color(self.theme.header_color());

        let navigation = Column::new()
            .spacing(20)
            .push(
                Button::new(&mut self.orders_button, Text::new("Take Orders"))
                    .on_press(Message::NavigateTo("orders")),
            )
            .push(
                Button::new(&mut self.search_button, Text::new("Search Orders"))
                    .on_press(Message::NavigateTo("search")),
            )
            .push(
                Button::new(&mut self.inventory_button, Text::new("Inventory"))
                    .on_press(Message::NavigateTo("inventory")),
            )
            .push(
                Button::new(&mut self.reports_button, Text::new("Reports"))
                    .on_press(Message::NavigateTo("reports")),
            )
            .push(
                Button::new(&mut self.settings_button, Text::new("Settings"))
                    .on_press(Message::NavigateTo("settings")),
            );

        let theme_toggle = Button::new(
            &mut self.settings_button,
            Text::new(match self.theme {
                Theme::Light => "Switch to Dark Mode",
                Theme::Dark => "Switch to Light Mode",
            }),
        )
        .on_press(Message::ToggleTheme);

        Container::new(
            Column::new()
                .spacing(30)
                .push(header)
                .push(navigation)
                .push(theme_toggle),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .center_x()
        .center_y()
        .into()
    }
}