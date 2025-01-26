use iced::{
    button, text_input, Button, Column, Container, Element, Length, Sandbox, Text, TextInput,
};

pub struct Inventory<'a> {
    item_name_input: text_input::State,
    stock_input: text_input::State,
    save_button: button::State,
    item_name: String,
    stock: String,
}

impl<'a> Sandbox for Inventory<'a> {
    type Message = Message;

    fn new() -> Self {
        Inventory {
            item_name_input: text_input::State::new(),
            stock_input: text_input::State::new(),
            save_button: button::State::new(),
            item_name: String::new(),
            stock: String::new(),
        }
    }

    fn title(&self) -> String {
        "Inventory".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ItemNameChanged(value) => {
                self.item_name = value;
            }
            Message::StockChanged(value) => {
                self.stock = value;
            }
            Message::SaveButtonPressed => {
                println!(
                    "Saving item: {}, Stock: {}",
                    self.item_name, self.stock
                );
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let content = Column::new()
            .push(Text::new("Inventory").size(30))
            .push(TextInput::new(
                &self.item_name_input,
                "Item Name...",
                &self.item_name,
                Message::ItemNameChanged,
            ))
            .push(TextInput::new(
                &self.stock_input,
                "Stock...",
                &self.stock,
                Message::StockChanged,
            ))
            .push(
                Button::new(&self.save_button, Text::new("Save"))
                    .on_press(Message::SaveButtonPressed),
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
    ItemNameChanged(String),
    StockChanged(String),
    SaveButtonPressed,
}