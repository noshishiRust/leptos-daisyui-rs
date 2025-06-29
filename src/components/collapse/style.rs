#[derive(Clone, Debug, Default)]
pub enum CollapseModifier {
    #[default]
    Default,
    Arrow,
    Plus,
    Open,
    Close,
}

impl CollapseModifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            CollapseModifier::Default => "",
            CollapseModifier::Arrow => "collapse-arrow",
            CollapseModifier::Plus => "collapse-plus",
            CollapseModifier::Open => "collapse-open",
            CollapseModifier::Close => "collapse-close",
        }
    }
}