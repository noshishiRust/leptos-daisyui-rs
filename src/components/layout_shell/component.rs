use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # LayoutShell Component
///
/// An application shell layout with header, sidebar, content, and footer areas.
/// Provides a common page structure for web applications.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("min-h-screen flex flex-col w-full");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn LayoutShell(
    /// Optional header content
    #[prop(optional)]
    header: Option<ViewFn>,

    /// Optional sidebar content
    #[prop(optional)]
    sidebar: Option<ViewFn>,

    /// Optional footer content
    #[prop(optional)]
    footer: Option<ViewFn>,

    /// Whether sidebar is fixed
    #[prop(optional, into)]
    fixed_sidebar: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Main content area
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("min-h-screen flex flex-col", class)
        >
            {move || {
                header
                    .as_ref()
                    .map(|h| {
                        view! { <header class="w-full">{h.run()}</header> }.into_any()
                    })
            }}

            <div class="flex flex-1">
                {move || {
                    sidebar
                        .as_ref()
                        .map(|s| {
                            view! {
                                <aside
                                    class=move || {
                                        if fixed_sidebar.get() {
                                            "w-64 flex-shrink-0"
                                        } else {
                                            "w-64 flex-shrink-0"
                                        }
                                    }
                                >

                                    {s.run()}
                                </aside>
                            }
                                .into_any()
                        })
                }}

                <main class="flex-1">{children()}</main>
            </div>

            {move || {
                footer
                    .as_ref()
                    .map(|f| {
                        view! { <footer class="w-full">{f.run()}</footer> }.into_any()
                    })
            }}

        </div>
    }
}
