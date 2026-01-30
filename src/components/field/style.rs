/// Validation state for the Field component
#[derive(Clone, Debug, Default, PartialEq)]
pub enum FieldState {
    /// Default state with no validation feedback
    #[default]
    Default,
    /// Error state with error message
    Error,
    /// Success state with success message
    Success,
    /// Warning state with warning message
    Warning,
}

impl FieldState {
    /// Returns the CSS class string for the field state
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldState::Default => "",
            FieldState::Error => "error",
            FieldState::Success => "success",
            FieldState::Warning => "warning",
        }
    }
}
