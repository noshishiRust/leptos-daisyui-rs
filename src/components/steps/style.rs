/// Direction variants for steps layout orientation.
///
/// The `StepsDirection` enum controls the flow direction of step indicators,
/// allowing for both traditional horizontal progress bars and space-efficient
/// vertical layouts.
///
/// # Variants
///
/// - `Horizontal` - Steps flow horizontally from left to right (default)
/// - `Vertical` - Steps flow vertically from top to bottom
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Horizontal` | (none) | Default horizontal layout |
/// | `Vertical` | `steps-vertical` | Vertical step layout |
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
///         // Horizontal steps (default)
///         <Steps direction=StepsDirection::Horizontal>
///             <Step>"Step 1"</Step>
///             <Step>"Step 2"</Step>
///         </Steps>
///
///         // Vertical steps for sidebar or narrow layouts
///         <Steps direction=StepsDirection::Vertical>
///             <Step>"Setup"</Step>
///             <Step>"Configure"</Step>
///             <Step>"Complete"</Step>
///         </Steps>
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum StepsDirection {
    /// Horizontal step flow from left to right (default)
    #[default]
    Horizontal,
    /// Vertical step flow from top to bottom
    Vertical,
}

impl StepsDirection {
    /// Returns the corresponding CSS class for the steps direction.
    ///
    /// # Returns
    ///
    /// A static string slice containing the CSS class name, or an empty string
    /// for the default horizontal direction.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            StepsDirection::Horizontal => "",
            StepsDirection::Vertical => "steps-vertical",
        }
    }
}

/// Color variants for individual step indicators.
///
/// The `StepColor` enum provides semantic color options for step indicators,
/// allowing you to convey different states such as completion, current progress,
/// warnings, or errors in your step sequences.
///
/// # Variants
///
/// - `Default` - Default step styling (neutral/inactive)
/// - `Primary` - Primary brand color (active/current step)
/// - `Secondary` - Secondary color variant
/// - `Accent` - Accent color for emphasis
/// - `Info` - Information state (typically blue)
/// - `Success` - Success state (typically green) - completed steps
/// - `Warning` - Warning state (typically yellow) - attention needed
/// - `Error` - Error state (typically red) - failed or blocked steps
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | (none) | Neutral/inactive step |
/// | `Primary` | `step-primary` | Active or current step |
/// | `Secondary` | `step-secondary` | Secondary variant |
/// | `Accent` | `step-accent` | Accent emphasis |
/// | `Info` | `step-info` | Information state |
/// | `Success` | `step-success` | Completed step |
/// | `Warning` | `step-warning` | Warning or attention |
/// | `Error` | `step-error` | Error or blocked step |
///
/// # Examples
///
/// ## Progress Indication
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Steps>
///             <Step color=StepColor::Success data_content="✓">"Account Created"</Step>
///             <Step color=StepColor::Success data_content="✓">"Email Verified"</Step>
///             <Step color=StepColor::Primary data_content="3">"Profile Setup"</Step>
///             <Step data_content="4">"Complete Setup"</Step>
///         </Steps>
///     }
/// }
/// ```
///
/// ## Error Handling
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Steps>
///             <Step color=StepColor::Success>"Payment Info"</Step>
///             <Step color=StepColor::Error>"Verification Failed"</Step>
///             <Step color=StepColor::Warning>"Retry Required"</Step>
///         </Steps>
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum StepColor {
    /// Default step styling - neutral/inactive appearance
    #[default]
    Default,
    /// Primary color - typically used for current/active step
    Primary,
    /// Secondary color variant
    Secondary,
    /// Accent color for special emphasis
    Accent,
    /// Information state - typically blue, for informational steps
    Info,
    /// Success state - typically green, for completed steps
    Success,
    /// Warning state - typically yellow, for steps requiring attention
    Warning,
    /// Error state - typically red, for failed or blocked steps
    Error,
}

impl StepColor {
    /// Returns the corresponding CSS class for the step color variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the CSS class name, or an empty string
    /// for the default variant.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            StepColor::Default => "",
            StepColor::Primary => "step-primary",
            StepColor::Secondary => "step-secondary",
            StepColor::Accent => "step-accent",
            StepColor::Info => "step-info",
            StepColor::Success => "step-success",
            StepColor::Warning => "step-warning",
            StepColor::Error => "step-error",
        }
    }
}
