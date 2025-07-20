use leptos::prelude::*;

/// Content layout component for the demos
#[component]
pub fn ContentLayout(
    title: &'static str,
    description: &'static str,
    /// The content to display within the layout
    children: Children,
) -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">{title}</h1>
            <p class="text-base-content/70">{description}</p>

            <div class="space-y-4">
                <div class="space-y-2">{{ children() }}</div>
            </div>
        </div>
    }
}
