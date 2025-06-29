#[derive(Clone, Debug, Default)]
pub enum TableSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
}

impl TableSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TableSize::Default => "",
            TableSize::Xs => "table-xs",
            TableSize::Sm => "table-sm",
            TableSize::Md => "table-md",
            TableSize::Lg => "table-lg",
        }
    }
}