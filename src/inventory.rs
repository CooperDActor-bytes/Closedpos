use iced::{Element, Sandbox, widget::{button, column, container, text_input, text}};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InventoryItem {
    name: String,
    stock: u32,
}

#[derive(Default)]
struct Inventory {
    item_name: String,
    stock: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ItemNameChanged(String),
    StockChanged(String),
    SaveItem,
}

impl Sandbox for Inventory {
    type Message = Message;

    fn new() -> Self {
        Inventory::default()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ItemNameChanged(name) => self.item_name = name,
            Message::StockChanged(stock) => self.stock = stock,
            Message::SaveItem => {
                let item = InventoryItem {
                    name: self.item_name.clone(),
                    stock: self.stock.parse().unwrap_or(0),
                };
                save_inventory(item);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text_input("Item Name", &self.item_name)
                .on_input(Message::ItemNameChanged),
            text_input("Stock", &self.stock)
                .on_input(Message::StockChanged),
            button("Save").on_press(Message::SaveItem),
        ];
        container(content).into()
    }
}

fn save_inventory(item: InventoryItem) {
    let file_path = "data/inventory.json";
    let mut inventory: Vec<InventoryItem> = match fs::read_to_string(file_path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => vec![],
    };
    inventory.push(item);
    fs::write(file_path, serde_json::to_string(&inventory).unwrap()).unwrap();
}