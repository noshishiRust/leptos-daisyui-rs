/// Style variants for the Select component.
///
/// Each variant corresponds to a specific daisyUI select style class.
/// The default variant applies standard select styling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | `""` | Standard select styling |
/// | `Ghost` | `"select-ghost"` | Transparent background with border on focus |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::select::SelectStyle;
///
/// let style = SelectStyle::Ghost;
/// assert_eq!(style.as_str(), "select-ghost");
/// ```
#[derive(Clone, Debug, Default)]
pub enum SelectStyle {
    /// Standard select styling with default appearance
    #[default]
    Default,
    /// Transparent background with border appearing on focus
    Ghost,
}

impl SelectStyle {
    /// Returns the corresponding CSS class name for the select style variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the daisyUI CSS class name.
    /// Returns an empty string for the `Default` variant to use standard styling.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leptos_daisyui::select::SelectStyle;
    ///
    /// assert_eq!(SelectStyle::Default.as_str(), "");
    /// assert_eq!(SelectStyle::Ghost.as_str(), "select-ghost");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectStyle::Default => "",
            SelectStyle::Ghost => "select-ghost",
        }
    }
}

/// Color variants for the Select component.
///
/// Each variant corresponds to a specific daisyUI select color class.
/// The default variant applies no additional color styling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | `""` | Default browser styling |
/// | `Primary` | `"select-primary"` | Primary theme color |
/// | `Secondary` | `"select-secondary"` | Secondary theme color |
/// | `Accent` | `"select-accent"` | Accent theme color |
/// | `Info` | `"select-info"` | Info color (blue) |
/// | `Success` | `"select-success"` | Success/positive color (green) |
/// | `Warning` | `"select-warning"` | Warning color (yellow/orange) |
/// | `Error` | `"select-error"` | Error/danger color (red) |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::select::SelectColor;
///
/// let color = SelectColor::Primary;
/// assert_eq!(color.as_str(), "select-primary");
/// ```
#[derive(Clone, Debug, Default)]
pub enum SelectColor {
    /// Default select styling with no additional color classes
    #[default]
    Default,
    /// Primary theme color
    Primary,
    /// Secondary theme color
    Secondary,
    /// Accent theme color
    Accent,
    /// Info color (blue)
    Info,
    /// Success/positive color (green)
    Success,
    /// Warning color (yellow/orange)
    Warning,
    /// Error/danger color (red)
    Error,
}

impl SelectColor {
    /// Returns the corresponding CSS class name for the select color variant.
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
    /// use leptos_daisyui::select::SelectColor;
    ///
    /// assert_eq!(SelectColor::Default.as_str(), "");
    /// assert_eq!(SelectColor::Primary.as_str(), "select-primary");
    /// assert_eq!(SelectColor::Error.as_str(), "select-error");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectColor::Default => "",
            SelectColor::Primary => "select-primary",
            SelectColor::Secondary => "select-secondary",
            SelectColor::Accent => "select-accent",
            SelectColor::Info => "select-info",
            SelectColor::Success => "select-success",
            SelectColor::Warning => "select-warning",
            SelectColor::Error => "select-error",
        }
    }
}

/// Size variants for the Select component.
///
/// Each variant corresponds to a specific daisyUI select size class.
/// Note that unlike other components, SelectSize does not have a `Default`
/// variant that returns an empty string - all sizes map to specific CSS classes.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Xs` | `"select-xs"` | Extra small select |
/// | `Sm` | `"select-sm"` | Small select |
/// | `Md` | `"select-md"` | Medium select (default) |
/// | `Lg` | `"select-lg"` | Large select |
/// | `Xl` | `"select-xl"` | Extra large select |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::select::SelectSize;
///
/// let size = SelectSize::Lg;
/// assert_eq!(size.as_str(), "select-lg");
/// ```
#[derive(Clone, Debug, Default)]
pub enum SelectSize {
    /// Extra small select
    Xs,
    /// Small select
    Sm,
    /// Medium select (default size)
    #[default]
    Md,
    /// Large select
    Lg,
    /// Extra large select
    Xl,
}

impl SelectSize {
    /// Returns the corresponding CSS class name for the select size variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the daisyUI CSS class name.
    /// Unlike other components, all size variants return a specific CSS class.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leptos_daisyui::select::SelectSize;
    ///
    /// assert_eq!(SelectSize::Xs.as_str(), "select-xs");
    /// assert_eq!(SelectSize::Md.as_str(), "select-md");
    /// assert_eq!(SelectSize::Xl.as_str(), "select-xl");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectSize::Xs => "select-xs",
            SelectSize::Sm => "select-sm",
            SelectSize::Md => "select-md",
            SelectSize::Lg => "select-lg",
            SelectSize::Xl => "select-xl",
        }
    }
}
