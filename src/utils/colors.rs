pub enum Colors {
    Primary,
    Secondary,
    Font,
    Background,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "",
            Color::Secondary => "",
            Color::Font => "",
            Color::Background => "",
        }
    }
}
