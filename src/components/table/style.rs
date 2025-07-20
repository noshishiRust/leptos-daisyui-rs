/// Size variants for table components, affecting padding and font size.
///
/// The `TableSize` enum provides different sizing options for tables to accommodate
/// various data densities and design requirements. Smaller sizes are useful for
/// data-heavy tables, while larger sizes provide better readability.
///
/// # Variants
///
/// - `Default` - Standard table sizing (no additional class)
/// - `Xs` - Extra small with minimal padding (maps to `table-xs`)
/// - `Sm` - Small with reduced padding (maps to `table-sm`)
/// - `Md` - Medium with comfortable padding (maps to `table-md`)
/// - `Lg` - Large with generous padding (maps to `table-lg`)
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | (none) | Standard table styling |
/// | `Xs` | `table-xs` | Extra small padding and text |
/// | `Sm` | `table-sm` | Small padding and text |
/// | `Md` | `table-md` | Medium padding and text |
/// | `Lg` | `table-lg` | Large padding and text |
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         // Compact table for data-heavy displays
///         <Table size=TableSize::Xs>
///             // ... table content
///         </Table>
///
///         // Comfortable table for general use
///         <Table size=TableSize::Md>
///             // ... table content
///         </Table>
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum TableSize {
    /// Standard table sizing with default daisyUI styling
    #[default]
    Default,
    /// Extra small size with minimal padding - ideal for data-heavy tables
    Xs,
    /// Small size with reduced padding - good for compact displays
    Sm,
    /// Medium size with comfortable padding - balanced readability
    Md,
    /// Large size with generous padding - maximum readability
    Lg,
}

impl TableSize {
    /// Returns the corresponding CSS class for the table size variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the CSS class name, or an empty string
    /// for the default variant.
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
