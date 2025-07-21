/// # Chat Placement Variants
///
/// Style enum for daisyUI chat placement classes that control the alignment
/// of chat messages within the conversation flow.
#[derive(Clone, Debug, Default)]
pub enum ChatPlacement {
    /// Left-aligned chat messages (sender on left side)
    #[default]
    Start,

    /// Right-aligned chat messages (sender on right side)
    End,
}

impl ChatPlacement {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            ChatPlacement::Start => "chat-start",
            ChatPlacement::End => "chat-end",
        }
    }
}

/// # Chat Bubble Color Variants
///
/// Style enum for daisyUI chat bubble color classes that control the semantic color scheme
/// of chat message bubbles. Colors follow daisyUI's semantic system for message context.
#[derive(Clone, Debug, Default)]
pub enum ChatBubbleColor {
    /// Default bubble color (no color class applied)
    #[default]
    Default,

    /// Neutral dark color for standard messages
    Neutral,

    /// Primary brand color for important messages
    Primary,

    /// Secondary brand color for secondary messages
    Secondary,

    /// Accent brand color for highlighted messages
    Accent,

    /// Info color for informational messages
    Info,

    /// Success color for positive/success messages
    Success,

    /// Warning color for warning messages
    Warning,

    /// Error color for error/critical messages
    Error,
}

impl ChatBubbleColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            ChatBubbleColor::Default => "",
            ChatBubbleColor::Neutral => "chat-bubble-neutral",
            ChatBubbleColor::Primary => "chat-bubble-primary",
            ChatBubbleColor::Secondary => "chat-bubble-secondary",
            ChatBubbleColor::Accent => "chat-bubble-accent",
            ChatBubbleColor::Info => "chat-bubble-info",
            ChatBubbleColor::Success => "chat-bubble-success",
            ChatBubbleColor::Warning => "chat-bubble-warning",
            ChatBubbleColor::Error => "chat-bubble-error",
        }
    }
}
