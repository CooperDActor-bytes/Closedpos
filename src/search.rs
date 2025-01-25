use iced::{button, text_input, Button, Column, Container, Element, Length, Sandbox, Text, TextInput};

pub struct SearchOrders<'a> {
    order_id_input: TextInput<'a, String>,
    employee_name_input: TextInput<'a, String>,
    search_button: button::State,
}

impl<'a> Sandbox for SearchOrders<'a> {
    type Message = ();

    fn new() -> Self {
        SearchOrders {
            order_id_input: TextInput::new("Order ID"),
            employee_name_input: TextInput::new("Employee Name"),
            search_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Search Orders")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        let content = Column::new()
            .push(Text::new("Search Orders"))
            .push(TextInput::new("Order ID"))
            .push(TextInput::new("Employee Name"))
            .push(Button::new(&mut self.search_button, Text::new("Search")));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}