//! Tab component implementations for tabbed navigation.

use super::style::{TabSize, TabVariant};
use crate::merge_classes;
use leptos::{
    html::{A, Div, Input},
    prelude::*,
};

/// A tab container component for organizing related content.
///
/// Creates a horizontal tab navigation bar with configurable size and visual style.
/// Serves as the container for individual Tab or TabRadio components.
///
/// # Props
///
/// - `size` - Size variant controlling tab dimensions and font size
/// - `variant` - Visual style variant (default, bordered, lifted, boxed)
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<div>` element
/// - `children` - Tab content (typically Tab or TabRadio components)
///
/// # Examples
///
/// ## Basic Tab Navigation
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, Tab};
///
/// #[component]
/// fn TabExample() -> impl IntoView {
///     let active_tab = RwSignal::new(0);
///
///     view! {
///         <Tabs>
///             <Tab
///                 active=Signal::derive(move || active_tab.get() == 0)
///                 on_click=Box::new(move || active_tab.set(0))
///             >
///                 "Home"
///             </Tab>
///             <Tab
///                 active=Signal::derive(move || active_tab.get() == 1)
///                 on_click=Box::new(move || active_tab.set(1))
///             >
///                 "About"
///             </Tab>
///             <Tab
///                 active=Signal::derive(move || active_tab.get() == 2)
///                 on_click=Box::new(move || active_tab.set(2))
///             >
///                 "Contact"
///             </Tab>
///         </Tabs>
///     }
/// }
/// ```
///
/// ## Styled Tabs
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, Tab, TabVariant, TabSize};
///
/// view! {
///     <Tabs variant=TabVariant::Bordered size=TabSize::Lg>
///         <Tab active=Signal::derive(|| true)>"Active"</Tab>
///         <Tab>"Inactive"</Tab>
///         <Tab disabled=Signal::derive(|| true)>"Disabled"</Tab>
///     </Tabs>
/// }
/// ```
///
/// ## Radio-based Tabs
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, TabRadio, TabVariant};
///
/// view! {
///     <Tabs variant=TabVariant::Boxed>
///         <TabRadio
///             name="tab-group"
///             checked=Signal::derive(|| true)
///         >
///             "Tab 1"
///         </TabRadio>
///         <TabRadio name="tab-group">"Tab 2"</TabRadio>
///         <TabRadio name="tab-group">"Tab 3"</TabRadio>
///     </Tabs>
/// }
/// ```
///
/// ## Lifted Tab Style
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, Tab, TabVariant};
///
/// view! {
///     <Tabs variant=TabVariant::Lifted class="-mb-px">
///         <Tab active=Signal::derive(|| true)>"Documents"</Tab>
///         <Tab>"Settings"</Tab>
///         <Tab>"Help"</Tab>
///     </Tabs>
///     <div class="border border-t-0 p-4">
///         "Content area appears connected to active tab"
///     </div>
/// }
/// ```
#[component]
pub fn Tabs(
    /// Size variant for tab dimensions
    #[prop(optional, into)]
    size: Signal<TabSize>,
    /// Visual style variant
    #[prop(optional, into)]
    variant: Signal<TabVariant>,
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Tab content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "tabs",
                size.get().as_str(),
                variant.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}

/// An individual tab item component.
///
/// Creates a clickable tab that can be active, disabled, or interactive.
/// Uses anchor element semantics for proper accessibility and navigation.
///
/// # Props
///
/// - `active` - Whether this tab is currently active/selected
/// - `disabled` - Whether this tab is disabled and non-interactive
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<a>` element
/// - `on_click` - Optional click handler function
/// - `children` - Tab label content (text, icons, etc.)
///
/// # Behavior
///
/// - Displays with active styling when `active=true`
/// - Ignores clicks and shows disabled styling when `disabled=true`
/// - Calls `on_click` handler when clicked (if provided and not disabled)
/// - Supports keyboard navigation (Enter/Space)
///
/// # Examples
///
/// ## Basic Tab
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::Tab;
///
/// view! {
///     <Tab active=Signal::derive(|| true)>
///         "Active Tab"
///     </Tab>
/// }
/// ```
///
/// ## Interactive Tab with Click Handler
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::Tab;
///
/// #[component]
/// fn InteractiveTab() -> impl IntoView {
///     let selected = RwSignal::new(false);
///
///     view! {
///         <Tab
///             active=selected.into()
///             on_click=Box::new(move || selected.set(true))
///         >
///             "Click Me"
///         </Tab>
///     }
/// }
/// ```
///
/// ## Disabled Tab
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::Tab;
///
/// view! {
///     <Tab disabled=Signal::derive(|| true)>
///         "Disabled Tab"
///     </Tab>
/// }
/// ```
///
/// ## Tab with Icon
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::Tab;
///
/// view! {
///     <Tab active=Signal::derive(|| false)>
///         <svg class="w-4 h-4 mr-2">/* home icon */</svg>
///         "Home"
///     </Tab>
/// }
/// ```
///
/// ## Tab Group Management
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, Tab};
///
/// #[component]
/// fn TabGroup() -> impl IntoView {
///     let active_tab = RwSignal::new(0);
///
///     let tabs = vec![
///         ("Dashboard", 0),
///         ("Analytics", 1),
///         ("Settings", 2),
///     ];
///
///     view! {
///         <Tabs>
///             {tabs.into_iter().map(|(label, index)| view! {
///                 <Tab
///                     active=Signal::derive(move || active_tab.get() == index)
///                     on_click=Box::new(move || active_tab.set(index))
///                 >
///                     {label}
///                 </Tab>
///             }).collect::<Vec<_>>()}
///         </Tabs>
///     }
/// }
/// ```
#[component]
pub fn Tab(
    /// Whether this tab is currently active
    #[prop(optional, into)]
    active: Signal<bool>,
    /// Whether this tab is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the anchor element
    #[prop(optional)]
    node_ref: NodeRef<A>,
    /// Optional click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn()>>,
    /// Tab label content
    children: Children,
) -> impl IntoView {
    view! {
        <a
            node_ref=node_ref
            class=move || merge_classes!("tab", class)
            class:tab-active=active
            class:tab-disabled=disabled
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </a>
    }
}

/// A radio input-based tab for form integration and accessibility.
///
/// Creates a tab using a radio input element, providing better accessibility
/// and form integration. Ideal for tab groups that need to maintain state
/// or be part of form submissions.
///
/// # Props
///
/// - `name` - Radio group name (tabs with same name are mutually exclusive)
/// - `checked` - Whether this radio tab is currently selected
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<input>` element
/// - `on_change` - Optional change handler that receives the new checked state
/// - `children` - Tab label content
///
/// # Behavior
///
/// - Radio buttons with the same `name` are mutually exclusive
/// - Native keyboard navigation (arrow keys between tabs)
/// - Form integration (value submitted with forms)
/// - Screen reader friendly with proper labeling
///
/// # Examples
///
/// ## Basic Radio Tabs
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, TabRadio};
///
/// view! {
///     <Tabs>
///         <TabRadio
///             name="view-mode"
///             checked=Signal::derive(|| true)
///         >
///             "List View"
///         </TabRadio>
///         <TabRadio name="view-mode">
///             "Grid View"
///         </TabRadio>
///         <TabRadio name="view-mode">
///             "Card View"
///         </TabRadio>
///     </Tabs>
/// }
/// ```
///
/// ## Radio Tabs with Change Handler
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, TabRadio};
///
/// #[component]
/// fn TabRadioExample() -> impl IntoView {
///     let selected_view = RwSignal::new("list".to_string());
///
///     view! {
///         <Tabs>
///             <TabRadio
///                 name="view-type"
///                 checked=Signal::derive(move || selected_view.get() == "list")
///                 on_change=Box::new(move |checked| {
///                     if checked {
///                         selected_view.set("list".to_string());
///                     }
///                 })
///             >
///                 "List"
///             </TabRadio>
///             <TabRadio
///                 name="view-type"
///                 checked=Signal::derive(move || selected_view.get() == "grid")
///                 on_change=Box::new(move |checked| {
///                     if checked {
///                         selected_view.set("grid".to_string());
///                     }
///                 })
///             >
///                 "Grid"
///             </TabRadio>
///         </Tabs>
///     }
/// }
/// ```
///
/// ## Form Integration
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, TabRadio};
///
/// view! {
///     <form>
///         <fieldset>
///             <legend>"Report Type"</legend>
///             <Tabs>
///                 <TabRadio name="report-type" checked=Signal::derive(|| true)>
///                     "Monthly"
///                 </TabRadio>
///                 <TabRadio name="report-type">
///                     "Quarterly"
///                 </TabRadio>
///                 <TabRadio name="report-type">
///                     "Annual"
///                 </TabRadio>
///             </Tabs>
///         </fieldset>
///         <button type="submit" class="btn btn-primary">"Generate Report"</button>
///     </form>
/// }
/// ```
///
/// ## Settings Tab Group
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Tabs, TabRadio, TabVariant};
///
/// view! {
///     <Tabs variant=TabVariant::Boxed>
///         <TabRadio
///             name="settings-section"
///             checked=Signal::derive(|| true)
///         >
///             <svg class="w-4 h-4 mr-2">/* general icon */</svg>
///             "General"
///         </TabRadio>
///         <TabRadio name="settings-section">
///             <svg class="w-4 h-4 mr-2">/* security icon */</svg>
///             "Security"
///         </TabRadio>
///         <TabRadio name="settings-section">
///             <svg class="w-4 h-4 mr-2">/* notifications icon */</svg>
///             "Notifications"
///         </TabRadio>
///     </Tabs>
/// }
/// ```
#[component]
pub fn TabRadio(
    /// Radio group name for mutual exclusion
    #[prop(optional)]
    name: Option<&'static str>,
    /// Whether this radio tab is selected
    #[prop(optional, into)]
    checked: Signal<bool>,
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the input element
    #[prop(optional)]
    node_ref: NodeRef<Input>,
    /// Optional change handler
    #[prop(optional)]
    on_change: Option<Box<dyn Fn(bool)>>,
    /// Tab label content
    children: Children,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="radio"
            name=name
            class=move || merge_classes!("tab", class)
            prop:checked=checked
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_checked(&ev));
                }
            }
        />
        {children()}
    }
}
