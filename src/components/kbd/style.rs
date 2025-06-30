#[derive(Clone, Debug, Default)]
pub enum KbdSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl KbdSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            KbdSize::Default => "",
            KbdSize::Xs => "kbd-xs",
            KbdSize::Sm => "kbd-sm",
            KbdSize::Md => "kbd-md",
            KbdSize::Lg => "kbd-lg",
            KbdSize::Xl => "kbd-xl",
        }
    }
}
