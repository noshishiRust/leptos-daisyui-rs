//! # Steps Component
//!
//! The Steps component displays a sequence of numbered steps or progress indicators.
//! It's ideal for showing multi-step processes, progress tracking, or guided workflows
//! with clear visual progression.
//!
//! ## Components
//!
//! - [`Steps`] - Container for step indicators with direction control
//! - [`Step`] - Individual step with color and content options
//!
//! ## Features
//!
//! - **Direction Control**: Horizontal (default) or vertical layouts
//! - **Color Variants**: Semantic color options for different step states
//! - **Custom Content**: Support for custom step labels and indicators
//! - **Data Attributes**: Optional data-content for step numbering
//! - **Progress Indication**: Visual representation of completion status
//!
//! ## CSS Classes
//!
//! | Component | CSS Class | Description |
//! |-----------|-----------|-------------|
//! | `Steps` | `steps` | Base steps container |
//! | `Steps` (vertical) | `steps-vertical` | Vertical step layout |
//! | `Step` | `step` | Individual step base |
//! | `Step` (primary) | `step-primary` | Primary color variant |
//! | `Step` (secondary) | `step-secondary` | Secondary color variant |
//! | `Step` (accent) | `step-accent` | Accent color variant |
//! | `Step` (info) | `step-info` | Info color variant |
//! | `Step` (success) | `step-success` | Success color variant |
//! | `Step` (warning) | `step-warning` | Warning color variant |
//! | `Step` (error) | `step-error` | Error color variant |
//!
//! ## Usage
//!
//! ### Basic Horizontal Steps
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Steps>
//!             <Step color=StepColor::Primary data_content="1">"Register"</Step>
//!             <Step color=StepColor::Primary data_content="2">"Choose plan"</Step>
//!             <Step data_content="3">"Purchase"</Step>
//!             <Step data_content="4">"Receive Product"</Step>
//!         </Steps>
//!     }
//! }
//! ```
//!
//! ### Vertical Steps with Progress
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let direction = RwSignal::new(StepsDirection::Vertical);
//!
//!     view! {
//!         <Steps direction=direction>
//!             <Step color=StepColor::Success data_content="✓">"Account Setup"</Step>
//!             <Step color=StepColor::Success data_content="✓">"Profile Complete"</Step>
//!             <Step color=StepColor::Primary data_content="3">"Email Verification"</Step>
//!             <Step data_content="4">"Start Using App"</Step>
//!         </Steps>
//!     }
//! }
//! ```
//!
//! ### Process Steps with Status Colors
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Steps>
//!             <Step color=StepColor::Success data_content="✓">
//!                 "Order Placed"
//!             </Step>
//!             <Step color=StepColor::Success data_content="✓">
//!                 "Payment Confirmed"
//!             </Step>
//!             <Step color=StepColor::Info data_content="3">
//!                 "Processing"
//!             </Step>
//!             <Step color=StepColor::Warning data_content="4">
//!                 "Shipping"
//!             </Step>
//!             <Step data_content="5">
//!                 "Delivered"
//!             </Step>
//!         </Steps>
//!     }
//! }
//! ```
//!
//! ### Custom Step Content
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Steps>
//!             <Step color=StepColor::Primary>
//!                 <div class="flex flex-col items-center">
//!                     <svg class="w-6 h-6 mb-1">
//!                         <path d="..." fill="currentColor" />
//!                     </svg>
//!                     "Upload"
//!                 </div>
//!             </Step>
//!             <Step color=StepColor::Primary>
//!                 <div class="flex flex-col items-center">
//!                     <svg class="w-6 h-6 mb-1">
//!                         <path d="..." fill="currentColor" />
//!                     </svg>
//!                     "Process"
//!                 </div>
//!             </Step>
//!         </Steps>
//!     }
//! }
//! ```
//!
//! ## Step Numbering
//!
//! The `data_content` prop allows you to customize the step indicator:
//! - Numbers: `"1"`, `"2"`, `"3"`
//! - Checkmarks: `"✓"`, `"✔"`
//! - Custom symbols: `"★"`, `"●"`, `"►"`
//! - Letters: `"A"`, `"B"`, `"C"`
//!
//! ## Accessibility
//!
//! - Use proper semantic markup for step progression
//! - Include descriptive text for each step
//! - Consider ARIA attributes for screen reader support
//! - Ensure sufficient color contrast for all step states
//! - Provide alternative text for icon-based steps
//!
//! ## Best Practices
//!
//! - Keep step labels concise and actionable
//! - Use consistent visual indicators across similar processes
//! - Provide clear feedback for completed, current, and upcoming steps
//! - Consider mobile responsiveness for vertical layouts
//! - Test with keyboard navigation for accessibility
//! - Use appropriate colors to convey step status

mod component;
mod style;

pub use component::*;
pub use style::*;
