#[derive(Clone, Debug, Default)]
pub enum DockSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl DockSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            DockSize::Default => "",
            DockSize::Xs => "dock-xs",
            DockSize::Sm => "dock-sm",
            DockSize::Md => "dock-md",
            DockSize::Lg => "dock-lg",
            DockSize::Xl => "dock-xl",
        }
    }
}