/// Color variants for the Toggle component.
///
/// Each variant corresponds to a specific daisyUI toggle color class.
/// The default variant applies no additional color styling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | `""` | Default browser styling |
/// | `Primary` | `"toggle-primary"` | Primary theme color |
/// | `Secondary` | `"toggle-secondary"` | Secondary theme color |
/// | `Accent` | `"toggle-accent"` | Accent theme color |
/// | `Success` | `"toggle-success"` | Success/positive color (green) |
/// | `Warning` | `"toggle-warning"` | Warning color (yellow/orange) |
/// | `Info` | `"toggle-info"` | Info color (blue) |
/// | `Error` | `"toggle-error"` | Error/danger color (red) |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::toggle::ToggleColor;
///
/// let color = ToggleColor::Primary;
/// assert_eq!(color.as_str(), "toggle-primary");
/// ```
#[derive(Clone, Debug, Default)]
pub enum ToggleColor {
    /// Default toggle styling with no additional color classes
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

impl ToggleColor {
    /// Returns the corresponding CSS class name for the toggle color variant.
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
    /// use leptos_daisyui::toggle::ToggleColor;
    ///
    /// assert_eq!(ToggleColor::Default.as_str(), "");
    /// assert_eq!(ToggleColor::Primary.as_str(), "toggle-primary");
    /// assert_eq!(ToggleColor::Error.as_str(), "toggle-error");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            ToggleColor::Default => "",
            ToggleColor::Primary => "toggle-primary",
            ToggleColor::Secondary => "toggle-secondary",
            ToggleColor::Accent => "toggle-accent",
            ToggleColor::Success => "toggle-success",
            ToggleColor::Warning => "toggle-warning",
            ToggleColor::Info => "toggle-info",
            ToggleColor::Error => "toggle-error",
        }
    }
}

/// Size variants for the Toggle component.
///
/// Each variant corresponds to a specific daisyUI toggle size class.
/// Note that unlike other components, ToggleSize does not have a `Default`
/// variant that returns an empty string - all sizes map to specific CSS classes.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Xs` | `"toggle-xs"` | Extra small toggle |
/// | `Sm` | `"toggle-sm"` | Small toggle |
/// | `Md` | `"toggle-md"` | Medium toggle (default) |
/// | `Lg` | `"toggle-lg"` | Large toggle |
/// | `Xl` | `"toggle-xl"` | Extra large toggle |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::toggle::ToggleSize;
///
/// let size = ToggleSize::Lg;
/// assert_eq!(size.as_str(), "toggle-lg");
/// ```
#[derive(Clone, Debug, Default)]
pub enum ToggleSize {
    /// Extra small toggle
    Xs,
    /// Small toggle
    Sm,
    /// Medium toggle (default size)
    #[default]
    Md,
    /// Large toggle
    Lg,
    /// Extra large toggle
    Xl,
}

impl ToggleSize {
    /// Returns the corresponding CSS class name for the toggle size variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the daisyUI CSS class name.
    /// Unlike other components, all size variants return a specific CSS class.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leptos_daisyui::toggle::ToggleSize;
    ///
    /// assert_eq!(ToggleSize::Xs.as_str(), "toggle-xs");
    /// assert_eq!(ToggleSize::Md.as_str(), "toggle-md");
    /// assert_eq!(ToggleSize::Xl.as_str(), "toggle-xl");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            ToggleSize::Xs => "toggle-xs",
            ToggleSize::Sm => "toggle-sm",
            ToggleSize::Md => "toggle-md",
            ToggleSize::Lg => "toggle-lg",
            ToggleSize::Xl => "toggle-xl",
        }
    }
}
