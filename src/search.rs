use iced::{Element, Sandbox, widget::{button, column, container, text_input, text}};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
struct Order {
    id: String,
    item: String,
    quantity: u32,
}

#[derive(Default)]
struct SearchOrders {
    search_id: String,
    result: Option<Order>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchIdChanged(String),
    Search,
}

impl Sandbox for SearchOrders {
    type Message = Message;

    fn new() -> Self {
        SearchOrders::default()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SearchIdChanged(id) => self.search_id = id,
            Message::Search => {
                self.result = find_order(&self.search_id);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text_input("Search Order ID", &self.search_id)
                .on_input(Message::SearchIdChanged),
            button("Search").on_press(Message::Search),
            match &self.result {
                Some(order) => text(format!(
                    "Order found: {} - {}x {}",
                    order.id, order.quantity, order.item
                )),
                None => text("No results found."),            }
        ];
        container(content).into()
    }
}

fn find_order(search_id: &str) -> Option<Order> {
    let file_path = "data/orders.json";
    let contents = fs::read_to_string(file_path).ok()?;
    let orders: Vec<Order> = serde_json::from_str(&contents).ok()?;
    orders.into_iter().find(|order| order.id == search_id)
}