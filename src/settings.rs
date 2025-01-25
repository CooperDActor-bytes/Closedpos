use iced::{button, text_input, Button, Column, Container, Element, Length, Sandbox, Text, TextInput};

pub struct Settings<'a> {
    tax_rate_input: TextInput<'a, String>,
    save_button: button::State,
}

impl<'a> Sandbox for Settings<'a> {
    type Message = ();

    fn new() -> Self {
        Settings {
            tax_rate_input: TextInput::new("Tax Rate"),
            save_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        let content = Column::new()
            .push(Text::new("Settings"))
            .push(TextInput::new("Tax Rate"))
            .push(Button::new(&mut self.save_button, Text::new("Save")));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}