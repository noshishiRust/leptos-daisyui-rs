#[derive(Clone, Debug, Default)]
pub enum AccordionModifier {
    #[default]
    Default,
    Arrow,
    Plus,
}

impl AccordionModifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccordionModifier::Default => "",
            AccordionModifier::Arrow => "collapse-arrow",
            AccordionModifier::Plus => "collapse-plus",
        }
    }
}
