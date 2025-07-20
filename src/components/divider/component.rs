use super::style::{DividerColor, DividerDirection, DividerPlacement};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// A visual separator component for dividing content sections.
///
/// The `Divider` component provides a flexible way to separate content with optional
/// text labels and various styling options. It supports both horizontal and vertical
/// orientations, multiple color variants, and customizable text placement.
///
/// # Props
///
/// - `color` - Optional color variant for the divider (default: theme default)
/// - `direction` - Optional divider orientation (default: `DividerDirection::Horizontal`)
/// - `placement` - Optional text placement within the divider (default: center)
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - Optional text content to display within the divider
///
/// # CSS Classes
///
/// - Base: `divider`
/// - Direction: `divider-horizontal` or `divider-vertical`
/// - Colors: `divider-primary`, `divider-success`, etc.
/// - Placement: `divider-start` or `divider-end`
///
/// # Examples
///
/// ## Simple Horizontal Divider
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <div>
///             <section>
///                 <h2>"First Section"</h2>
///                 <p>"Content of the first section..."</p>
///             </section>
///             
///             <Divider />
///             
///             <section>
///                 <h2>"Second Section"</h2>
///                 <p>"Content of the second section..."</p>
///             </section>
///         </div>
///     }
/// }
/// ```
///
/// ## Divider with Text Label
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <div>
///             <div class="card">
///                 <h3>"Login with Account"</h3>
///                 // Login form content
///             </div>
///             
///             <Divider color=DividerColor::Primary>
///                 "OR"
///             </Divider>
///             
///             <div class="card">
///                 <h3>"Continue as Guest"</h3>
///                 // Guest options
///             </div>
///         </div>
///     }
/// }
/// ```
///
/// ## Vertical Divider
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let vertical = RwSignal::new(DividerDirection::Vertical);
///
///     view! {
///         <div class="flex h-64">
///             <div class="flex-1 p-4">
///                 <h3>"Left Panel"</h3>
///                 <p>"Content for the left side..."</p>
///             </div>
///             
///             <Divider direction=vertical />
///             
///             <div class="flex-1 p-4">
///                 <h3>"Right Panel"</h3>
///                 <p>"Content for the right side..."</p>
///             </div>
///         </div>
///     }
/// }
/// ```
///
/// ## Semantic Color Dividers
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <div class="space-y-6">
///             <section>
///                 <h2>"Task Status"</h2>
///                 
///                 <Divider
///                     color=DividerColor::Success
///                     placement=DividerPlacement::Start
///                 >
///                     "Completed (5)"
///                 </Divider>
///                 // Completed tasks list
///                 
///                 <Divider
///                     color=DividerColor::Warning
///                     placement=DividerPlacement::Start
///                 >
///                     "In Progress (3)"
///                 </Divider>
///                 // In progress tasks list
///                 
///                 <Divider
///                     color=DividerColor::Error
///                     placement=DividerPlacement::Start
///                 >
///                     "Blocked (1)"
///                 </Divider>
///                 // Blocked tasks list
///             </section>
///         </div>
///     }
/// }
/// ```
///
/// ## Custom Placement
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
///             <Divider placement=DividerPlacement::End>
///                 "Right-aligned label"
///             </Divider>
///             
///             <Divider>
///                 "Center-aligned label (default)"
///             </Divider>
///         </div>
///     }
/// }
/// ```
#[component]
pub fn Divider(
    /// Color variant for the divider styling
    #[prop(optional, into)]
    color: Signal<DividerColor>,
    /// Direction of the divider (horizontal or vertical)
    #[prop(optional, into)]
    direction: Signal<DividerDirection>,
    /// Text placement within the divider
    #[prop(optional, into)]
    placement: Signal<DividerPlacement>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Optional text content to display within the divider
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "divider",
                color.get().as_str(),
                direction.get().as_str(),
                placement.get().as_str(),
                class
                )
            }
        >
            {children.map(|c| c())}
        </div>
    }
}
