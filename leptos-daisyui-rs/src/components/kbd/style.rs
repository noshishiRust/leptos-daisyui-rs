/// # Kbd Size Variants
///
/// Style enum for daisyUI kbd size classes that control the physical dimensions
/// of keyboard key display components. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum KbdSize {
    /// Extra small size for inline text usage
    Xs,

    /// Small size for compact layouts
    Sm,

    /// Medium size for standard usage
    #[default]
    Md,

    /// Large size for emphasis and visibility
    Lg,

    /// Extra large size for prominent display
    Xl,
}

impl KbdSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            KbdSize::Xs => "kbd-xs",
            KbdSize::Sm => "kbd-sm",
            KbdSize::Md => "kbd-md",
            KbdSize::Lg => "kbd-lg",
            KbdSize::Xl => "kbd-xl",
        }
    }
}
