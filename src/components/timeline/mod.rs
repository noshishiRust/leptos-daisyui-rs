//! # Timeline Component
//!
//! The Timeline component displays a series of events or milestones in chronological order.
//! It's ideal for showing progress, history, or step-by-step processes with clear visual
//! indicators and flexible content placement.
//!
//! ## Components
//!
//! - [`Timeline`] - Main timeline container with direction and layout options
//! - [`TimelineItem`] - Individual timeline entry with position logic
//! - [`TimelineItemStart`] - Content positioned at the start of a timeline item
//! - [`TimelineItemMiddle`] - Content positioned in the middle (typically icons)
//! - [`TimelineItemEnd`] - Content positioned at the end of a timeline item
//!
//! ## Features
//!
//! - **Direction Control**: Vertical (default) or horizontal layouts
//! - **Icon Snapping**: Align icons to the timeline for consistent presentation
//! - **Compact Mode**: Reduced spacing for dense timelines
//! - **Flexible Positioning**: Smart content placement based on item position
//! - **Boxed Content**: Optional box styling for timeline content
//!
//! ## CSS Classes
//!
//! | Component | CSS Class | Description |
//! |-----------|-----------|-------------|
//! | `Timeline` | `timeline` | Base timeline container |
//! | `Timeline` (vertical) | `timeline-vertical` | Vertical direction (default) |
//! | `Timeline` (horizontal) | `timeline-horizontal` | Horizontal direction |
//! | `Timeline` (snap) | `timeline-snap-icon` | Snap icons to timeline |
//! | `Timeline` (compact) | `timeline-compact` | Compact spacing |
//! | Timeline item parts | `timeline-start` | Start content area |
//! | Timeline item parts | `timeline-middle` | Middle content area (icons) |
//! | Timeline item parts | `timeline-end` | End content area |
//! | Timeline item parts | `timeline-box` | Box styling for content |
//!
//! ## Usage
//!
//! ### Basic Vertical Timeline
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Timeline>
//!             <TimelineItem position=TimelineItemPosition::Start>
//!                 <TimelineItemStart>
//!                     <div class="timeline-box">"Project Started"</div>
//!                 </TimelineItemStart>
//!                 <TimelineItemMiddle>
//!                     <svg class="w-5 h-5 text-primary">
//!                         <circle cx="12" cy="12" r="8" fill="currentColor" />
//!                     </svg>
//!                 </TimelineItemMiddle>
//!             </TimelineItem>
//!             
//!             <TimelineItem position=TimelineItemPosition::End>
//!                 <TimelineItemMiddle>
//!                     <svg class="w-5 h-5 text-success">
//!                         <path d="M9 12l2 2 4-4" stroke="currentColor" fill="none" />
//!                     </svg>
//!                 </TimelineItemMiddle>
//!                 <TimelineItemEnd>
//!                     <div class="timeline-box">"Milestone Reached"</div>
//!                 </TimelineItemEnd>
//!             </TimelineItem>
//!         </Timeline>
//!     }
//! }
//! ```
//!
//! ### Horizontal Timeline
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let direction = RwSignal::new(TimelineDirection::Horizontal);
//!
//!     view! {
//!         <Timeline direction=direction>
//!             <TimelineItem position=TimelineItemPosition::Start>
//!                 <TimelineItemStart>
//!                     <div class="timeline-box">"Q1 2024"</div>
//!                 </TimelineItemStart>
//!                 <TimelineItemMiddle>
//!                     <div class="timeline-box bg-primary text-primary-content">"1"</div>
//!                 </TimelineItemMiddle>
//!             </TimelineItem>
//!             
//!             <TimelineItem position=TimelineItemPosition::Between>
//!                 <TimelineItemMiddle>
//!                     <div class="timeline-box bg-secondary text-secondary-content">"2"</div>
//!                 </TimelineItemMiddle>
//!             </TimelineItem>
//!         </Timeline>
//!     }
//! }
//! ```
//!
//! ### With Boxed Content and Icons
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let snap_icons = RwSignal::new(true);
//!     let compact = RwSignal::new(true);
//!
//!     view! {
//!         <Timeline snap_icon=snap_icons compact=compact>
//!             <TimelineItem position=TimelineItemPosition::Start>
//!                 <TimelineItemStart boxed=true>
//!                     <time class="font-mono italic">"2024-01-01"</time>
//!                     <div class="text-lg font-black">"Launch Day"</div>
//!                     "Application goes live to users"</n//!                 </TimelineItemStart>
//!                 <TimelineItemMiddle>
//!                     <svg class="w-5 h-5">
//!                         <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" fill="currentColor" />
//!                     </svg>
//!                 </TimelineItemMiddle>
//!             </TimelineItem>
//!         </Timeline>
//!     }
//! }
//! ```
//!
//! ## Position Logic
//!
//! The `TimelineItemPosition` enum controls the visual structure:
//!
//! - `Start` - First item, shows connector line after
//! - `End` - Last item, shows connector line before
//! - `Between` - Middle items, shows connector lines before and after
//!
//! ## Accessibility
//!
//! - Use semantic time elements for dates and timestamps
//! - Provide descriptive text for timeline events
//! - Consider ARIA labels for complex timeline interactions
//! - Ensure sufficient color contrast for timeline indicators
//! - Test keyboard navigation for interactive elements
//!
//! ## Best Practices
//!
//! - Keep timeline content concise and scannable
//! - Use consistent icon styles throughout the timeline
//! - Consider chronological order for time-based content
//! - Provide clear visual hierarchy between events
//! - Use appropriate spacing for content density
//! - Test responsive behavior on different screen sizes

mod component;
mod style;

pub use component::*;
pub use style::*;
