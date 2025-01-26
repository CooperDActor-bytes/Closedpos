use iced::{
    button, text_input, Button, Column, Container, Element, Length, Sandbox, Text, TextInput,
};

pub struct SearchOrders<'a> {
    order_id_input: text_input::State,
    employee_name_input: text_input::State,
    search_button: button::State,
    order_id: String,
    employee_name: String,
}

impl<'a> Sandbox for SearchOrders<'a> {
    type Message = Message;

    fn new() -> Self {
        SearchOrders {
            order_id_input: text_input::State::new(),
            employee_name_input: text_input::State::new(),
            search_button: button::State::new(),
            order_id: String::new(),
            employee_name: String::new(),
        }
    }

    fn title(&self) -> String {
        "Search Orders".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OrderIdChanged(value) => {
                self.order_id = value;
            }
            Message::EmployeeNameChanged(value) => {
                self.employee_name = value;
            }
            Message::SearchButtonPressed => {
                println!(
                    "Searching for Order ID: {}, Employee Name: {}",
                    self.order_id, self.employee_name
                );
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let content = Column::new()
            .push(Text::new("Search Orders").size(30))
            .push(TextInput::new(
                &self.order_id_input,
                "Order ID...",
                &self.order_id,
                Message::OrderIdChanged,
            ))
            .push(TextInput::new(
                &self.employee_name_input,
                "Employee Name...",
                &self.employee_name,
                Message::EmployeeNameChanged,
            ))
            .push(
                Button::new(&self.search_button, Text::new("Search"))
                    .on_press(Message::SearchButtonPressed),
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
    OrderIdChanged(String),
    EmployeeNameChanged(String),
    SearchButtonPressed,
}