use iced::{Button, Column, Text, TextInput, Sandbox, Element, Length};

pub struct AccessControl {
    password_input: TextInput<String>,
    login_button: Button,
    status_message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Authenticate(String),
    SetPassword(String),
}

impl Sandbox for AccessControl {
    type Message = Message;

    fn new() -> Self {
        Self {
            password_input: TextInput::new("", "Enter password", "", Message::SetPassword),
            login_button: Button::new(Text::new("Login")).on_press(Message::Authenticate("".to_string())),
            status_message: "Please enter your password.".to_string(),
        }
    }

    fn title(&self) -> String {
        "Access Control".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Authenticate(password) => {
                if password == "Base64EncodedPasswordHere" {
                    self.status_message = "Access Granted!".to_string();
                } else {
                    self.status_message = "Invalid password!".to_string();
                }
            }
            Message::SetPassword(password) => {
                self.status_message = password;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .spacing(10)
            .push(Text::new(&self.status_message))
            .push(self.password_input.view())
            .push(self.login_button)
            .into()
    }
}