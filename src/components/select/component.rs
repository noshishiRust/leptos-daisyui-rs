use super::style::{SelectColor, SelectSize, SelectStyle};
use crate::merge_classes;
use leptos::{
    html::{Option_, Select as HtmlSelect},
    prelude::*,
};

/// A dropdown selection component for choosing from multiple options.
///
/// The Select component provides a styled dropdown interface that allows users to
/// choose from a list of options. It supports various styling options and integrates
/// well with forms and validation.
///
/// # Props
///
/// * `style` - The style variant of the select (optional, reactive)
/// * `color` - The color variant of the select (optional, reactive)
/// * `size` - The size variant of the select (optional, reactive)
/// * `disabled` - Whether the select is disabled (optional, reactive)
/// * `class` - Additional CSS classes to apply (optional)
/// * `node_ref` - Reference to the underlying HTML select element (optional)
/// * `children` - The child elements (typically `SelectOption` components)
///
/// # CSS Classes
///
/// This component applies the following CSS classes:
/// - Base: `select`
/// - Style: Applied from `SelectStyle` enum
/// - Color: Applied from `SelectColor` enum
/// - Size: Applied from `SelectSize` enum
/// - Additional: Any classes provided via the `class` prop
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::select::{Select, SelectOption, SelectColor, SelectSize, SelectStyle};
///
/// #[component]
/// fn CountrySelector() -> impl IntoView {
///     view! {
///         <div class="form-control w-full max-w-xs">
///             <label class="label">
///                 <span class="label-text">"Select your country"</span>
///             </label>
///             <Select
///                 style=SelectStyle::Default
///                 color=SelectColor::Primary
///                 size=SelectSize::Md
///             >
///                 <SelectOption>"Choose a country"</SelectOption>
///                 <SelectOption>"United States"</SelectOption>
///                 <SelectOption>"Canada"</SelectOption>
///                 <SelectOption>"United Kingdom"</SelectOption>
///                 <SelectOption disabled=true>"Unavailable Option"</SelectOption>
///             </Select>
///         </div>
///     }
/// }
/// ```
///
/// # Accessibility
///
/// - Uses semantic `<select>` element for proper screen reader support
/// - Supports keyboard navigation (arrow keys, Enter to select, typing to search)
/// - Properly announces options and current selection to screen readers
/// - Compatible with form validation and submission
/// - Supports focus management and visual focus indicators
#[component]
pub fn Select(
    /// The style variant of the select
    #[prop(optional, into)]
    style: Signal<SelectStyle>,
    /// The color variant of the select
    #[prop(optional, into)]
    color: Signal<SelectColor>,
    /// The size variant of the select
    #[prop(optional, into)]
    size: Signal<SelectSize>,
    /// Whether the select is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying HTML select element
    #[prop(optional)]
    node_ref: NodeRef<HtmlSelect>,
    /// The child elements (typically SelectOption components)
    children: Children,
) -> impl IntoView {
    view! {
        <select
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "select",
                style.get().as_str(),
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            disabled=disabled
        >
            {children()}
        </select>
    }
}

/// An option element for use within a Select component.
///
/// The SelectOption component represents a single selectable option within a
/// Select dropdown. It provides the standard HTML option functionality with
/// additional styling support.
///
/// # Props
///
/// * `disabled` - Whether the option is disabled and cannot be selected (optional, reactive)
/// * `class` - Additional CSS classes to apply (optional)
/// * `node_ref` - Reference to the underlying HTML option element (optional)
/// * `children` - The content of the option (typically text)
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::select::{Select, SelectOption, SelectColor};
///
/// #[component]
/// fn PrioritySelector() -> impl IntoView {
///     view! {
///         <Select color=SelectColor::Info>
///             <SelectOption>"Select priority"</SelectOption>
///             <SelectOption>"Low"</SelectOption>
///             <SelectOption>"Medium"</SelectOption>
///             <SelectOption>"High"</SelectOption>
///             <SelectOption disabled=true>"Urgent (Premium only)"</SelectOption>
///         </Select>
///     }
/// }
/// ```
///
/// # Accessibility
///
/// - Uses semantic `<option>` element
/// - Properly integrates with parent `<select>` for screen reader support
/// - Supports disabled state with appropriate ARIA attributes
/// - Compatible with keyboard navigation patterns
#[component]
pub fn SelectOption(
    /// Whether the option is disabled and cannot be selected
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying HTML option element
    #[prop(optional)]
    node_ref: NodeRef<Option_>,
    /// The content of the option
    children: Children,
) -> impl IntoView {
    view! {
        <option node_ref=node_ref class=class disabled=disabled>
            {children()}
        </option>
    }
}
