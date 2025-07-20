/// Color variants for the Radio component.
///
/// Each variant corresponds to a specific daisyUI radio color class.
/// The default variant applies no additional color styling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | `""` | Default browser styling |
/// | `Primary` | `"radio-primary"` | Primary theme color |
/// | `Secondary` | `"radio-secondary"` | Secondary theme color |
/// | `Accent` | `"radio-accent"` | Accent theme color |
/// | `Success` | `"radio-success"` | Success/positive color (green) |
/// | `Warning` | `"radio-warning"` | Warning color (yellow/orange) |
/// | `Info` | `"radio-info"` | Info color (blue) |
/// | `Error` | `"radio-error"` | Error/danger color (red) |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::radio::RadioColor;
///
/// let color = RadioColor::Primary;
/// assert_eq!(color.as_str(), "radio-primary");
/// ```
#[derive(Clone, Debug, Default)]
pub enum RadioColor {
    /// Default radio styling with no additional color classes
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

impl RadioColor {
    /// Returns the corresponding CSS class name for the radio color variant.
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
    /// use leptos_daisyui::radio::RadioColor;
    ///
    /// assert_eq!(RadioColor::Default.as_str(), "");
    /// assert_eq!(RadioColor::Primary.as_str(), "radio-primary");
    /// assert_eq!(RadioColor::Error.as_str(), "radio-error");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RadioColor::Default => "",
            RadioColor::Primary => "radio-primary",
            RadioColor::Secondary => "radio-secondary",
            RadioColor::Accent => "radio-accent",
            RadioColor::Success => "radio-success",
            RadioColor::Warning => "radio-warning",
            RadioColor::Info => "radio-info",
            RadioColor::Error => "radio-error",
        }
    }
}

/// Size variants for the Radio component.
///
/// Each variant corresponds to a specific daisyUI radio size class.
/// The default variant applies no additional size styling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | `""` | Default radio size |
/// | `Xs` | `"radio-xs"` | Extra small radio |
/// | `Sm` | `"radio-sm"` | Small radio |
/// | `Md` | `"radio-md"` | Medium radio |
/// | `Lg` | `"radio-lg"` | Large radio |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::radio::RadioSize;
///
/// let size = RadioSize::Lg;
/// assert_eq!(size.as_str(), "radio-lg");
/// ```
#[derive(Clone, Debug, Default)]
pub enum RadioSize {
    /// Default radio size with no additional size classes
    #[default]
    Default,
    /// Extra small radio
    Xs,
    /// Small radio
    Sm,
    /// Medium radio
    Md,
    /// Large radio
    Lg,
}

impl RadioSize {
    /// Returns the corresponding CSS class name for the radio size variant.
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
    /// use leptos_daisyui::radio::RadioSize;
    ///
    /// assert_eq!(RadioSize::Default.as_str(), "");
    /// assert_eq!(RadioSize::Xs.as_str(), "radio-xs");
    /// assert_eq!(RadioSize::Lg.as_str(), "radio-lg");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RadioSize::Default => "",
            RadioSize::Xs => "radio-xs",
            RadioSize::Sm => "radio-sm",
            RadioSize::Md => "radio-md",
            RadioSize::Lg => "radio-lg",
        }
    }
}
