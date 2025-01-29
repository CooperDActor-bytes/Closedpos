use iced::{Element, Sandbox, Settings, widget::{button, column, container, text_input, text}};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: String,
    item: String,
    quantity: u32,
}

#[derive(Default)]
struct Orders {
    order_id: String,
    item_name: String,
    quantity: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    OrderIdChanged(String),
    ItemChanged(String),
    QuantityChanged(String),
    SubmitOrder,
}

impl Sandbox for Orders {
    type Message = Message;

    fn new() -> Self {
        Orders::default()
    }

    fn title(&self) -> String {
        "Orders".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OrderIdChanged(id) => self.order_id = id,
            Message::ItemChanged(item) => self.item_name = item,
            Message::QuantityChanged(qty) => self.quantity = qty,
            Message::SubmitOrder => {
                let order = Order {
                    id: self.order_id.clone(),
                    item: self.item_name.clone(),
                    quantity: self.quantity.parse().unwrap_or(1),
                };
                save_order(order);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text_input("Order ID", &self.order_id)
                .on_input(Message::OrderIdChanged),
            text_input("Item", &self.item_name)
                .on_input(Message::ItemChanged),
            text_input("Quantity", &self.quantity)
                .on_input(Message::QuantityChanged),
            button("Submit Order").on_press(Message::SubmitOrder),
        ];
        container(content).into()
    }
}

fn save_order(order: Order) {
    let file_path = "data/orders.json";
    let mut orders: Vec<Order> = match fs::read_to_string(file_path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => vec![],
    };
    orders.push(order);
    fs::write(file_path, serde_json::to_string(&orders).unwrap()).unwrap();
}