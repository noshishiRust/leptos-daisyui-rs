//! Style enums for Menu component variants.

/// Defines the layout direction of menu items.
///
/// Controls whether menu items are arranged vertically (default) or horizontally.
/// Maps to daisyUI's menu direction classes.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Vertical` | (none) | Default vertical layout |
/// | `Horizontal` | `.menu-horizontal` | Horizontal inline layout |
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Menu, MenuDirection};
///
/// view! {
///     <Menu direction=MenuDirection::Horizontal>
///         // Menu items will be arranged horizontally
///     </Menu>
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum MenuDirection {
    /// Default vertical layout with items stacked vertically
    #[default]
    Vertical,
    /// Horizontal layout with items arranged inline
    Horizontal,
}

impl MenuDirection {
    /// Returns the CSS class string for this menu direction.
    ///
    /// # Returns
    ///
    /// - `""` for `Vertical` (default styling)
    /// - `"menu-horizontal"` for `Horizontal`
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuDirection::Vertical => "",
            MenuDirection::Horizontal => "menu-horizontal",
        }
    }
}

/// Defines the size/scale of menu items.
///
/// Controls the overall size of menu items including padding, font size, and spacing.
/// Maps to daisyUI's menu size classes for consistent scaling.
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Padding | Font Size |
/// |---------|-----------|---------|----------|
/// | `Xs` | `.menu-xs` | `0.25rem` | `0.75rem` |
/// | `Sm` | `.menu-sm` | `0.5rem` | `0.875rem` |
/// | `Md` | `.menu-md` | `0.75rem` | `1rem` |
/// | `Lg` | `.menu-lg` | `1rem` | `1.125rem` |
/// | `Xl` | `.menu-xl` | `1.25rem` | `1.25rem` |
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Menu, MenuSize};
///
/// view! {
///     <Menu size=MenuSize::Lg>
///         // Large menu items with increased padding
///     </Menu>
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum MenuSize {
    /// Extra small menu items with minimal padding
    Xs,
    /// Small menu items with reduced padding
    Sm,
    /// Medium menu items with standard padding (default)
    #[default]
    Md,
    /// Large menu items with increased padding
    Lg,
    /// Extra large menu items with maximum padding
    Xl,
}

impl MenuSize {
    /// Returns the CSS class string for this menu size.
    ///
    /// # Returns
    ///
    /// The corresponding daisyUI menu size class:
    /// - `"menu-xs"` for `Xs`
    /// - `"menu-sm"` for `Sm`
    /// - `"menu-md"` for `Md`
    /// - `"menu-lg"` for `Lg`
    /// - `"menu-xl"` for `Xl`
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuSize::Xs => "menu-xs",
            MenuSize::Sm => "menu-sm",
            MenuSize::Md => "menu-md",
            MenuSize::Lg => "menu-lg",
            MenuSize::Xl => "menu-xl",
        }
    }
}
