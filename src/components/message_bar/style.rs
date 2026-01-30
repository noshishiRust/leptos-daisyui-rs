/// MessageBar type variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum MessageBarType {
    /// Info message (default)
    #[default]
    Info,
    /// Success message
    Success,
    /// Warning message
    Warning,
    /// Error message
    Error,
}

impl MessageBarType {
    /// Returns the background CSS class for this message type
    pub fn bg_class(&self) -> &'static str {
        match self {
            Self::Info => "bg-info",
            Self::Success => "bg-success",
            Self::Warning => "bg-warning",
            Self::Error => "bg-error",
        }
    }

    /// Returns the text color CSS class for this message type
    pub fn text_class(&self) -> &'static str {
        match self {
            Self::Info => "text-info-content",
            Self::Success => "text-success-content",
            Self::Warning => "text-warning-content",
            Self::Error => "text-error-content",
        }
    }

    /// Returns the icon character for this message type
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Info => "ℹ️",
            Self::Success => "✓",
            Self::Warning => "⚠",
            Self::Error => "✕",
        }
    }
}
