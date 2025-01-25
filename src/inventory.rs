use iced::{button, text_input, Button, Column, Container, Element, Length, Sandbox, Text, TextInput};

pub struct Inventory<'a> {
    item_name_input: TextInput<'a, String>,
    stock_input: TextInput<'a, String>,
    save_button: button::State,
}

impl<'a> Sandbox for Inventory<'a> {
    type Message = ();

    fn new() -> Self {
        Inventory {
            item_name_input: TextInput::new("Item Name"),
            stock_input: TextInput::new("Stock"),
            save_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Inventory")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        let content = Column::new()
            .push(Text::new("Inventory"))
            .push(TextInput::new("Item Name"))
            .push(TextInput::new("Stock"))
            .push(Button::new(&mut self.save_button, Text::new("Save")));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}