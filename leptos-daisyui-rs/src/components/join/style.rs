/// # Join Direction Variants
///
/// Style enum for daisyUI join direction classes that control how child elements
/// are connected together in a joined layout.
#[derive(Clone, Debug, Default)]
pub enum JoinDirection {
    /// Elements joined horizontally in a row (default)
    #[default]
    Horizontal,

    /// Elements joined vertically in a column
    Vertical,
}

impl JoinDirection {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            JoinDirection::Horizontal => "join-horizontal",
            JoinDirection::Vertical => "join-vertical",
        }
    }
}
