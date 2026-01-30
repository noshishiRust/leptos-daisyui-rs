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

/// # Accordion Input Type
///
/// Controls the type of input used for accordion toggle behavior:
/// - Radio: Only one accordion can be open at a time in a group (cannot be closed by clicking again)
/// - Checkbox: Accordion can be toggled open/closed independently (can be closed by clicking again)
#[derive(Clone, Debug, Default)]
pub enum AccordionInputType {
    /// Radio button - only one can be open at a time, cannot be closed by clicking again
    #[default]
    Radio,

    /// Checkbox - can be toggled open/closed, allows multiple open accordions
    Checkbox,
}

impl AccordionInputType {
    /// Returns the HTML input type string
    pub fn as_str(&self) -> &'static str {
        match self {
            AccordionInputType::Radio => "radio",
            AccordionInputType::Checkbox => "checkbox",
        }
    }
}

/// # Accordion Open force Modifiers
#[derive(Clone, Debug, Default)]
pub enum AccordionForceModifier {
    /// No indicator (default)
    #[default]
    Default,

    /// Force open
    Open,

    /// Force close
    Close,
}

impl AccordionForceModifier {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            AccordionForceModifier::Default => "",
            AccordionForceModifier::Open => "collapse-open",
            AccordionForceModifier::Close => "collapse-close",
        }
    }
}
