use super::style::{TimelineDirection, TimelineItemPosition};
use crate::merge_classes;
use leptos::{
    html::{Div, Li, Ul},
    prelude::*,
};

/// A Timeline component that displays a series of events or milestones in chronological order.
///
/// The `Timeline` component creates a structured layout for displaying sequential events,
/// progress steps, or historical data with clear visual flow and flexible content positioning.
/// It supports both vertical and horizontal orientations with various styling options.
///
/// This component renders as a `<ul>` element with proper semantic structure.
///
/// # Props
///
/// - `direction` - Optional timeline orientation (default: `TimelineDirection::Vertical`)
/// - `snap_icon` - Optional icon alignment to timeline for consistent positioning
/// - `compact` - Optional reduced spacing for dense timeline layouts
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the ul element
/// - `children` - Timeline content (typically `TimelineItem` components)
///
/// # CSS Classes
///
/// - Base: `timeline`
/// - Direction: `timeline-vertical` or `timeline-horizontal`
/// - Icon snap: `timeline-snap-icon` (when snap_icon is true)
/// - Compact: `timeline-compact` (when compact is true)
///
/// # Examples
///
/// ## Basic Vertical Timeline
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Timeline>
///             <TimelineItem position=TimelineItemPosition::Start>
///                 <TimelineItemStart>
///                     <div class="text-xl font-bold">"Project Kickoff"</div>
///                     <time class="font-mono italic">"2024-01-01"</time>
///                 </TimelineItemStart>
///                 <TimelineItemMiddle>
///                     <svg class="w-5 h-5 text-primary">
///                         <circle cx="10" cy="10" r="8" fill="currentColor" />
///                     </svg>
///                 </TimelineItemMiddle>
///             </TimelineItem>
///             
///             <TimelineItem position=TimelineItemPosition::End>
///                 <TimelineItemMiddle>
///                     <svg class="w-5 h-5 text-success">
///                         <path d="M9 12l2 2 4-4" stroke="currentColor" fill="none" />
///                     </svg>
///                 </TimelineItemMiddle>
///                 <TimelineItemEnd>
///                     <div class="text-xl font-bold">"Project Complete"</div>
///                     <time class="font-mono italic">"2024-06-30"</time>
///                 </TimelineItemEnd>
///             </TimelineItem>
///         </Timeline>
///     }
/// }
/// ```
///
/// ## Horizontal Compact Timeline
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let is_horizontal = RwSignal::new(TimelineDirection::Horizontal);
///     let is_compact = RwSignal::new(true);
///     let snap_icons = RwSignal::new(true);
///
///     view! {
///         <Timeline
///             direction=is_horizontal
///             compact=is_compact
///             snap_icon=snap_icons
///         >
///             <TimelineItem position=TimelineItemPosition::Start>
///                 <TimelineItemStart>
///                     <div class="timeline-box">"Q1"</div>
///                 </TimelineItemStart>
///                 <TimelineItemMiddle>
///                     <div class="timeline-box bg-primary text-primary-content">"1"</div>
///                 </TimelineItemMiddle>
///             </TimelineItem>
///             
///             <TimelineItem position=TimelineItemPosition::Between>
///                 <TimelineItemMiddle>
///                     <div class="timeline-box bg-secondary text-secondary-content">"2"</div>
///                 </TimelineItemMiddle>
///             </TimelineItem>
///         </Timeline>
///     }
/// }
/// ```
#[component]
pub fn Timeline(
    /// Timeline orientation (vertical or horizontal)
    #[prop(optional, into)]
    direction: Signal<TimelineDirection>,
    /// Snap icons to timeline for consistent alignment
    #[prop(optional, into)]
    snap_icon: Signal<bool>,
    /// Use compact spacing for dense layouts
    #[prop(optional, into)]
    compact: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the ul element
    #[prop(optional)]
    node_ref: NodeRef<Ul>,
    /// Timeline content (TimelineItem components)
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "timeline",
                    direction.get().as_str(),
                    class
                )
            }
            class:timeline-snap-icon=snap_icon
            class:timeline-compact=compact
        >
            {children()}
        </ul>
    }
}

/// An individual timeline item with intelligent connector line management.
///
/// The `TimelineItem` component represents a single event or milestone in the timeline.
/// It automatically manages connector lines based on the item's position, ensuring
/// proper visual flow between timeline events.
///
/// # Props
///
/// - `position` - Required position in timeline (affects connector line display)
/// - `class` - Optional additional CSS classes for the list item
/// - `start_class` - Optional CSS classes for the starting connector line
/// - `end_class` - Optional CSS classes for the ending connector line
/// - `node_ref` - Optional node reference for the li element
/// - `children` - Timeline item content (typically Start, Middle, End components)
///
/// # Connector Line Logic
///
/// The component automatically renders `<hr>` elements based on position:
/// - `Start` position: Shows ending connector only
/// - `End` position: Shows starting connector only
/// - `Between` position: Shows both starting and ending connectors
///
/// # Examples
///
/// ## First Timeline Item
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Timeline>
///             <TimelineItem position=TimelineItemPosition::Start>
///                 <TimelineItemStart>
///                     <div class="text-xl font-bold">"Starting Event"</div>
///                     <div class="text-lg">"This is where it all began"</div>
///                 </TimelineItemStart>
///                 <TimelineItemMiddle>
///                     <svg class="w-5 h-5 text-primary">
///                         <circle cx="10" cy="10" r="8" fill="currentColor" />
///                     </svg>
///                 </TimelineItemMiddle>
///             </TimelineItem>
///         </Timeline>
///     }
/// }
/// ```
///
/// ## Middle Timeline Item with Custom Connector Styling
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Timeline>
///             <TimelineItem
///                 position=TimelineItemPosition::Between
///                 start_class="bg-primary"
///                 end_class="bg-secondary"
///             >
///                 <TimelineItemStart>
///                     <time class="font-mono italic">"2024-03-15"</time>
///                     <div class="text-xl font-bold">"Milestone Reached"</div>
///                 </TimelineItemStart>
///                 <TimelineItemMiddle>
///                     <svg class="w-5 h-5 text-success">
///                         <path d="M9 12l2 2 4-4" stroke="currentColor" fill="none" />
///                     </svg>
///                 </TimelineItemMiddle>
///                 <TimelineItemEnd>
///                     <div class="text-lg">"Successfully completed phase 2"</div>
///                 </TimelineItemEnd>
///             </TimelineItem>
///         </Timeline>
///     }
/// }
/// ```
#[component]
pub fn TimelineItem(
    /// Position in timeline (affects which connector lines are shown)
    #[prop(into)]
    position: Signal<TimelineItemPosition>,
    /// Additional CSS classes for the list item
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// CSS classes for the starting connector line (hr element)
    #[prop(optional, into)]
    start_class: &'static str,
    /// CSS classes for the ending connector line (hr element)
    #[prop(optional, into)]
    end_class: &'static str,
    /// Node reference for the li element
    #[prop(optional)]
    node_ref: NodeRef<Li>,
    /// Timeline item content
    children: Children,
) -> impl IntoView {
    view! {
        <li node_ref=node_ref class=class>
            {move || {
                let position = position.get();
                if position.is_end() || position.is_between() {
                    view! { <hr class=start_class /> }.into_any()
                } else {
                    ().into_any()
                }
            }}

            {children()}

            {move || {
                let position = position.get();
                if position.is_start() || position.is_between() {
                    view! { <hr class=end_class /> }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </li>
    }
}

/// Content positioned at the start of a timeline item.
///
/// The `TimelineItemStart` component positions content at the beginning of a timeline item.
/// In vertical timelines, this is typically on the left side; in horizontal timelines,
/// this is usually above the timeline line.
///
/// # Props
///
/// - `boxed` - Optional box styling with background and padding
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - Start content (text, dates, descriptions)
///
/// # CSS Classes
///
/// - Base: `timeline-start`
/// - Box styling: `timeline-box` (when boxed is true)
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
///         <TimelineItem position=TimelineItemPosition::Start>
///             <TimelineItemStart boxed=true>
///                 <time class="font-mono italic">"1984"</time>
///                 <div class="text-lg font-black">"First Macintosh"</div>
///                 "Apple Computer introduces the Macintosh personal computer"
///             </TimelineItemStart>
///             <TimelineItemMiddle>
///                 <svg class="w-5 h-5 text-primary">
///                     <path d="..." fill="currentColor" />
///                 </svg>
///             </TimelineItemMiddle>
///         </TimelineItem>
///     }
/// }
/// ```
#[component]
pub fn TimelineItemStart(
    /// Apply box styling with background and padding
    #[prop(optional, into)]
    boxed: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Start content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("timeline-start", class)
            class:timeline-box=boxed
        >
            {children()}
        </div>
    }
}

/// Content positioned in the middle of a timeline item, typically for icons.
///
/// The `TimelineItemMiddle` component positions content at the center of the timeline,
/// directly on the timeline line. This is commonly used for icons, badges, or
/// numbered indicators that mark specific timeline events.
///
/// # Props
///
/// - `boxed` - Optional box styling with background and padding
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - Middle content (icons, badges, numbers)
///
/// # CSS Classes
///
/// - Base: `timeline-middle`
/// - Box styling: `timeline-box` (when boxed is true)
///
/// # Examples
///
/// ## Icon Indicator
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <TimelineItem position=TimelineItemPosition::Between>
///             <TimelineItemMiddle>
///                 <svg class="w-5 h-5 text-success">
///                     <path d="M9 12l2 2 4-4" stroke="currentColor" fill="none" stroke-width="2" />
///                 </svg>
///             </TimelineItemMiddle>
///         </TimelineItem>
///     }
/// }
/// ```
///
/// ## Numbered Badge
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <TimelineItem position=TimelineItemPosition::Between>
///             <TimelineItemMiddle boxed=true>
///                 <div class="bg-primary text-primary-content rounded-full w-8 h-8 flex items-center justify-center font-bold">
///                     "3"
///                 </div>
///             </TimelineItemMiddle>
///         </TimelineItem>
///     }
/// }
/// ```
#[component]
pub fn TimelineItemMiddle(
    /// Apply box styling with background and padding
    #[prop(optional, into)]
    boxed: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Middle content (typically icons or indicators)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("timeline-middle", class)
            class:timeline-box=boxed
        >
            {children()}
        </div>
    }
}

/// Content positioned at the end of a timeline item.
///
/// The `TimelineItemEnd` component positions content at the end of a timeline item.
/// In vertical timelines, this is typically on the right side; in horizontal timelines,
/// this is usually below the timeline line.
///
/// # Props
///
/// - `boxed` - Optional box styling with background and padding
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - End content (text, descriptions, additional details)
///
/// # CSS Classes
///
/// - Base: `timeline-end`
/// - Box styling: `timeline-box` (when boxed is true)
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
///         <TimelineItem position=TimelineItemPosition::End>
///             <TimelineItemMiddle>
///                 <svg class="w-5 h-5 text-error">
///                     <circle cx="10" cy="10" r="8" fill="currentColor" />
///                 </svg>
///             </TimelineItemMiddle>
///             <TimelineItemEnd boxed=true>
///                 <time class="font-mono italic">"2023-12-31"</time>
///                 <div class="text-lg font-black">"Project Deadline"</div>
///                 "Final deliverables must be submitted by end of day"
///             </TimelineItemEnd>
///         </TimelineItem>
///     }
/// }
/// ```
#[component]
pub fn TimelineItemEnd(
    /// Apply box styling with background and padding
    #[prop(optional, into)]
    boxed: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// End content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("timeline-end", class)
            class:timeline-box=boxed
        >
            {children()}
        </div>
    }
}
