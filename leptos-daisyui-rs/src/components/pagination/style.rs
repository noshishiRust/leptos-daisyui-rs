/// # Pagination Size Variants
///
/// Style enum for daisyUI pagination size classes that control the physical dimensions
/// of pagination components. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum PaginationSize {
    /// Extra small size for compact layouts
    Xs,

    /// Small size for minimal space usage
    Sm,

    /// Medium size for standard usage
    #[default]
    Md,

    /// Large size for emphasis and visibility
    Lg,

    /// X Large Size
    Xl,
}

impl PaginationSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            PaginationSize::Xs => "join-xs",
            PaginationSize::Sm => "join-sm",
            PaginationSize::Md => "join-md",
            PaginationSize::Lg => "join-lg",
            PaginationSize::Xl => "join-xl",
        }
    }
}
