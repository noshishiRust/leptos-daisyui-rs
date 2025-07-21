/// # Table Size Variants
///
/// Style enum for daisyUI table size classes that control the physical dimensions
/// of table components. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum TableSize {
    /// Default size (no size class applied)
    #[default]
    Default,
    /// Extra small size for compact layouts
    Xs,
    /// Small size for minimal space usage
    Sm,
    /// Medium size for standard usage
    Md,
    /// Large size for emphasis and visibility
    Lg,
}

impl TableSize {
    /// CSS class string
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
