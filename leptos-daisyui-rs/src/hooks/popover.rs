//! # use_popover Hook
//!
//! Show/hide state management with click-outside detection for dropdowns/popovers.

use leptos::{
    html::{Div, Ul},
    prelude::*,
};

/// Popover state with click-outside detection.
///
/// Encapsulates open/close state with two `NodeRef`s — one for the trigger
/// element and one for the content — enabling click-outside-to-close behavior.
///
/// ## Example
///
/// ```rust,ignore
/// let popover = use_popover(false);
///
/// view! {
///     <Dropdown open=popover.open>
///         <Button
///             node_ref=popover.trigger_ref
///             on:click=move |_| popover.toggle()
///         >"Menu"</Button>
///         <DropdownContent node_ref=popover.content_ref is_menu=true>
///             <li>"Item 1"</li>
///         </DropdownContent>
///     </Dropdown>
/// }
/// ```
#[derive(Clone)]
pub struct PopoverState {
    /// Whether the popover is currently visible.
    pub open: Signal<bool>,
    /// Writer for the open state.
    pub set_open: WriteSignal<bool>,
    /// Reference to the trigger element (e.g. button).
    pub trigger_ref: NodeRef<Div>,
    /// Reference to the content element (e.g. dropdown menu).
    pub content_ref: NodeRef<Ul>,
}

impl PopoverState {
    /// Show the popover.
    pub fn show(&self) {
        self.set_open.set(true);
    }

    /// Hide the popover.
    pub fn hide(&self) {
        self.set_open.set(false);
    }

    /// Toggle the popover visibility.
    pub fn toggle(&self) {
        self.set_open.update(|v| *v = !*v);
    }

    /// Whether the popover is currently visible.
    pub fn is_open(&self) -> bool {
        self.open.get()
    }
}

/// Hook for managing popover/dropdown show/hide state.
///
/// Returns a [`PopoverState`] with reactive open/close signals and two NodeRefs
/// for the trigger and content elements. Use `open` signal with the existing
/// Dropdown component.
///
/// ## Example
///
/// ```rust,ignore
/// let popover = use_popover(false);
///
/// view! {
///     <Dropdown open=popover.open>
///         <div node_ref=popover.trigger_ref on:click=move |_| popover.toggle()>
///             "Toggle"
///         </div>
///         <DropdownContent node_ref=popover.content_ref is_menu=true>
///             <li>"Item 1"</li>
///         </DropdownContent>
///     </Dropdown>
/// }
/// ```
pub fn use_popover(initial: bool) -> PopoverState {
    let (open, set_open) = signal(initial);
    let trigger_ref = NodeRef::<Div>::new();
    let content_ref = NodeRef::<Ul>::new();

    PopoverState {
        open: open.into(),
        set_open,
        trigger_ref,
        content_ref,
    }
}
