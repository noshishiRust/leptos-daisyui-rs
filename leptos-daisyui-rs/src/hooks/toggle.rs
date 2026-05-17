//! # use_toggle Hook
//!
//! Boolean toggle state management with checkbox element integration.

use leptos::{html::Input, prelude::*};

/// Toggle state with checkbox element integration.
///
/// Encapsulates checked state and `NodeRef<Input>` for direct
/// input element control.
///
/// ## Example
///
/// ```rust,ignore
/// let dark_mode = use_toggle(false);
///
/// view! {
///     <Toggle
///         node_ref=dark_mode.node_ref
///         color=Signal::from(ToggleColor::Primary)
///         on:change=move |_| dark_mode.sync_from_dom()
///     />
///     <span>{move || if dark_mode.is_checked() { "Dark" } else { "Light" }}</span>
/// }
/// ```
#[derive(Clone)]
pub struct ToggleState {
    /// Whether the toggle is currently checked.
    pub checked: Signal<bool>,
    /// Writer for the checked state.
    pub set_checked: WriteSignal<bool>,
    /// Reference to the `<input type="checkbox">` element.
    pub node_ref: NodeRef<Input>,
}

impl ToggleState {
    /// Set to checked (ON).
    pub fn on(&self) {
        self.set_checked.set(true);
        self.sync_to_dom();
    }

    /// Set to unchecked (OFF).
    pub fn off(&self) {
        self.set_checked.set(false);
        self.sync_to_dom();
    }

    /// Toggle the state.
    pub fn toggle(&self) {
        self.set_checked.update(|v| *v = !*v);
        self.sync_to_dom();
    }

    /// Whether the toggle is currently checked.
    pub fn is_checked(&self) -> bool {
        self.checked.get()
    }

    /// Read the `checked` property from the DOM input element and update the signal.
    ///
    /// Call this from an `on:change` handler to keep the signal in sync with user input.
    pub fn sync_from_dom(&self) {
        if let Some(node) = self.node_ref.get() {
            let input: &web_sys::HtmlInputElement = &node;
            self.set_checked.set(input.checked());
        }
    }

    /// Write the current signal value to the DOM input element's `checked` property.
    fn sync_to_dom(&self) {
        if let Some(node) = self.node_ref.get() {
            let input: &web_sys::HtmlInputElement = &node;
            input.set_checked(self.checked.get());
        }
    }
}

/// Hook for managing toggle/checkbox boolean state with `<input>` element integration.
///
/// Returns a [`ToggleState`] containing the reactive `checked` signal, its writer,
/// and a `NodeRef<Input>` for binding to the Toggle component.
///
/// ## Example
///
/// ```rust,ignore
/// let dark_mode = use_toggle(false);
///
/// view! {
///     <Toggle
///         node_ref=dark_mode.node_ref
///         on:change=move |_| dark_mode.sync_from_dom()
///     />
/// }
/// ```
pub fn use_toggle(initial: bool) -> ToggleState {
    let (checked, set_checked) = signal(initial);
    let node_ref = NodeRef::<Input>::new();

    ToggleState {
        checked: checked.into(),
        set_checked,
        node_ref,
    }
}
