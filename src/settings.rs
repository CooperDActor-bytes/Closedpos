use iced::{Element, Sandbox, widget::{button, column, container, text_input, text}};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SettingsData {
    tax_rate: f32,
}

#[derive(Default)]
struct Settings {
    tax_rate: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    TaxRateChanged(String),
    SaveSettings,
}

impl Sandbox for Settings {
    type Message = Message;

    fn new() -> Self {
        Settings::default()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TaxRateChanged(rate) => self.tax_rate = rate,
            Message::SaveSettings => {
                let settings = SettingsData {
                    tax_rate: self.tax_rate.parse().unwrap_or(0.0),
                };
                save_settings(settings);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text_input("Tax Rate (%)", &self.tax_rate)
                .on_input(Message::TaxRateChanged),
            button("Save").on_press(Message::SaveSettings),
        ];
        container(content).into()
    }
}

fn save_settings(settings: SettingsData) {
    let file_path = "data/settings.json";
    fs::write(file_path, serde_json::to_string(&settings).unwrap()).unwrap();
}