use iced::{
    button, Column, Container, Element, Length, Row, Sandbox, Text, TextInput, Button,
    Align,
};
use iced::widget::pick_list;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Orders {
    item_picker: pick_list::State<String>,
    quantity_input: TextInput<String>,
    add_to_cart_button: button::State,
    order_button: button::State,
    items: Vec<String>,
    quantity: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddToCart,
    PlaceOrder,
    SetQuantity(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuItem {
    name: String,
    price: f32,
}

impl Sandbox for Orders {
    type Message = Message;

    fn new() -> Self {
        Self {
            items: vec!["Ice Cream", "Sundae", "Milkshake", "Cone", "Popsicle"].into_iter().map(|s| s.to_string()).collect(),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Take Orders")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddToCart => {
                // Add selected item to cart (you can implement cart logic here)
                println!("Item added to cart");
            }
            Message::PlaceOrder => {
                // Submit the order (e.g., save order to the database or process payment)
                println!("Order placed!");
            }
            Message::SetQuantity(quantity) => {
                self.quantity = quantity;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let item_picker = pick_list::PickList::new(
            &mut self.item_picker,
            &self.items,
            None,
            Message::AddToCart,
        );

        let quantity_input = TextInput::new(
            &mut self.quantity_input,
            "Enter Quantity",
            &self.quantity,
            Message::SetQuantity,
        )
        .padding(10)
        .size(30);

        let add_to_cart_button = Button::new(
            &mut self.add_to_cart_button,
            Text::new("Add to Cart"),
        )
        .on_press(Message::AddToCart)
        .padding(15);

        let order_button = Button::new(
            &mut self.order_button,
            Text::new("Place Order"),
        )
        .on_press(Message::PlaceOrder)
        .padding(15);

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(Text::new("Select Menu Item"))
            .push(item_picker)
            .push(quantity_input)
            .push(add_to_cart_button)
            .push(order_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}