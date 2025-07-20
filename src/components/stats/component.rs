use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// A container component for displaying statistical data in a card-based layout.
///
/// The `Stats` component creates a flexible container for organizing and displaying
/// key metrics, analytics, or important numbers. It supports both horizontal (default)
/// and vertical layouts to accommodate different design needs.
///
/// # Props
///
/// - `vertical` - Optional signal to toggle vertical layout (default: false)
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - Child components (typically `Stat` components)
///
/// # CSS Classes
///
/// - Base: `stats`
/// - Vertical modifier: `stats-vertical` (when `vertical` is true)
///
/// # Examples
///
/// ## Basic Horizontal Stats
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stats>
///             <Stat>
///                 <StatTitle>"Total Sales"</StatTitle>
///                 <StatValue>"$12,345"</StatValue>
///                 <StatDesc>"This month"</StatDesc>
///             </Stat>
///             <Stat>
///                 <StatTitle>"Growth"</StatTitle>
///                 <StatValue>"+15.3%"</StatValue>
///                 <StatDesc>"Compared to last month"</StatDesc>
///             </Stat>
///         </Stats>
///     }
/// }
/// ```
///
/// ## Vertical Layout
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let is_vertical = RwSignal::new(true);
///
///     view! {
///         <Stats vertical=is_vertical>
///             <Stat>
///                 <StatTitle>"Active Users"</StatTitle>
///                 <StatValue>"2,847"</StatValue>
///             </Stat>
///             <Stat>
///                 <StatTitle>"Conversion Rate"</StatTitle>
///                 <StatValue>"3.2%"</StatValue>
///             </Stat>
///         </Stats>
///     }
/// }
/// ```
#[component]
pub fn Stats(
    /// Toggle vertical layout (default: false for horizontal layout)
    #[prop(optional, into)]
    vertical: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Child components (typically Stat components)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("stats", class)
            class:stats-vertical=vertical
        >
            {children()}
        </div>
    }
}

/// An individual statistic item within a Stats container.
///
/// The `Stat` component represents a single statistical data point and serves as
/// a container for the various stat components (title, value, description, etc.).
/// Each stat item is visually separated and can contain multiple semantic parts.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - Child components (StatTitle, StatValue, StatDesc, etc.)
///
/// # CSS Classes
///
/// - Base: `stat`
///
/// # Examples
///
/// ## Complete Stat with All Parts
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stats>
///             <Stat>
///                 <StatFigure>
///                     <svg class="w-8 h-8 fill-current">
///                         <path d="..." />
///                     </svg>
///                 </StatFigure>
///                 <StatTitle>"Total Downloads"</StatTitle>
///                 <StatValue>"31K"</StatValue>
///                 <StatDesc>"Jan 1st - Feb 1st"</StatDesc>
///                 <StatActions>
///                     <Button size=ButtonSize::Sm>"View Details"</Button>
///                 </StatActions>
///             </Stat>
///         </Stats>
///     }
/// }
/// ```
///
/// ## Minimal Stat
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stats>
///             <Stat>
///                 <StatValue>"42"</StatValue>
///                 <StatTitle>"Active Projects"</StatTitle>
///             </Stat>
///         </Stats>
///     }
/// }
/// ```
#[component]
pub fn Stat(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Child components (StatTitle, StatValue, StatDesc, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat", class)>
            {children()}
        </div>
    }
}

/// A title/label component for a statistic.
///
/// The `StatTitle` component displays the label or name for a statistic.
/// It provides context for what the stat value represents and is typically
/// displayed above or alongside the stat value.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - Content for the title (typically text)
///
/// # CSS Classes
///
/// - Base: `stat-title`
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
///         <Stat>
///             <StatTitle>"Monthly Revenue"</StatTitle>
///             <StatValue>"$24,500"</StatValue>
///         </Stat>
///     }
/// }
/// ```
#[component]
pub fn StatTitle(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Title content
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-title", class)>
            {children()}
        </div>
    }
}

/// The primary value display component for a statistic.
///
/// The `StatValue` component displays the main numerical or textual value
/// of a statistic. This is typically the most prominent element within a stat
/// and is styled to draw attention as the key data point.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - The value content (numbers, text, formatted data)
///
/// # CSS Classes
///
/// - Base: `stat-value`
///
/// # Examples
///
/// ## Numeric Value
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stat>
///             <StatTitle>"Total Users"</StatTitle>
///             <StatValue>"1,234"</StatValue>
///         </Stat>
///     }
/// }
/// ```
///
/// ## Formatted Value with Units
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stat>
///             <StatTitle>"Response Time"</StatTitle>
///             <StatValue>"1.2ms"</StatValue>
///             <StatDesc>"Average over 24h"</StatDesc>
///         </Stat>
///     }
/// }
/// ```
#[component]
pub fn StatValue(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// The primary value content
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-value", class)>
            {children()}
        </div>
    }
}

/// A description component providing additional context for a statistic.
///
/// The `StatDesc` component displays supplementary information about a statistic,
/// such as time periods, comparisons, trends, or explanatory text. It helps
/// provide context and meaning to the stat value.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - Description content (text, trends, context)
///
/// # CSS Classes
///
/// - Base: `stat-desc`
///
/// # Examples
///
/// ## Time Period Context
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stat>
///             <StatTitle>"Sales"</StatTitle>
///             <StatValue>"$12,345"</StatValue>
///             <StatDesc>"January 1 - January 31"</StatDesc>
///         </Stat>
///     }
/// }
/// ```
///
/// ## Trend Information
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stat>
///             <StatTitle>"Growth Rate"</StatTitle>
///             <StatValue>"+12.5%"</StatValue>
///             <StatDesc class="text-green-600">"↗︎ +2.3% from last month"</StatDesc>
///         </Stat>
///     }
/// }
/// ```
#[component]
pub fn StatDesc(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Description content
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-desc", class)>
            {children()}
        </div>
    }
}

/// A figure component for displaying icons, images, or visual elements in a statistic.
///
/// The `StatFigure` component provides a designated area for visual elements that
/// complement the statistic, such as icons, avatars, charts, or other graphics.
/// It's typically positioned prominently within the stat layout.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - Visual content (icons, images, charts)
///
/// # CSS Classes
///
/// - Base: `stat-figure`
///
/// # Examples
///
/// ## With Icon
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stat>
///             <StatFigure>
///                 <svg class="w-8 h-8 fill-current text-primary">
///                     <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z" />
///                 </svg>
///             </StatFigure>
///             <StatTitle>"Active Sessions"</StatTitle>
///             <StatValue>"1,247"</StatValue>
///         </Stat>
///     }
/// }
/// ```
///
/// ## With Avatar
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stat>
///             <StatFigure>
///                 <div class="avatar online">
///                     <div class="w-16 rounded-full">
///                         <img src="/avatar.jpg" alt="User avatar" />
///                     </div>
///                 </div>
///             </StatFigure>
///             <StatValue>"86%"</StatValue>
///             <StatTitle>"Tasks Complete"</StatTitle>
///         </Stat>
///     }
/// }
/// ```
#[component]
pub fn StatFigure(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Visual content (icons, images, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-figure", class)>
            {children()}
        </div>
    }
}

/// An actions container for interactive elements within a statistic.
///
/// The `StatActions` component provides a designated area for buttons, links,
/// or other interactive elements that allow users to take action related to
/// the displayed statistic. This enables stats to be more than just display
/// components by adding interactivity.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the div element
/// - `children` - Interactive content (buttons, links, etc.)
///
/// # CSS Classes
///
/// - Base: `stat-actions`
///
/// # Examples
///
/// ## With Action Button
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stat>
///             <StatTitle>"Pending Orders"</StatTitle>
///             <StatValue>"23"</StatValue>
///             <StatDesc>"Awaiting processing"</StatDesc>
///             <StatActions>
///                 <Button size=ButtonSize::Sm color=ButtonColor::Primary>
///                     "Process Orders"
///                 </Button>
///             </StatActions>
///         </Stat>
///     }
/// }
/// ```
///
/// ## Multiple Actions
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Stat>
///             <StatTitle>"Error Rate"</StatTitle>
///             <StatValue>"0.3%"</StatValue>
///             <StatActions>
///                 <Button size=ButtonSize::Xs color=ButtonColor::Error>
///                     "View Errors"
///                 </Button>
///                 <Button size=ButtonSize::Xs variant=ButtonVariant::Outline>
///                     "Export Log"
///                 </Button>
///             </StatActions>
///         </Stat>
///     }
/// }
/// ```
#[component]
pub fn StatActions(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Interactive content (buttons, links, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-actions", class)>
            {children()}
        </div>
    }
}
