/// # Footer Placement Variants
///
/// Style enum for daisyUI footer placement classes that control the alignment
/// of footer content within the footer container.
#[derive(Clone, Debug, Default)]
pub enum FooterPlacement {
    /// Default footer placement (no placement class applied)
    #[default]
    Default,
    /// Center-aligned footer content
    Center,
}

impl FooterPlacement {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            FooterPlacement::Default => "",
            FooterPlacement::Center => "footer-center",
        }
    }
}

/// # Footer Direction Variants
///
/// Style enum for daisyUI footer direction classes that control the layout
/// direction of footer content items.
#[derive(Clone, Debug, Default)]
pub enum FooterDirection {
    /// Default footer direction (no direction class applied)
    #[default]
    Default,
    /// Horizontal layout for footer items
    Horizontal,
    /// Vertical layout for footer items
    Vertical,
}

impl FooterDirection {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            FooterDirection::Default => "",
            FooterDirection::Horizontal => "footer-horizontal",
            FooterDirection::Vertical => "footer-vertical",
        }
    }
}
