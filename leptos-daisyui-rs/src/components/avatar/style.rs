/// # Avatar Modifier Variants
///
/// Style enum for daisyUI avatar modifier classes that control the status indicators
/// and type of avatar components.
#[derive(Clone, Debug, Default)]
pub enum AvatarModifier {
    /// Default avatar with no status indicator
    #[default]
    Default,

    /// Shows online status indicator (green dot)
    Online,

    /// Shows offline status indicator (gray dot)
    Offline,

    /// Indicates a placeholder avatar
    Placeholder,
}

impl AvatarModifier {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            AvatarModifier::Default => "",
            AvatarModifier::Online => "avatar-online",
            AvatarModifier::Offline => "avatar-offline",
            AvatarModifier::Placeholder => "avatar-placeholder",
        }
    }
}
