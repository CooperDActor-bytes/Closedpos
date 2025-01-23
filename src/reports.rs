use iced::{
    button, Column, Container, Element, Length, Text, Button, Align,
};

#[derive(Default)]
pub struct Reports {
    daily_button: button::State,
    weekly_button: button::State,
    monthly_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    GenerateDailyReport,
    GenerateWeeklyReport,
    GenerateMonthlyReport,
}

impl Sandbox for Reports {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Sales Reports")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GenerateDailyReport => {
                // Generate daily report
            }
            Message::GenerateWeeklyReport => {
                // Generate weekly report
            }
            Message::GenerateMonthlyReport => {
                // Generate monthly report
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let daily_button = Button::new(
            &mut self.daily_button,
            Text::new("Daily Report"),
        )
        .on_press(Message::GenerateDailyReport)
        .padding(15);

        let weekly_button = Button::new(
            &mut self.weekly_button,
            Text::new("Weekly Report"),
        )
        .on_press(Message::GenerateWeeklyReport)
        .padding(15);

        let monthly_button = Button::new(
            &mut self.monthly_button,
            Text::new("Monthly Report"),
        )
        .on_press(Message::GenerateMonthlyReport)
        .padding(15);

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(Text::new("Sales Reports"))
            .push(daily_button)
            .push(weekly_button)
            .push(monthly_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}