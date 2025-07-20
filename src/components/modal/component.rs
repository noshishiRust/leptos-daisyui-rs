//! Modal component implementations for overlay dialogs.

use crate::merge_classes;
use leptos::{
    html::{Dialog, Div, Form},
    prelude::*,
};

/// A modal dialog component using native HTML `<dialog>` element.
///
/// Creates an overlay dialog with proper accessibility, focus management, and state control.
/// Automatically manages the DOM dialog state based on the `open` signal.
///
/// # Props
///
/// - `open` - Signal controlling whether the modal is open
/// - `backdrop` - Whether to include a backdrop (click-to-close overlay)
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<dialog>` element
/// - `children` - Modal content (typically ModalBox and children)
///
/// # Behavior
///
/// - Automatically calls `showModal()` when `open` becomes `true`
/// - Automatically calls `close()` when `open` becomes `false`
/// - Native ESC key support for closing (when using HTML dialog)
/// - Proper focus trapping and restoration
/// - ARIA attributes for accessibility
///
/// # Examples
///
/// ## Basic Modal
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Modal, ModalBox};
///
/// #[component]
/// fn BasicModal() -> impl IntoView {
///     let open = RwSignal::new(false);
///
///     view! {
///         <button on:click=move |_| open.set(true)>
///             "Open Modal"
///         </button>
///
///         <Modal open=open.into()>
///             <ModalBox>
///                 <h2>"Hello Modal!"</h2>
///                 <p>"This is a simple modal dialog."</p>
///                 <button on:click=move |_| open.set(false)>
///                     "Close"
///                 </button>
///             </ModalBox>
///         </Modal>
///     }
/// }
/// ```
///
/// ## Modal with Backdrop
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Modal, ModalBox};
///
/// view! {
///     <Modal
///         open=open.into()
///         backdrop=Signal::derive(|| true)
///     >
///         <ModalBox>
///             "Content with click-outside-to-close backdrop"
///         </ModalBox>
///     </Modal>
/// }
/// ```
///
/// ## Confirmation Dialog
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Modal, ModalBox, ModalAction};
///
/// #[component]
/// fn ConfirmDialog() -> impl IntoView {
///     let open = RwSignal::new(false);
///     let confirmed = RwSignal::new(false);
///
///     view! {
///         <Modal open=open.into() backdrop=Signal::derive(|| true)>
///             <ModalBox>
///                 <h3 class="font-bold text-lg">"Delete Item"</h3>
///                 <p class="py-4">"This action cannot be undone."</p>
///                 <ModalAction>
///                     <button
///                         class="btn"
///                         on:click=move |_| open.set(false)
///                     >
///                         "Cancel"
///                     </button>
///                     <button
///                         class="btn btn-error"
///                         on:click=move |_| {
///                             confirmed.set(true);
///                             open.set(false);
///                         }
///                     >
///                         "Delete"
///                     </button>
///                 </ModalAction>
///             </ModalBox>
///         </Modal>
///     }
/// }
/// ```
///
/// ## Form Modal
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Modal, ModalBox, ModalAction};
///
/// view! {
///     <Modal open=open.into()>
///         <ModalBox>
///             <h3 class="font-bold text-lg">"Add New Item"</h3>
///             <div class="form-control w-full max-w-xs">
///                 <label class="label">
///                     <span class="label-text">"Item name"</span>
///                 </label>
///                 <input
///                     type="text"
///                     placeholder="Enter name"
///                     class="input input-bordered w-full max-w-xs"
///                 />
///             </div>
///             <ModalAction>
///                 <button class="btn">"Cancel"</button>
///                 <button class="btn btn-primary">"Save"</button>
///             </ModalAction>
///         </ModalBox>
///     </Modal>
/// }
/// ```
#[component]
pub fn Modal(
    /// Signal controlling modal open state
    #[prop(optional, into)]
    open: Signal<bool>,
    /// Whether to include backdrop for click-to-close
    #[prop(optional, into)]
    backdrop: Signal<bool>,
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the dialog element
    #[prop(optional)]
    node_ref: NodeRef<Dialog>,
    /// Modal content
    children: Children,
) -> impl IntoView {
    Effect::new(move || {
        let Some(node) = node_ref.get() else { return };

        if open.get() {
            let _ = node.show_modal();
        } else {
            node.close();
        }
    });

    view! {
        <dialog
            aria_modal=move || open.get()
            aria-label="Modal"
            node_ref=node_ref
            class=move || merge_classes!("modal", class)
            class:modal-open=open
        >
            {children()}
            {move || {
                if backdrop.get() { view! { <ModalBackdrop /> }.into_any() } else { ().into_any() }
            }}
        </dialog>
    }
}

/// Content container for modal dialogs.
///
/// Provides styled container for modal content with proper spacing, background,
/// and responsive design. Should be used inside a Modal component.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<div>` element
/// - `children` - Modal content (text, forms, other components)
///
/// # Styling
///
/// - Provides background color and rounded corners
/// - Responsive width with max-width constraints
/// - Proper padding and spacing
/// - Shadow and elevation effects
///
/// # Examples
///
/// ## Simple Content
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::ModalBox;
///
/// view! {
///     <ModalBox>
///         <h2 class="text-2xl font-bold">"Title"</h2>
///         <p class="py-4">"Modal content goes here."</p>
///     </ModalBox>
/// }
/// ```
///
/// ## Custom Styling
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::ModalBox;
///
/// view! {
///     <ModalBox class="max-w-2xl bg-base-200">
///         "Wide modal with custom background"
///     </ModalBox>
/// }
/// ```
///
/// ## Rich Content
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::ModalBox;
///
/// view! {
///     <ModalBox>
///         <div class="flex items-center gap-3 mb-4">
///             <svg class="w-6 h-6 text-warning">/* icon */</svg>
///             <h3 class="font-bold text-lg">"Warning"</h3>
///         </div>
///         <p class="mb-4">
///             "This action will permanently delete all selected items. "
///             "This cannot be undone."
///         </p>
///         <div class="bg-warning/10 p-3 rounded">
///             <p class="text-sm">
///                 "Tip: You can backup your data before proceeding."
///             </p>
///         </div>
///     </ModalBox>
/// }
/// ```
#[component]
pub fn ModalBox(
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Modal content
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("modal-box", class)>
            {children()}
        </div>
    }
}

/// Action button container for modal dialogs.
///
/// Provides a styled container for action buttons, typically placed at the bottom
/// of a modal. Handles proper spacing and alignment for button groups.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<div>` element
/// - `children` - Action buttons or other interactive elements
///
/// # Layout
///
/// - Right-aligned button layout by default
/// - Proper spacing between buttons
/// - Responsive design for mobile devices
///
/// # Examples
///
/// ## Standard Actions
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::ModalAction;
///
/// view! {
///     <ModalAction>
///         <button class="btn">"Cancel"</button>
///         <button class="btn btn-primary">"Save"</button>
///     </ModalAction>
/// }
/// ```
///
/// ## Multiple Actions
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::ModalAction;
///
/// view! {
///     <ModalAction>
///         <button class="btn btn-ghost">"Maybe Later"</button>
///         <button class="btn btn-outline">"Learn More"</button>
///         <button class="btn btn-primary">"Get Started"</button>
///     </ModalAction>
/// }
/// ```
///
/// ## Destructive Actions
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::ModalAction;
///
/// view! {
///     <ModalAction>
///         <button class="btn btn-ghost">"Cancel"</button>
///         <button class="btn btn-error">"Delete"</button>
///     </ModalAction>
/// }
/// ```
///
/// ## Custom Layout
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::ModalAction;
///
/// view! {
///     <ModalAction class="justify-between">
///         <button class="btn btn-sm btn-ghost">"Help"</button>
///         <div class="space-x-2">
///             <button class="btn">"Cancel"</button>
///             <button class="btn btn-primary">"Apply"</button>
///         </div>
///     </ModalAction>
/// }
/// ```
#[component]
pub fn ModalAction(
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Action buttons
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("modal-action", class)>
            {children()}
        </div>
    }
}

/// Backdrop component for modal click-to-close functionality.
///
/// Creates an invisible overlay that closes the modal when clicked.
/// Uses a form with `method="dialog"` to leverage native dialog behavior.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<form>` element
///
/// # Behavior
///
/// - Automatically included when Modal has `backdrop=true`
/// - Clicking the backdrop closes the modal
/// - Uses native HTML dialog close mechanism
/// - Invisible but covers the entire modal area
///
/// # Technical Details
///
/// This component uses a `<form method="dialog">` with a button to close the modal.
/// This leverages the native HTML dialog API for proper closing behavior.
///
/// # Examples
///
/// ## Automatic Usage
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::Modal;
///
/// // ModalBackdrop is automatically included when backdrop=true
/// view! {
///     <Modal backdrop=Signal::derive(|| true)>
///         // Modal content
///     </Modal>
/// }
/// ```
///
/// ## Manual Usage (Advanced)
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Modal, ModalBackdrop};
///
/// view! {
///     <Modal backdrop=Signal::derive(|| false)>
///         // Modal content
///         <ModalBackdrop class="custom-backdrop"/>
///     </Modal>
/// }
/// ```
///
/// ## Custom Backdrop Styling
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::ModalBackdrop;
///
/// view! {
///     <ModalBackdrop class="bg-black/50 backdrop-blur-sm"/>
/// }
/// ```
#[component]
pub fn ModalBackdrop(
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the form element
    #[prop(optional)]
    node_ref: NodeRef<Form>,
) -> impl IntoView {
    view! {
        <form
            node_ref=node_ref
            method="dialog"
            class=move || merge_classes!("modal-backdrop", class)
        >
            <button>close</button>
        </form>
    }
}
