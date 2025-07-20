/// Style variants for the Textarea component.
///
/// Each variant corresponds to a specific daisyUI textarea style class.
/// The default variant applies standard textarea styling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | `""` | Standard textarea styling |
/// | `Ghost` | `"textarea-ghost"` | Transparent background with border on focus |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::textarea::TextareaStyle;
///
/// let style = TextareaStyle::Ghost;
/// assert_eq!(style.as_str(), "textarea-ghost");
/// ```
#[derive(Clone, Debug, Default)]
pub enum TextareaStyle {
    /// Standard textarea styling with default appearance
    #[default]
    Default,
    /// Transparent background with border appearing on focus
    Ghost,
}

impl TextareaStyle {
    /// Returns the corresponding CSS class name for the textarea style variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the daisyUI CSS class name.
    /// Returns an empty string for the `Default` variant to use standard styling.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leptos_daisyui::textarea::TextareaStyle;
    ///
    /// assert_eq!(TextareaStyle::Default.as_str(), "");
    /// assert_eq!(TextareaStyle::Ghost.as_str(), "textarea-ghost");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaStyle::Default => "",
            TextareaStyle::Ghost => "textarea-ghost",
        }
    }
}

/// Color variants for the Textarea component.
///
/// Each variant corresponds to a specific daisyUI textarea color class.
/// The default variant applies no additional color styling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | `""` | Default browser styling |
/// | `Primary` | `"textarea-primary"` | Primary theme color |
/// | `Secondary` | `"textarea-secondary"` | Secondary theme color |
/// | `Accent` | `"textarea-accent"` | Accent theme color |
/// | `Info` | `"textarea-info"` | Info color (blue) |
/// | `Success` | `"textarea-success"` | Success/positive color (green) |
/// | `Warning` | `"textarea-warning"` | Warning color (yellow/orange) |
/// | `Error` | `"textarea-error"` | Error/danger color (red) |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::textarea::TextareaColor;
///
/// let color = TextareaColor::Primary;
/// assert_eq!(color.as_str(), "textarea-primary");
/// ```
#[derive(Clone, Debug, Default)]
pub enum TextareaColor {
    /// Default textarea styling with no additional color classes
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

impl TextareaColor {
    /// Returns the corresponding CSS class name for the textarea color variant.
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
    /// use leptos_daisyui::textarea::TextareaColor;
    ///
    /// assert_eq!(TextareaColor::Default.as_str(), "");
    /// assert_eq!(TextareaColor::Primary.as_str(), "textarea-primary");
    /// assert_eq!(TextareaColor::Error.as_str(), "textarea-error");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaColor::Default => "",
            TextareaColor::Primary => "textarea-primary",
            TextareaColor::Secondary => "textarea-secondary",
            TextareaColor::Accent => "textarea-accent",
            TextareaColor::Info => "textarea-info",
            TextareaColor::Success => "textarea-success",
            TextareaColor::Warning => "textarea-warning",
            TextareaColor::Error => "textarea-error",
        }
    }
}

/// Size variants for the Textarea component.
///
/// Each variant corresponds to a specific daisyUI textarea size class.
/// Note that unlike other components, TextareaSize does not have a `Default`
/// variant that returns an empty string - all sizes map to specific CSS classes.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Xs` | `"textarea-xs"` | Extra small textarea |
/// | `Sm` | `"textarea-sm"` | Small textarea |
/// | `Md` | `"textarea-md"` | Medium textarea (default) |
/// | `Lg` | `"textarea-lg"` | Large textarea |
/// | `Xl` | `"textarea-xl"` | Extra large textarea |
///
/// # Example
///
/// ```rust
/// use leptos_daisyui::textarea::TextareaSize;
///
/// let size = TextareaSize::Lg;
/// assert_eq!(size.as_str(), "textarea-lg");
/// ```
#[derive(Clone, Debug, Default)]
pub enum TextareaSize {
    /// Extra small textarea
    Xs,
    /// Small textarea
    Sm,
    /// Medium textarea (default size)
    #[default]
    Md,
    /// Large textarea
    Lg,
    /// Extra large textarea
    Xl,
}

impl TextareaSize {
    /// Returns the corresponding CSS class name for the textarea size variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the daisyUI CSS class name.
    /// Unlike other components, all size variants return a specific CSS class.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leptos_daisyui::textarea::TextareaSize;
    ///
    /// assert_eq!(TextareaSize::Xs.as_str(), "textarea-xs");
    /// assert_eq!(TextareaSize::Md.as_str(), "textarea-md");
    /// assert_eq!(TextareaSize::Xl.as_str(), "textarea-xl");
    /// ```
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaSize::Xs => "textarea-xs",
            TextareaSize::Sm => "textarea-sm",
            TextareaSize::Md => "textarea-md",
            TextareaSize::Lg => "textarea-lg",
            TextareaSize::Xl => "textarea-xl",
        }
    }
}
