/// # Accordion Style Modifiers
///
/// Style enum for daisyUI accordion modifier classes that control the visual appearance
/// and interaction indicators for accordion/collapse components.
#[derive(Clone, Debug, Default)]
pub enum AccordionModifier {
    /// No visual indicator (default)
    #[default]
    Default,

    /// Shows a rotating arrow indicator on the right side
    Arrow,

    /// Shows a plus/minus toggle indicator on the right side
    Plus,
}

impl AccordionModifier {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            AccordionModifier::Default => "",
            AccordionModifier::Arrow => "collapse-arrow",
            AccordionModifier::Plus => "collapse-plus",
        }
    }
}
