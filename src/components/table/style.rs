/// # Table Size Variants
///
/// Style enum for daisyUI table size classes that control the physical dimensions
/// of table components. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum TableSize {
    /// Extra small size for compact layouts
    Xs,

    /// Small size for minimal space usage
    Sm,

    /// Medium size for standard usage
    #[default]
    Md,

    /// Large size for emphasis and visibility
    Lg,

    /// Extra large size for compact layouts
    Xl,
}

impl TableSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TableSize::Xs => "table-xs",
            TableSize::Sm => "table-sm",
            TableSize::Md => "table-md",
            TableSize::Lg => "table-lg",
            TableSize::Xl => "table-xl",
        }
    }
}
