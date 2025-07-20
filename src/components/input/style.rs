/// # Input Style Variants
///
/// Style enum for daisyUI input style classes that control the visual appearance
/// and behavior of input components. Styles modify the base input appearance.
///
/// ## daisyUI Style Classes
///
/// These variants correspond to daisyUI's input style classes:
/// - `input-ghost` - Ghost style with transparent background
///
/// ## Usage
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::input::*;
///
/// view! {
///     <Input style=InputStyle::Default placeholder="Standard input" />
///     <Input style=InputStyle::Ghost placeholder="Ghost input" />
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum InputStyle {
    /// Default input style (no style class applied)
    ///
    /// Maps to: no additional class
    #[default]
    Default,

    /// Ghost style with transparent background
    ///
    /// Maps to: `input-ghost`
    Ghost,
}

impl InputStyle {
    /// Returns the CSS class string for the input style
    ///
    /// # Returns
    /// - `""` for `Default` (no additional class)
    /// - `"input-ghost"` for `Ghost`
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            InputStyle::Default => "",
            InputStyle::Ghost => "input-ghost",
        }
    }
}

/// # Input Color Variants
///
/// Style enum for daisyUI input color classes that control the semantic color scheme
/// of input components. Colors follow daisyUI's semantic color system.
///
/// ## daisyUI Color Classes
///
/// These variants correspond to daisyUI's input color classes:
/// - `input-neutral` - Neutral dark color for non-saturated UI
/// - `input-primary` - Primary brand color
/// - `input-secondary` - Secondary brand color
/// - `input-accent` - Accent brand color
/// - `input-info` - Info color for informative inputs
/// - `input-success` - Success color for valid inputs
/// - `input-warning` - Warning color for caution inputs
/// - `input-error` - Error color for invalid inputs
///
/// ## Usage
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::input::*;
///
/// view! {
///     <Input color=InputColor::Primary placeholder="Primary input" />
///     <Input color=InputColor::Error placeholder="Error input" />
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum InputColor {
    /// Default input color (no color class applied)
    ///
    /// Maps to: no additional class
    #[default]
    Default,

    /// Neutral dark color for non-saturated UI elements
    ///
    /// Maps to: `input-neutral`
    Neutral,

    /// Primary brand color for main inputs
    ///
    /// Maps to: `input-primary`
    Primary,

    /// Secondary brand color for secondary inputs
    ///
    /// Maps to: `input-secondary`
    Secondary,

    /// Accent brand color for highlighted inputs
    ///
    /// Maps to: `input-accent`
    Accent,

    /// Info color for informative inputs
    ///
    /// Maps to: `input-info`
    Info,

    /// Success color for valid or successful inputs
    ///
    /// Maps to: `input-success`
    Success,

    /// Warning color for caution or attention inputs
    ///
    /// Maps to: `input-warning`
    Warning,

    /// Error color for invalid or failed inputs
    ///
    /// Maps to: `input-error`
    Error,
}

impl InputColor {
    /// Returns the CSS class string for the input color
    ///
    /// # Returns
    /// - `""` for `Default` (no additional class)
    /// - `"input-{color}"` for all other variants
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            InputColor::Default => "",
            InputColor::Neutral => "input-neutral",
            InputColor::Primary => "input-primary",
            InputColor::Secondary => "input-secondary",
            InputColor::Accent => "input-accent",
            InputColor::Info => "input-info",
            InputColor::Success => "input-success",
            InputColor::Warning => "input-warning",
            InputColor::Error => "input-error",
        }
    }
}

/// # Input Size Variants
///
/// Style enum for daisyUI input size classes that control the dimensions
/// of input components. Sizes range from extra small to extra large.
///
/// ## daisyUI Size Classes
///
/// These variants correspond to daisyUI's input size classes:
/// - `input-xs` - Extra small size (20px height)
/// - `input-sm` - Small size (24px height)  
/// - `input-md` - Medium size (32px height, default)
/// - `input-lg` - Large size (40px height)
/// - `input-xl` - Extra large size (48px height)
///
/// ## Usage
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::input::*;
///
/// view! {
///     <Input size=InputSize::Xs placeholder="Extra small" />
///     <Input size=InputSize::Md placeholder="Medium (default)" />
///     <Input size=InputSize::Xl placeholder="Extra large" />
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum InputSize {
    /// Extra small input size
    ///
    /// Maps to: `input-xs`
    Xs,

    /// Small input size
    ///
    /// Maps to: `input-sm`
    Sm,

    /// Medium input size (default)
    ///
    /// Maps to: `input-md`
    #[default]
    Md,

    /// Large input size
    ///
    /// Maps to: `input-lg`
    Lg,

    /// Extra large input size
    ///
    /// Maps to: `input-xl`
    Xl,
}

impl InputSize {
    /// Returns the CSS class string for the input size
    ///
    /// # Returns
    /// - `"input-{size}"` for all variants
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            InputSize::Xs => "input-xs",
            InputSize::Sm => "input-sm",
            InputSize::Md => "input-md",
            InputSize::Lg => "input-lg",
            InputSize::Xl => "input-xl",
        }
    }
}
