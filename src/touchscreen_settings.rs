use iced::{
    button, Color, Column, Container, Element, Length, Row, Text, Button, Sandbox, TextInput,
    Alignment, Point,
};

#[derive(Default)]
pub struct TouchscreenSettings {
    button_position: Point,
    button_color: Color,
    background_color: Color,
    save_button: button::State,
    change_button_position: button::State,
    change_color_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeButtonPosition,
    ChangeButtonColor,
    SaveSettings,
}

impl Sandbox for TouchscreenSettings {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Touchscreen Settings")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeButtonPosition => {
                // Change the button's position on the screen (relative positioning)
                self.button_position = Point::new(200.0, 300.0); // Example of repositioning
            }
            Message::ChangeButtonColor => {
                // Change the button's color dynamically
                self.button_color = Color::from_rgb(0.0, 1.0, 0.0); // Green color
            }
            Message::SaveSettings => {
                // Save settings (e.g., save button position and colors to JSON file)
                println!("Settings saved.");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let change_button_position = Button::new(
            &mut self.change_button_position,
            Text::new("Change Position"),
        )
        .on_press(Message::ChangeButtonPosition)
        .padding(10)
        .style(iced::button::Style {
            background: Some(self.button_color),
            ..Default::default()
        })
        .position(self.button_position);

        let change_color_button = Button::new(
            &mut self.change_color_button,
            Text::new("Change Color"),
        )
        .on_press(Message::ChangeButtonColor)
        .padding(10);

        let save_button = Button::new(
            &mut self.save_button,
            Text::new("Save Settings"),
        )
        .on_press(Message::SaveSettings)
        .padding(15);

        Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(change_button_position)
            .push(change_color_button)
            .push(save_button)
            .into()
    }
}