use iced::{
    button, Column, Container, Element, Length, Text, TextInput, Button, Align,
};

#[derive(Default)]
pub struct Settings {
    tax_rate_input: TextInput<String>,
    save_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetTaxRate(String),
    SaveSettings,
}

impl Sandbox for Settings {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SetTaxRate(rate) => {
                // Set the tax rate
            }
            Message::SaveSettings => {
                // Save settings
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let tax_rate_input = TextInput::new(
            &mut self.tax_rate_input,
            "Tax Rate (%)",
            "",
            Message::SetTaxRate,
        )
        .padding(10)
        .size(30);

        let save_button = Button::new(
            &mut self.save_button,
            Text::new("Save Settings"),
        )
        .on_press(Message::SaveSettings)
        .padding(15);

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(Text::new("Settings"))
            .push(tax_rate_input)
            .push(save_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}