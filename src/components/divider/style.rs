/// Color variants for divider styling and semantic meaning.
///
/// The `DividerColor` enum provides various color options for dividers,
/// allowing you to match the divider appearance to the content context
/// or convey semantic meaning through color.
///
/// # Variants
///
/// - `Default` - Default divider color (no additional class)
/// - `Neutral` - Neutral color variant
/// - `Primary` - Primary brand color
/// - `Secondary` - Secondary color variant
/// - `Accent` - Accent color for emphasis
/// - `Success` - Success color (typically green)
/// - `Warning` - Warning color (typically yellow/orange)
/// - `Info` - Information color (typically blue)
/// - `Error` - Error color (typically red)
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | (none) | Default theme color |
/// | `Neutral` | `divider-neutral` | Neutral gray tone |
/// | `Primary` | `divider-primary` | Primary brand color |
/// | `Secondary` | `divider-secondary` | Secondary brand color |
/// | `Accent` | `divider-accent` | Accent emphasis color |
/// | `Success` | `divider-success` | Success state color |
/// | `Warning` | `divider-warning` | Warning state color |
/// | `Info` | `divider-info` | Information state color |
/// | `Error` | `divider-error` | Error state color |
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
///         <div>
///             <Divider color=DividerColor::Primary>"Important Section"</Divider>
///             <Divider color=DividerColor::Success>"Completed Tasks"</Divider>
///             <Divider color=DividerColor::Warning>"Needs Attention"</Divider>
///         </div>
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum DividerColor {
    /// Default divider color using theme defaults
    #[default]
    Default,
    /// Neutral color variant for subtle separation
    Neutral,
    /// Primary brand color for important sections
    Primary,
    /// Secondary color variant
    Secondary,
    /// Accent color for special emphasis
    Accent,
    /// Success color for completed or positive sections
    Success,
    /// Warning color for sections requiring attention
    Warning,
    /// Information color for informational sections
    Info,
    /// Error color for problem or error sections
    Error,
}

impl DividerColor {
    /// Returns the corresponding CSS class for the divider color variant.
    ///
    /// # Returns
    ///
    /// A static string slice containing the CSS class name, or an empty string
    /// for the default variant.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            DividerColor::Default => "",
            DividerColor::Neutral => "divider-neutral",
            DividerColor::Primary => "divider-primary",
            DividerColor::Secondary => "divider-secondary",
            DividerColor::Accent => "divider-accent",
            DividerColor::Success => "divider-success",
            DividerColor::Warning => "divider-warning",
            DividerColor::Info => "divider-info",
            DividerColor::Error => "divider-error",
        }
    }
}

/// Direction variants for divider orientation.
///
/// The `DividerDirection` enum controls the orientation of the divider,
/// allowing for both traditional horizontal separators and vertical
/// separators for side-by-side content.
///
/// # Variants
///
/// - `Horizontal` - Horizontal divider line (default)
/// - `Vertical` - Vertical divider line
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Horizontal` | `divider-horizontal` | Horizontal separator line |
/// | `Vertical` | `divider-vertical` | Vertical separator line |
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
///         // Horizontal divider for stacked content
///         <div>
///             <p>"Top content"</p>
///             <Divider direction=DividerDirection::Horizontal />
///             <p>"Bottom content"</p>
///         </div>
///
///         // Vertical divider for side-by-side content
///         <div class="flex">
///             <div>"Left content"</div>
///             <Divider direction=DividerDirection::Vertical />
///             <div>"Right content"</div>
///         </div>
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum DividerDirection {
    /// Horizontal divider for separating content vertically (default)
    #[default]
    Horizontal,
    /// Vertical divider for separating content horizontally
    Vertical,
}

impl DividerDirection {
    /// Returns the corresponding CSS class for the divider direction.
    ///
    /// # Returns
    ///
    /// A static string slice containing the CSS class name.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            DividerDirection::Horizontal => "divider-horizontal",
            DividerDirection::Vertical => "divider-vertical",
        }
    }
}

/// Placement variants for divider text positioning.
///
/// The `DividerPlacement` enum controls where text content appears
/// within the divider, allowing for different visual layouts and
/// emphasis patterns.
///
/// # Variants
///
/// - `Default` - Center placement (default behavior)
/// - `Start` - Text aligned to the start of the divider
/// - `End` - Text aligned to the end of the divider
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | (none) | Center-aligned text |
/// | `Start` | `divider-start` | Start-aligned text |
/// | `End` | `divider-end` | End-aligned text |
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
///         <div>
///             <Divider placement=DividerPlacement::Start>
///                 "Left-aligned label"
///             </Divider>
///             
///             <Divider placement=DividerPlacement::Default>
///                 "Center-aligned label"
///             </Divider>
///             
///             <Divider placement=DividerPlacement::End>
///                 "Right-aligned label"
///             </Divider>
///         </div>
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum DividerPlacement {
    /// Default center placement for divider text
    #[default]
    Default,
    /// Start-aligned placement (left in LTR, right in RTL)
    Start,
    /// End-aligned placement (right in LTR, left in RTL)
    End,
}

impl DividerPlacement {
    /// Returns the corresponding CSS class for the divider text placement.
    ///
    /// # Returns
    ///
    /// A static string slice containing the CSS class name, or an empty string
    /// for the default center placement.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            DividerPlacement::Default => "",
            DividerPlacement::Start => "divider-start",
            DividerPlacement::End => "divider-end",
        }
    }
}
