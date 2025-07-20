//! Style definitions for the Collapse component.
//!
//! This module provides the style enums and utilities for customizing
//! the appearance and behavior of collapse components.

/// Modifies the visual style and behavior of collapse components.
///
/// ## Variants
///
/// - `Default`: No visual indicator (tabindex-based toggle)
/// - `Arrow`: Shows a rotatable arrow indicator
/// - `Plus`: Shows a plus/minus toggle indicator  
/// - `Open`: Forces the collapse to be in open state
/// - `Close`: Forces the collapse to be in closed state
///
/// ## CSS Classes
///
/// Each variant maps to specific daisyUI CSS classes:
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | _(none)_ | Basic collapsible behavior |
/// | `Arrow` | `collapse-arrow` | Rotating arrow indicator |
/// | `Plus` | `collapse-plus` | Plus/minus toggle icon |
/// | `Open` | `collapse-open` | Forced open state |
/// | `Close` | `collapse-close` | Forced closed state |
///
/// ## Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Collapse, CollapseModifier};
///
/// // Arrow indicator collapse
/// view! {
///     <Collapse modifier=CollapseModifier::Arrow>
///         // content
///     </Collapse>
/// }
///
/// // Always open collapse
/// view! {
///     <Collapse modifier=CollapseModifier::Open>
///         // content always visible
///     </Collapse>
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum CollapseModifier {
    /// No visual indicator, basic collapsible behavior via tabindex
    #[default]
    Default,
    /// Displays a rotatable arrow indicator that shows expand/collapse state
    Arrow,
    /// Shows a plus/minus toggle indicator for expand/collapse state
    Plus,
    /// Forces the collapse to be in an open (expanded) state
    Open,
    /// Forces the collapse to be in a closed (collapsed) state
    Close,
}

impl CollapseModifier {
    /// Returns the CSS class string for the modifier.
    ///
    /// The default variant returns an empty string for optimal performance,
    /// while other variants return their corresponding daisyUI class names.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            CollapseModifier::Default => "",
            CollapseModifier::Arrow => "collapse-arrow",
            CollapseModifier::Plus => "collapse-plus",
            CollapseModifier::Open => "collapse-open",
            CollapseModifier::Close => "collapse-close",
        }
    }
}
