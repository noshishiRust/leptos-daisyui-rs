/// Modifies the visual style and behavior of collapse components.
#[derive(Clone, Debug, Default)]
pub enum CollapseModifier {
    /// No visual indicator, basic collapsible behavior via tabindex
    #[default]
    Default,

    /// Displays a rotatable arrow indicator that shows expand/collapse state
    Arrow,

    /// Shows a plus/minus toggle indicator for expand/collapse state
    Plus,
}

impl CollapseModifier {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            CollapseModifier::Default => "",
            CollapseModifier::Arrow => "collapse-arrow",
            CollapseModifier::Plus => "collapse-plus",
        }
    }
}

/// Modifies the visual style and behavior of collapse components.
#[derive(Clone, Debug, Default)]
pub enum CollapseForceModifier {
    /// No visual indicator, basic collapsible behavior via tabindex
    #[default]
    Default,

    /// Forces the collapse to be in an open (expanded) state
    Open,

    /// Forces the collapse to be in a closed (collapsed) state
    Close,
}

impl CollapseForceModifier {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            CollapseForceModifier::Default => "",
            CollapseForceModifier::Open => "collapse-open",
            CollapseForceModifier::Close => "collapse-close",
        }
    }
}
