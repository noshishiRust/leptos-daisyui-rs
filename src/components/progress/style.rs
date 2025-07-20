/// # Progress Color Variants
///
/// Style enum for daisyUI progress color classes that control the semantic color scheme
/// of progress bar components. Colors follow daisyUI's semantic color system.
///
/// ## daisyUI Color Classes
///
/// These variants correspond to daisyUI's progress color classes:
/// - `progress-primary` - Primary brand color for main progress indicators
/// - `progress-secondary` - Secondary brand color for secondary progress
/// - `progress-accent` - Accent brand color for highlighted progress
/// - `progress-info` - Info color for informational progress
/// - `progress-success` - Success color for completed or positive progress
/// - `progress-warning` - Warning color for caution or intermediate progress
/// - `progress-error` - Error color for failed or problematic progress
///
/// ## Usage
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::progress::*;
///
/// view! {
///     <Progress color=ProgressColor::Primary value=75.0 max=100.0 />
///     <Progress color=ProgressColor::Success value=100.0 max=100.0 />
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum ProgressColor {
    /// Default progress color (no color class applied)
    ///
    /// Maps to: no additional class
    #[default]
    Default,

    /// Primary brand color for main progress indicators
    ///
    /// Maps to: `progress-primary`
    Primary,

    /// Secondary brand color for secondary progress
    ///
    /// Maps to: `progress-secondary`
    Secondary,

    /// Accent brand color for highlighted progress
    ///
    /// Maps to: `progress-accent`
    Accent,

    /// Success color for completed or positive progress
    ///
    /// Maps to: `progress-success`
    Success,

    /// Info color for informational progress
    ///
    /// Maps to: `progress-info`
    Info,

    /// Warning color for caution or intermediate progress
    ///
    /// Maps to: `progress-warning`
    Warning,

    /// Error color for failed or problematic progress
    ///
    /// Maps to: `progress-error`
    Error,
}

impl ProgressColor {
    /// Returns the corresponding daisyUI CSS class name for this color.
    ///
    /// # Returns
    /// - `""` for `Default` (no additional class)
    /// - `"progress-primary"` for `Primary`
    /// - `"progress-secondary"` for `Secondary`
    /// - `"progress-accent"` for `Accent`
    /// - `"progress-success"` for `Success`
    /// - `"progress-info"` for `Info`
    /// - `"progress-warning"` for `Warning`
    /// - `"progress-error"` for `Error`
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressColor::Default => "",
            ProgressColor::Primary => "progress-primary",
            ProgressColor::Secondary => "progress-secondary",
            ProgressColor::Accent => "progress-accent",
            ProgressColor::Success => "progress-success",
            ProgressColor::Info => "progress-info",
            ProgressColor::Warning => "progress-warning",
            ProgressColor::Error => "progress-error",
        }
    }
}
