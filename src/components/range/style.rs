/// Color variants for the Range component.
///
/// Each variant corresponds to a specific daisyUI range color class.
/// The default variant applies no additional color styling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | `""` | Default browser styling |
/// | `Primary` | `"range-primary"` | Primary theme color |
/// | `Secondary` | `"range-secondary"` | Secondary theme color |
/// | `Accent` | `"range-accent"` | Accent theme color |
/// | `Success` | `"range-success"` | Success/positive color (green) |
/// | `Warning` | `"range-warning"` | Warning color (yellow/orange) |
/// | `Info` | `"range-info"` | Info color (blue) |
/// | `Error` | `"range-error"` | Error/danger color (red) |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::range::RangeColor;
///
/// let color = RangeColor::Primary;
/// assert_eq!(color.as_str(), "range-primary");
/// ```
#[derive(Clone, Debug, Default)]
pub enum RangeColor {
    /// Default range styling with no additional color classes
    #[default]
    Default,
    /// Primary theme color
    Primary,
    /// Secondary theme color
    Secondary,
    /// Accent theme color
    Accent,
    /// Success/positive color (green)
    Success,
    /// Warning color (yellow/orange)
    Warning,
    /// Info color (blue)
    Info,
    /// Error/danger color (red)
    Error,
}

impl RangeColor {
    /// Returns the corresponding CSS class name for the range color variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the daisyUI CSS class name.
    /// Returns an empty string for the `Default` variant to avoid applying
    /// unnecessary CSS classes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leptos_daisyui::range::RangeColor;
    ///
    /// assert_eq!(RangeColor::Default.as_str(), "");
    /// assert_eq!(RangeColor::Primary.as_str(), "range-primary");
    /// assert_eq!(RangeColor::Error.as_str(), "range-error");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RangeColor::Default => "",
            RangeColor::Primary => "range-primary",
            RangeColor::Secondary => "range-secondary",
            RangeColor::Accent => "range-accent",
            RangeColor::Success => "range-success",
            RangeColor::Warning => "range-warning",
            RangeColor::Info => "range-info",
            RangeColor::Error => "range-error",
        }
    }
}

/// Size variants for the Range component.
///
/// Each variant corresponds to a specific daisyUI range size class.
/// The default variant applies no additional size styling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | `""` | Default range size |
/// | `Xs` | `"range-xs"` | Extra small range |
/// | `Sm` | `"range-sm"` | Small range |
/// | `Md` | `"range-md"` | Medium range |
/// | `Lg` | `"range-lg"` | Large range |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::range::RangeSize;
///
/// let size = RangeSize::Lg;
/// assert_eq!(size.as_str(), "range-lg");
/// ```
#[derive(Clone, Debug, Default)]
pub enum RangeSize {
    /// Default range size with no additional size classes
    #[default]
    Default,
    /// Extra small range
    Xs,
    /// Small range
    Sm,
    /// Medium range
    Md,
    /// Large range
    Lg,
}

impl RangeSize {
    /// Returns the corresponding CSS class name for the range size variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the daisyUI CSS class name.
    /// Returns an empty string for the `Default` variant to avoid applying
    /// unnecessary CSS classes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leptos_daisyui::range::RangeSize;
    ///
    /// assert_eq!(RangeSize::Default.as_str(), "");
    /// assert_eq!(RangeSize::Xs.as_str(), "range-xs");
    /// assert_eq!(RangeSize::Lg.as_str(), "range-lg");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RangeSize::Default => "",
            RangeSize::Xs => "range-xs",
            RangeSize::Sm => "range-sm",
            RangeSize::Md => "range-md",
            RangeSize::Lg => "range-lg",
        }
    }
}
