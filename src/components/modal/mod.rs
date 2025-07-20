//! # Modal Component Module
//!
//! Modal dialog components for displaying overlay content and user interactions.
//! Built on the HTML `<dialog>` element with proper accessibility and backdrop handling.
//!
//! ## Components
//!
//! - [`Modal`] - Root modal container with open/close state management
//! - [`ModalBox`] - Content container within the modal
//! - [`ModalAction`] - Action button container (typically at bottom)
//! - [`ModalBackdrop`] - Backdrop for closing modal when clicked
//!
//! ## Features
//!
//! - **Native Dialog**: Built on HTML `<dialog>` element for proper semantics
//! - **State Management**: Reactive open/close state with automatic DOM updates
//! - **Backdrop Control**: Optional backdrop with click-to-close functionality
//! - **Accessibility**: Proper ARIA attributes and focus management
//! - **Keyboard Support**: ESC key to close modal (native dialog behavior)
//!
//! ## Modal Structure
//!
//! ```text
//! ┌────────────────────────────────────────────────┐
//! │                 [Modal]                   │
//! │  ┌──────────────────────────────────────┐  │
//! │  │            [ModalBox]            │  │
//! │  │                                  │  │
//! │  │  Modal content goes here      │  │
//! │  │                                  │  │
//! │  │  ┌──────────────────────────┐  │  │
//! │  │  │      [ModalAction]      │  │  │
//! │  │  │   [Cancel] [Confirm]   │  │  │
//! │  │  └──────────────────────────┘  │  │
//! │  └──────────────────────────────────────┘  │
//! │              [ModalBackdrop]              │
//! └────────────────────────────────────────────────┘
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{Modal, ModalBox, ModalAction};
//!
//! #[component]
//! fn ConfirmDialog() -> impl IntoView {
//!     let open = RwSignal::new(false);
//!
//!     view! {
//!         <button
//!             class="btn btn-primary"
//!             on:click=move |_| open.set(true)
//!         >
//!             "Open Modal"
//!         </button>
//!
//!         <Modal open=open.into() backdrop=Signal::derive(|| true)>
//!             <ModalBox>
//!                 <h3 class="font-bold text-lg">"Confirm Action"</h3>
//!                 <p class="py-4">"Are you sure you want to proceed?"</p>
//!                 <ModalAction>
//!                     <button
//!                         class="btn"
//!                         on:click=move |_| open.set(false)
//!                     >
//!                         "Cancel"
//!                     </button>
//!                     <button
//!                         class="btn btn-primary"
//!                         on:click=move |_| open.set(false)
//!                     >
//!                         "Confirm"
//!                     </button>
//!                 </ModalAction>
//!             </ModalBox>
//!         </Modal>
//!     }
//! }
//! ```
//!
//! ## CSS Classes
//!
//! - `.modal` - Base modal overlay container
//! - `.modal-open` - Applied when modal is open
//! - `.modal-box` - Content container with styling
//! - `.modal-action` - Action button container
//! - `.modal-backdrop` - Backdrop element for click-to-close

mod component;

pub use component::*;
