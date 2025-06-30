#[derive(Clone, Debug, Default)]
pub enum MockupPhoneColor {
    #[default]
    Default,
    Black,
}

impl MockupPhoneColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            MockupPhoneColor::Default => "",
            MockupPhoneColor::Black => "mockup-phone-black",
        }
    }
}
