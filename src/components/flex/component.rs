use super::style::{FlexAlign, FlexDirection, FlexJustify, FlexWrap};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Flex Component
///
/// A flexbox layout wrapper with direction, gap, justify, and align properties.
/// Provides a convenient way to create flexible layouts with daisyUI styling.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("flex flex-row flex-col flex-wrap flex-nowrap gap-1 gap-2 gap-4 gap-8 justify-start justify-center justify-end justify-between items-start items-center items-end");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Flex(
    /// Flex direction (row, column)
    #[prop(optional, into)]
    direction: Signal<FlexDirection>,

    /// Whether to wrap flex items
    #[prop(optional, into)]
    wrap: Signal<FlexWrap>,

    /// Gap between flex items
    #[prop(optional, into)]
    gap: Signal<u8>,

    /// Justify content alignment
    #[prop(optional, into)]
    justify: Signal<FlexJustify>,

    /// Align items alignment
    #[prop(optional, into)]
    align: Signal<FlexAlign>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child elements to layout with flex
    children: Children,
) -> impl IntoView {
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
            class=move || {
                merge_classes!(
                    "flex",
                    direction.get().as_str(),
                    wrap.get().as_str(),
                    gap_class.get(),
                    justify.get().as_str(),
                    align.get().as_str(),
                    class
                )
            }
        >

            {children()}
        </div>
    }
}
