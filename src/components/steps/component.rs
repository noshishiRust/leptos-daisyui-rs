use super::style::{StepColor, StepsDirection};
use crate::merge_classes;
use leptos::{
    html::{Li, Ul},
    prelude::*,
};

/// A container component for displaying a sequence of numbered steps or progress indicators.
///
/// The `Steps` component creates a visual representation of a multi-step process,
/// allowing users to understand their current position and overall progress.
/// It supports both horizontal and vertical layouts to fit different design needs.
///
/// # Props
///
/// - `direction` - Optional steps orientation (default: `StepsDirection::Horizontal`)
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the ul element
/// - `children` - Step components
///
/// # CSS Classes
///
/// - Base: `steps`
/// - Vertical: `steps-vertical` (when direction is `StepsDirection::Vertical`)
///
/// # Examples
///
/// ## Horizontal Progress Steps
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
///             <Step color=StepColor::Primary data_content="3">"Choose Plan"</Step>
///             <Step data_content="4">"Payment"</Step>
///             <Step data_content="5">"Complete"</Step>
///         </Steps>
///     }
/// }
/// ```
///
/// ## Vertical Process Steps
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let is_vertical = RwSignal::new(StepsDirection::Vertical);
///
///     view! {
///         <div class="w-64">
///             <Steps direction=is_vertical>
///                 <Step color=StepColor::Success data_content="1">"Project Setup"</Step>
///                 <Step color=StepColor::Success data_content="2">"Development"</Step>
///                 <Step color=StepColor::Info data_content="3">"Testing"</Step>
///                 <Step data_content="4">"Deployment"</Step>
///             </Steps>
///         </div>
///     }
/// }
/// ```
#[component]
pub fn Steps(
    /// Direction of the steps layout (horizontal or vertical)
    #[prop(optional, into)]
    direction: Signal<StepsDirection>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the ul element
    #[prop(optional)]
    node_ref: NodeRef<Ul>,
    /// Step components
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "steps",
                direction.get().as_str(),
                class
                )
            }
        >
            {children()}
        </ul>
    }
}

/// An individual step component representing a single stage in a process.
///
/// The `Step` component displays a single step with customizable appearance
/// and content. It supports color variants to indicate different states and
/// optional data content for step numbering or symbols.
///
/// # Props
///
/// - `color` - Optional color variant indicating step state
/// - `data_content` - Optional content for step indicator (numbers, symbols, checkmarks)
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the li element
/// - `children` - Step content (label, description)
///
/// # CSS Classes
///
/// - Base: `step`
/// - Color variants: `step-primary`, `step-success`, `step-error`, etc.
///
/// # Data Content
///
/// The `data_content` prop allows customization of the step indicator:
/// - Numbers: `"1"`, `"2"`, `"3"`
/// - Checkmarks: `"✓"`, `"✔"`
/// - Symbols: `"★"`, `"●"`, `"►"`
/// - Letters: `"A"`, `"B"`, `"C"`
///
/// # Examples
///
/// ## Numbered Steps
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Steps>
///             <Step color=StepColor::Primary data_content="1">"Setup Account"</Step>
///             <Step color=StepColor::Primary data_content="2">"Verify Email"</Step>
///             <Step data_content="3">"Complete Profile"</Step>
///         </Steps>
///     }
/// }
/// ```
///
/// ## Status Indicators
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Steps>
///             <Step color=StepColor::Success data_content="✓">"Payment Processed"</Step>
///             <Step color=StepColor::Warning data_content="!">"Pending Review"</Step>
///             <Step color=StepColor::Error data_content="×">"Failed Validation"</Step>
///         </Steps>
///     }
/// }
/// ```
///
/// ## Rich Content Steps
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Steps>
///             <Step color=StepColor::Success>
///                 <div class="flex flex-col">
///                     <span class="font-bold">"Order Placed"</span>
///                     <span class="text-sm text-gray-500">"March 15, 2024"</span>
///                 </div>
///             </Step>
///             <Step color=StepColor::Primary>
///                 <div class="flex flex-col">
///                     <span class="font-bold">"Processing"</span>
///                     <span class="text-sm text-gray-500">"Estimated 2-3 days"</span>
///                 </div>
///             </Step>
///         </Steps>
///     }
/// }
/// ```
#[component]
pub fn Step(
    /// Color variant indicating step state or importance
    #[prop(optional, into)]
    color: Signal<StepColor>,
    /// Optional content for the step indicator (numbers, symbols, etc.)
    #[prop(optional, into)]
    data_content: Option<&'static str>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the li element
    #[prop(optional)]
    node_ref: NodeRef<Li>,
    /// Step content (label, description, or rich content)
    children: Children,
) -> impl IntoView {
    view! {
        <li
            node_ref=node_ref
            class=move || {
                merge_classes!("step",
                color.get().as_str(),
                class)
            }
            data-content=data_content
        >
            {children()}
        </li>
    }
}
