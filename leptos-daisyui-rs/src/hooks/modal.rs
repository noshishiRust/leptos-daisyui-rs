//! # use_modal Hook
//!
//! Modal open/close state management with dialog element integration.

use leptos::{html::Dialog, prelude::*};

/// Modal state with dialog element integration.
///
/// Encapsulates open/close state and `NodeRef<Dialog>` for direct
/// dialog element control via `show_modal()` / `close()`.
///
/// ## Example
///
/// ```rust,ignore
/// let modal = use_modal(false);
///
/// view! {
///     <Button on:click=move |_| modal.open()>"Open"</Button>
///     <Modal open=modal.open node_ref=modal.node_ref backdrop=Signal::from(true)>
///         <ModalBox>
///             <p>"Content"</p>
///         </ModalBox>
///         <ModalAction>
///             <Button on:click=move |_| modal.close()>"Close"</Button>
///         </ModalAction>
///     </Modal>
/// }
/// ```
#[derive(Clone)]
pub struct ModalState {
    /// Whether the modal is currently open.
    pub open: Signal<bool>,
    /// Writer for the open state.
    pub set_open: WriteSignal<bool>,
    /// Reference to the `<dialog>` element.
    pub node_ref: NodeRef<Dialog>,
}

impl ModalState {
    /// Open the modal.
    ///
    /// Sets `open` to `true` and calls `show_modal()` on the dialog element.
    pub fn open(&self) {
        self.set_open.set(true);
        if let Some(node) = self.node_ref.get() {
            let _ = node.show_modal();
        }
    }

    /// Close the modal.
    ///
    /// Sets `open` to `false` and calls `close()` on the dialog element.
    pub fn close(&self) {
        self.set_open.set(false);
        if let Some(node) = self.node_ref.get() {
            node.close();
        }
    }

    /// Toggle the modal open/closed.
    pub fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }

    /// Whether the modal is currently open.
    pub fn is_open(&self) -> bool {
        self.open.get()
    }
}

/// Hook for managing modal open/close state with `<dialog>` element integration.
///
/// Returns a [`ModalState`] containing the reactive `open` signal, its writer,
/// and a `NodeRef<Dialog>` for binding to the Modal component.
///
/// ## Example
///
/// ```rust,ignore
/// let modal = use_modal(false); // initially closed
///
/// view! {
///     <Button on:click=move |_| modal.open()>"Open Modal"</Button>
///     <Modal open=modal.open node_ref=modal.node_ref>
///         <ModalBox>"Hello!"</ModalBox>
///     </Modal>
/// }
/// ```
pub fn use_modal(initial: bool) -> ModalState {
    let (open, set_open) = signal(initial);
    let node_ref = NodeRef::<Dialog>::new();

    ModalState {
        open: open.into(),
        set_open,
        node_ref,
    }
}
