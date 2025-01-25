use iced::{
    button, text_input, Button, Column, Container, Element, Length, Sandbox, Settings, Text, TextInput,
};

pub struct Orders<'a> {
    item_picker: iced::widget::pick_list::State<String>,
    quantity_input: TextInput<'a, String>,
    add_button: button::State,
}

impl<'a> Sandbox for Orders<'a> {
    type Message = ();

    fn new() -> Self {
        Orders {
            item_picker: iced::widget::pick_list::State::default(),
            quantity_input: TextInput::new("Quantity"),
            add_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Orders")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        let content = Column::new()
            .push(Text::new("Orders"))
            .push(TextInput::new("Item Picker"))
            .push(TextInput::new("Quantity Input"))
            .push(Button::new(&mut self.add_button, Text::new("Add")));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}