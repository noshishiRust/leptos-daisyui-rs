//! # Divider Component
//!
//! The Divider component provides visual separation between sections of content.
//! It supports both horizontal and vertical orientations with optional text labels
//! and various color themes for different design contexts.
//!
//! ## Components
//!
//! - [`Divider`] - Flexible divider with direction, color, and content options
//!
//! ## Features
//!
//! - **Direction Control**: Horizontal (default) or vertical orientation
//! - **Color Variants**: Semantic color options for different contexts
//! - **Text Labels**: Optional text content within the divider
//! - **Placement Options**: Start, center, or end text positioning
//! - **Responsive Design**: Adapts well to different container sizes
//!
//! ## CSS Classes
//!
//! | Component | CSS Class | Description |
//! |-----------|-----------|-------------|
//! | `Divider` | `divider` | Base divider styling |
//! | Direction | `divider-horizontal` | Horizontal divider (default) |
//! | Direction | `divider-vertical` | Vertical divider |
//! | Colors | `divider-neutral` | Neutral color variant |
//! | Colors | `divider-primary` | Primary color variant |
//! | Colors | `divider-secondary` | Secondary color variant |
//! | Colors | `divider-accent` | Accent color variant |
//! | Colors | `divider-success` | Success color variant |
//! | Colors | `divider-warning` | Warning color variant |
//! | Colors | `divider-info` | Info color variant |
//! | Colors | `divider-error` | Error color variant |
//! | Placement | `divider-start` | Text aligned to start |
//! | Placement | `divider-end` | Text aligned to end |
//!
//! ## Usage
//!
//! ### Basic Horizontal Divider
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <div>
//!             <p>"Content above the divider"</p>
//!             <Divider />
//!             <p>"Content below the divider"</p>
//!         </div>
//!     }
//! }
//! ```
//!
//! ### Divider with Text Label
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <div>
//!             <div class="card">
//!                 <h2>"User Information"</h2>
//!                 <p>"Name: John Doe"</p>
//!                 <p>"Email: john@example.com"</p>
//!             </div>
//!             
//!             <Divider color=DividerColor::Primary>
//!                 "OR"
//!             </Divider>
//!             
//!             <div class="card">
//!                 <h2>"Guest Access"</h2>
//!                 <p>"Continue without account"</p>
//!             </div>
//!         </div>
//!     }
//! }
//! ```
//!
//! ### Vertical Divider
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let direction = RwSignal::new(DividerDirection::Vertical);
//!
//!     view! {
//!         <div class="flex h-32">
//!             <div class="flex-1 p-4">
//!                 "Left content area"
//!             </div>
//!             
//!             <Divider direction=direction />
//!             
//!             <div class="flex-1 p-4">
//!                 "Right content area"
//!             </div>
//!         </div>
//!     }
//! }
//! ```
//!
//! ### Colored Dividers with Placement
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <div class="space-y-4">
//!             <Divider
//!                 color=DividerColor::Success
//!                 placement=DividerPlacement::Start
//!             >
//!                 "Completed Tasks"
//!             </Divider>
//!             
//!             <Divider
//!                 color=DividerColor::Warning
//!                 placement=DividerPlacement::End
//!             >
//!                 "Pending Review"
//!             </Divider>
//!             
//!             <Divider color=DividerColor::Error>
//!                 "Critical Issues"
//!             </Divider>
//!         </div>
//!     }
//! }
//! ```
//!
//! ### Section Separators
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <article>
//!             <section>
//!                 <h2>"Introduction"</h2>
//!                 <p>"Article introduction content..."</p>
//!             </section>
//!             
//!             <Divider color=DividerColor::Neutral>
//!                 "Main Content"
//!             </Divider>
//!             
//!             <section>
//!                 <h2>"Chapter 1"</h2>
//!                 <p>"Main article content..."</p>
//!             </section>
//!             
//!             <Divider color=DividerColor::Neutral />
//!             
//!             <section>
//!                 <h2>"Conclusion"</h2>
//!                 <p>"Article conclusion..."</p>
//!             </section>
//!         </article>
//!     }
//! }
//! ```
//!
//! ## Design Considerations
//!
//! - Use horizontal dividers to separate content sections vertically
//! - Use vertical dividers to separate content areas horizontally
//! - Choose colors that complement your content and maintain readability
//! - Keep divider text concise and meaningful
//! - Consider spacing around dividers for proper visual hierarchy
//!
//! ## Accessibility
//!
//! - Dividers are primarily visual and don't require special accessibility consideration
//! - Ensure sufficient color contrast when using colored dividers
//! - Use semantic HTML structure rather than relying solely on visual dividers
//! - Consider whether divider text needs to be announced by screen readers
//!
//! ## Best Practices
//!
//! - Use dividers sparingly to avoid visual clutter
//! - Maintain consistent divider styling within the same context
//! - Consider whitespace as an alternative to dividers when appropriate
//! - Test responsive behavior, especially with vertical dividers
//! - Ensure divider colors align with your design system

mod component;
mod style;

pub use component::*;
pub use style::*;
