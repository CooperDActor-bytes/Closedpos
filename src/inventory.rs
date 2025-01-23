use iced::{
    button, Column, Container, Element, Length, Row, Text, TextInput, Button, Align,
};
use iced::widget::pick_list;

#[derive(Default)]
pub struct Inventory {
    item_name_input: TextInput<String>,
    stock_input: TextInput<String>,
    add_item_button: button::State,
    update_stock_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddItem,
    UpdateStock,
    SetItemName(String),
    SetStock(String),
}

impl Sandbox for Inventory {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Inventory Management")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddItem => {
                // Add a new item to inventory
            }
            Message::UpdateStock => {
                // Update stock levels
            }
            Message::SetItemName(item_name) => {
                // Set item name
            }
            Message::SetStock(stock) => {
                // Set stock level
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let item_name_input = TextInput::new(
            &mut self.item_name_input,
            "Item Name",
            "",
            Message::SetItemName,
        )
        .padding(10)
        .size(30);

        let stock_input = TextInput::new(
            &mut self.stock_input,
            "Stock Quantity",
            "",
            Message::SetStock,
        )
        .padding(10)
        .size(30);

        let add_item_button = Button::new(
            &mut self.add_item_button,
            Text::new("Add Item"),
        )
        .on_press(Message::AddItem)
        .padding(15);

        let update_stock_button = Button::new(
            &mut self.update_stock_button,
            Text::new("Update Stock"),
        )
        .on_press(Message::UpdateStock)
        .padding(15);

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(Text::new("Inventory Management"))
            .push(item_name_input)
            .push(stock_input)
            .push(add_item_button)
            .push(update_stock_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}