mod reports;
mod settings;
mod search;
mod utils;

use iced::{
    button, Application, Button, Column, Command, Container, Element, Length, Sandbox, Settings,
    Text,
};

pub fn main() -> iced::Result {
    RangerPOS::run(Settings::default())
}

struct RangerPOS {
    current_view: View,
    reports_button: button::State,
    settings_button: button::State,
    search_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ShowReports,
    ShowSettings,
    ShowSearch,
}

#[derive(Debug, Clone)]
enum View {
    MainMenu,
    Reports,
    Settings,
    Search,
}

impl Application for RangerPOS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                current_view: View::MainMenu,
                reports_button: button::State::new(),
                settings_button: button::State::new(),
                search_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "RangerPOS".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ShowReports => self.current_view = View::Reports,
            Message::ShowSettings => self.current_view = View::Settings,
            Message::ShowSearch => self.current_view = View::Search,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        match self.current_view {
            View::MainMenu => self.main_menu(),
            View::Reports => reports::Reports::new().view(),
            View::Settings => settings::Settings::new().view(),
            View::Search => search::SearchOrders::new().view(),
        }
    }
}

impl RangerPOS {
    fn main_menu(&self) -> Element<Message> {
        let reports_button = Button::new(&self.reports_button, Text::new("Reports"))
            .on_press(Message::ShowReports);

        let settings_button = Button::new(&self.settings_button, Text::new("Settings"))
            .on_press(Message::ShowSettings);

        let search_button = Button::new(&self.search_button, Text::new("Search Orders"))
            .on_press(Message::ShowSearch);

        let content = Column::new()
            .padding(20)
            .spacing(10)
            .push(reports_button)
            .push(settings_button)
            .push(search_button);

        Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}