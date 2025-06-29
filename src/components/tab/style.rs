#[derive(Clone, Debug, Default)]
pub enum TabSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
}

impl TabSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TabSize::Default => "",
            TabSize::Xs => "tabs-xs",
            TabSize::Sm => "tabs-sm",
            TabSize::Md => "tabs-md",
            TabSize::Lg => "tabs-lg",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum TabVariant {
    #[default]
    Default,
    Bordered,
    Lifted,
    Boxed,
}

impl TabVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            TabVariant::Default => "",
            TabVariant::Bordered => "tabs-bordered",
            TabVariant::Lifted => "tabs-lifted",
            TabVariant::Boxed => "tabs-boxed",
        }
    }
}