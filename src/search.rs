use iced::{
    button, Column, Container, Element, Length, Row, Text, TextInput, Button, Align,
};
use iced::widget::pick_list;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct SearchOrders {
    order_id_input: TextInput<String>,
    employee_name_input: TextInput<String>,
    search_button: button::State,
    search_results: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Search,
    SetOrderId(String),
    SetEmployeeName(String),
}

impl Sandbox for SearchOrders {
    type Message = Message;

    fn new() -> Self {
        Self {
            search_results: vec![],
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Search Orders")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Search => {
                // Perform search based on input fields (order ID or employee name)
                self.search_results.push(String::from("Order #123 - Completed"));
                self.search_results.push(String::from("Order #456 - Completed"));
            }
            Message::SetOrderId(order_id) => {
                // Update order ID input field
            }
            Message::SetEmployeeName(employee_name) => {
                // Update employee name input field
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let order_id_input = TextInput::new(
            &mut self.order_id_input,
            "Enter Order ID",
            "",
            Message::SetOrderId,
        )
        .padding(10)
        .size(30);

        let employee_name_input = TextInput::new(
            &mut self.employee_name_input,
            "Enter Employee Name",
            "",
            Message::SetEmployeeName,
        )
        .padding(10)
        .size(30);

        let search_button = Button::new(
            &mut self.search_button,
            Text::new("Search"),
        )
        .on_press(Message::Search)
        .padding(15);

        let result_list = Column::new()
            .spacing(10)
            .push(Text::new("Search Results"))
            .push(Text::new(self.search_results.join("\n")));

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(Text::new("Search Orders"))
            .push(order_id_input)
            .push(employee_name_input)
            .push(search_button)
            .push(result_list);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}