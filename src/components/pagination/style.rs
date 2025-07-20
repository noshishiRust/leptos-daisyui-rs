#[derive(Clone, Debug, Default)]
pub enum PaginationSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
}

impl PaginationSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            PaginationSize::Default => "",
            PaginationSize::Xs => "join-xs",
            PaginationSize::Sm => "join-sm",
            PaginationSize::Md => "join-md",
            PaginationSize::Lg => "join-lg",
        }
    }
}
