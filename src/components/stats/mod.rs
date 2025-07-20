//! # Stats Component
//!
//! The Stats component displays statistical data in a card-based layout. It's designed to show key metrics,
//! analytics, or important numbers in a visually appealing way.
//!
//! ## Components
//!
//! - [`Stats`] - Container component for statistical displays
//! - [`Stat`] - Individual statistic item
//! - [`StatTitle`] - Title/label for a statistic
//! - [`StatValue`] - Primary value display
//! - [`StatDesc`] - Description or additional context
//! - [`StatFigure`] - Icon or image area
//! - [`StatActions`] - Action buttons area
//!
//! ## Features
//!
//! - **Layout Options**: Horizontal (default) or vertical arrangement
//! - **Flexible Structure**: Compose stats with multiple semantic parts
//! - **Visual Hierarchy**: Clear distinction between titles, values, and descriptions
//! - **Action Support**: Include buttons or interactive elements
//! - **Icon Integration**: Support for visual indicators and icons
//!
//! ## CSS Classes
//!
//! | Component | CSS Class | Description |
//! |-----------|-----------|-------------|
//! | `Stats` | `stats` | Container for stats layout |
//! | `Stats` (vertical) | `stats-vertical` | Vertical layout modifier |
//! | `Stat` | `stat` | Individual stat item |
//! | `StatTitle` | `stat-title` | Stat title/label |
//! | `StatValue` | `stat-value` | Primary value display |
//! | `StatDesc` | `stat-desc` | Description text |
//! | `StatFigure` | `stat-figure` | Icon/image container |
//! | `StatActions` | `stat-actions` | Actions container |
//!
//! ## Usage
//!
//! ### Basic Stats Display
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Stats>
//!             <Stat>
//!                 <StatTitle>"Total Page Views"</StatTitle>
//!                 <StatValue>"89,400"</StatValue>
//!                 <StatDesc>"21% more than last month"</StatDesc>
//!             </Stat>
//!             <Stat>
//!                 <StatTitle>"Total Users"</StatTitle>
//!                 <StatValue>"4,200"</StatValue>
//!                 <StatDesc>"↗︎ 400 (22%)"</StatDesc>
//!             </Stat>
//!         </Stats>
//!     }
//! }
//! ```
//!
//! ### With Icons and Actions
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Stats>
//!             <Stat>
//!                 <StatFigure>
//!                     <div class="avatar online">
//!                         <div class="w-16 rounded-full">
//!                             <img src="/images/stock/photo-1534528741775-53994a69daeb.jpg" />
//!                         </div>
//!                     </div>
//!                 </StatFigure>
//!                 <StatValue>"86%"</StatValue>
//!                 <StatTitle>"Tasks done"</StatTitle>
//!                 <StatDesc>"31 tasks remaining"</StatDesc>
//!                 <StatActions>
//!                     <Button size=ButtonSize::Sm color=ButtonColor::Success>"View tasks"</Button>
//!                 </StatActions>
//!             </Stat>
//!         </Stats>
//!     }
//! }
//! ```
//!
//! ### Vertical Layout
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let vertical = RwSignal::new(true);
//!
//!     view! {
//!         <Stats vertical>
//!             <Stat>
//!                 <StatTitle>"Downloads"</StatTitle>
//!                 <StatValue>"31K"</StatValue>
//!                 <StatDesc>"Jan 1st - Feb 1st"</StatDesc>
//!             </Stat>
//!             <Stat>
//!                 <StatTitle>"New Users"</StatTitle>
//!                 <StatValue>"4,200"</StatValue>
//!                 <StatDesc>"↗︎ 400 (22%)"</StatDesc>
//!             </Stat>
//!         </Stats>
//!     }
//! }
//! ```
//!
//! ## Accessibility
//!
//! - Use semantic markup with proper heading levels for stat titles
//! - Ensure sufficient color contrast for stat values and descriptions
//! - Consider screen reader announcements for dynamic stat updates
//! - Include descriptive text for icons and figures
//! - Maintain logical tab order for interactive elements
//!
//! ## Best Practices
//!
//! - Keep stat values concise and easy to scan
//! - Use consistent units and formatting across related stats
//! - Provide context with descriptions where helpful
//! - Consider using trend indicators (arrows, percentages) for comparisons
//! - Group related statistics together logically
//! - Use appropriate data visualization for complex metrics

mod component;

pub use component::*;
