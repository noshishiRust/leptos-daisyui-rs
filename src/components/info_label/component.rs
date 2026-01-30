use crate::merge_classes;
use leptos::{html::Span, prelude::*};

/// # InfoLabel Component
///
/// Information label with icon for help text, tooltips, or info badges.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("badge badge-info gap-2");
/// ```
#[component]
pub fn InfoLabel(
    #[prop(optional, into)] icon: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("badge badge-info gap-2", class)>
            {move || {
                if icon.get() {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-4 w-4"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                        </svg>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}
            {children()}
        </span>
    }
}
