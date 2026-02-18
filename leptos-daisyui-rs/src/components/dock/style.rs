/// # Dock Size Variants
///
/// Style enum for daisyUI dock size classes that control the dimensions
/// of dock navigation components.
#[derive(Clone, Debug, Default)]
pub enum DockSize {
    /// Default size (no size class applied)
    #[default]
    Default,

    /// Extra small dock for minimal layouts
    Xs,

    /// Small dock for compact navigation
    Sm,

    /// Medium dock for standard usage
    Md,

    /// Large dock for prominent navigation
    Lg,

    /// Extra large dock for maximum visibility
    Xl,
}

impl DockSize {
    /// CSS class string
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
