/// # Pagination Size Variants
///
/// Style enum for daisyUI pagination size classes that control the physical dimensions
/// of pagination components. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum PaginationSize {
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
