use iced::{Element, Sandbox, widget::{button, column, container, text}};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
struct Order {
    id: String,
    item: String,
    quantity: u32,
}

#[derive(Default)]
struct Reports {
    report_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    GenerateReport,
}

impl Sandbox for Reports {
    type Message = Message;

    fn new() -> Self {
        Reports::default()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GenerateReport => {
                self.report_text = generate_report();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            button("Generate Report").on_press(Message::GenerateReport),
            text(&self.report_text)
        ];
        container(content).into()
    }
}

fn generate_report() -> String {
    let file_path = "data/orders.json";
    let contents = fs::read_to_string(file_path).ok()?;
    let orders: Vec<Order> = serde_json::from_str(&contents).ok()?;
    
    let total_orders = orders.len();
    let total_items: u32 = orders.iter().map(|o| o.quantity).sum();

    Some(format!("Total Orders: {}\nTotal Items Sold: {}", total_orders, total_items))
        .unwrap_or_else(|| "No data available".to_string())
}