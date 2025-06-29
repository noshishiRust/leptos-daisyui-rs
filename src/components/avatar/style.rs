#[derive(Clone, Debug, Default)]
pub enum AvatarModifier {
    #[default]
    Default,
    Online,
    Offline,
    Placeholder,
}

impl AvatarModifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            AvatarModifier::Default => "",
            AvatarModifier::Online => "avatar-online",
            AvatarModifier::Offline => "avatar-offline",
            AvatarModifier::Placeholder => "avatar-placeholder",
        }
    }
}
