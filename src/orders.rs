use iced::{
    button, text_input, Button, Column, Container, Element, Length, Sandbox, Text, TextInput,
};

pub struct Orders<'a> {
    item_picker: iced::widget::pick_list::State<String>,
    quantity_input: text_input::State,
    add_button: button::State,
    quantity: String,
}

impl<'a> Sandbox for Orders<'a> {
    type Message = Message;

    fn new() -> Self {
        Orders {
            item_picker: iced::widget::pick_list::State::default(),
            quantity_input: text_input::State::new(),
            add_button: button::State::new(),
            quantity: String::new(),
        }
    }

    fn title(&self) -> String {
        "Orders".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::QuantityChanged(value) => {
                self.quantity = value;
            }
            Message::AddButtonPressed => {
                println!("Adding item with quantity: {}", self.quantity);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let content = Column::new()
            .push(Text::new("Orders").size(30))
            .push(TextInput::new(
                &self.quantity_input,
                "Enter quantity...",
                &self.quantity,
                Message::QuantityChanged,
            ))
            .push(
                Button::new(&self.add_button, Text::new("Add"))
                    .on_press(Message::AddButtonPressed),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    QuantityChanged(String),
    AddButtonPressed,
}