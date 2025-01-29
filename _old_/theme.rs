pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn header_color(&self) -> iced::Color {
        match self {
            Theme::Light => iced::Color::BLACK,
            Theme::Dark => iced::Color::WHITE,
        }
    }
}