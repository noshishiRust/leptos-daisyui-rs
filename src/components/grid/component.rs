use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Grid Component
///
/// A CSS Grid layout wrapper with configurable columns, rows, and gaps.
/// Provides a convenient way to create grid-based layouts.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("grid grid-cols-1 grid-cols-2 grid-cols-3 grid-cols-4 grid-cols-6 grid-cols-12 gap-1 gap-2 gap-4 gap-8");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Grid(
    /// Number of columns (1-12)
    #[prop(optional, into)]
    cols: Signal<u8>,

    /// Gap between grid items
    #[prop(optional, into)]
    gap: Signal<u8>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child elements to layout in grid
    children: Children,
) -> impl IntoView {
    let cols_class = Signal::derive(move || {
        let c = cols.get();
        match c {
            1 => "grid-cols-1",
            2 => "grid-cols-2",
            3 => "grid-cols-3",
            4 => "grid-cols-4",
            6 => "grid-cols-6",
            12 => "grid-cols-12",
            _ => "grid-cols-1", // default
        }
    });

    let gap_class = Signal::derive(move || {
        let g = gap.get();
        match g {
            0 => "",
            1 => "gap-1",
            2 => "gap-2",
            4 => "gap-4",
            8 => "gap-8",
            _ => "gap-4", // default
        }
    });

    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("grid", cols_class.get(), gap_class.get(), class)
        >
            {children()}
        </div>
    }
}
