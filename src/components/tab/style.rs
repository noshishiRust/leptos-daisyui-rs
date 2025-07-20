//! Style enums for Tab component variants.

/// Defines the size/scale of tab components.
///
/// Controls the overall size of tabs including padding, font size, and spacing.
/// Maps to daisyUI's tab size classes for consistent scaling across the interface.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Padding | Font Size |
/// |---------|-----------|---------|----------|
/// | `Default` | (none) | `0.75rem` | `1rem` |
/// | `Xs` | `.tabs-xs` | `0.25rem` | `0.75rem` |
/// | `Sm` | `.tabs-sm` | `0.5rem` | `0.875rem` |
/// | `Md` | `.tabs-md` | `0.75rem` | `1rem` |
/// | `Lg` | `.tabs-lg` | `1rem` | `1.125rem` |
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, TabSize};
///
/// view! {
///     <Tabs size=TabSize::Lg>
///         // Large tabs with increased padding and font size
///     </Tabs>
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum TabSize {
    /// Default tab size with standard padding
    #[default]
    Default,
    /// Extra small tabs with minimal padding
    Xs,
    /// Small tabs with reduced padding
    Sm,
    /// Medium tabs with standard padding
    Md,
    /// Large tabs with increased padding
    Lg,
}

impl TabSize {
    /// Returns the CSS class string for this tab size.
    ///
    /// # Returns
    ///
    /// - `""` for `Default` (uses default styling)
    /// - `"tabs-xs"` for `Xs`
    /// - `"tabs-sm"` for `Sm`
    /// - `"tabs-md"` for `Md`
    /// - `"tabs-lg"` for `Lg`
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TabSize::Default => "",
            TabSize::Xs => "tabs-xs",
            TabSize::Sm => "tabs-sm",
            TabSize::Md => "tabs-md",
            TabSize::Lg => "tabs-lg",
        }
    }
}

/// Defines the visual style/variant of tab components.
///
/// Controls the overall appearance and visual treatment of tab containers.
/// Each variant provides different visual emphasis and context cues.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | (none) | Clean minimal tabs |
/// | `Bordered` | `.tabs-bordered` | Tabs with bottom borders |
/// | `Lifted` | `.tabs-lifted` | Tabs that appear lifted/elevated |
/// | `Boxed` | `.tabs-boxed` | Tabs with full boxing and background |
///
/// # Visual Examples
///
/// - **Default**: Simple underlined active state
/// - **Bordered**: All tabs have bottom borders, active tab highlighted
/// - **Lifted**: Active tab appears raised above content area
/// - **Boxed**: Tabs enclosed in boxes with background colors
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, TabVariant};
///
/// view! {
///     <Tabs variant=TabVariant::Lifted>
///         // Tabs will appear elevated/lifted
///     </Tabs>
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum TabVariant {
    /// Default minimal tab styling
    #[default]
    Default,
    /// Tabs with bottom borders for all items
    Bordered,
    /// Tabs that appear lifted/elevated above content
    Lifted,
    /// Fully boxed tabs with background and borders
    Boxed,
}

impl TabVariant {
    /// Returns the CSS class string for this tab variant.
    ///
    /// # Returns
    ///
    /// - `""` for `Default` (uses default styling)
    /// - `"tabs-bordered"` for `Bordered`
    /// - `"tabs-lifted"` for `Lifted`
    /// - `"tabs-boxed"` for `Boxed`
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TabVariant::Default => "",
            TabVariant::Bordered => "tabs-bordered",
            TabVariant::Lifted => "tabs-lifted",
            TabVariant::Boxed => "tabs-boxed",
        }
    }
}
