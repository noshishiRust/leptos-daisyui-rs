#[derive(Clone, Debug, Default)]
pub enum ChatPlacement {
    #[default]
    Start,
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

#[derive(Clone, Debug, Default)]
pub enum ChatBubbleColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
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
