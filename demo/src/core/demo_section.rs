use crate::core::{CodeBlock, Section};
use leptos::prelude::*;
use leptos::web_sys;

/// Demo section component that displays a component preview with its code
///
/// This component creates a section with:
/// - An anchor link for hash navigation (#section-id)
/// - A rendered preview area
/// - A code snippet area (collapsible)
#[component]
pub fn DemoSection(
    /// Unique identifier for hash navigation (e.g., "colors", "sizes")
    id: &'static str,
    /// Section title displayed in the UI
    title: &'static str,
    /// The code snippet to display
    code: &'static str,
    /// Whether to show items in a row (flex-row) vs column
    #[prop(default = false)]
    row: bool,
    /// Whether to show items in a column (flex-col) vs row
    #[prop(default = false)]
    col: bool,
    /// The rendered component preview
    children: Children,
) -> impl IntoView {
    let (code_visible, set_code_visible) = signal(false);

    view! {
        <div id=id class="demo-section scroll-mt-24">
            <div class="flex items-center gap-2 mb-3">
                <a
                    href=format!("#{id}")
                    class="text-xl font-semibold hover:text-primary transition-colors flex items-center gap-2"
                    on:click=|e| {
                        e.prevent_default();
                        let id = e.current_target().unwrap().unchecked_ref::<web_sys::HtmlElement>();
                        let href = id.get_attribute("href").unwrap();
                        let hash = href.strip_prefix('#').unwrap();
                        if let Some(el) = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.get_element_by_id(hash))
                        {
                            el.scroll_into_view_with_bool(true);
                        }
                    }
                >
                    {title}
                    <span class="text-base-content/40 text-lg">#</span>
                </a>
                <button
                    on:click=move |_| set_code_visible.update(|v| *v = !v)
                    class="btn btn-ghost btn-xs ml-auto gap-1"
                    aria-label="Toggle code"
                >
                    <span class="text-sm">{move || if code_visible.get() { "Hide Code" } else { "Show Code" }}</span>
                    <Icon
                        icon=if code_visible.get() {
                            icondata::AiEyeInvisibleOutlined
                        } else {
                            icondata::AiCodeOutlined
                        }
                        class:rotate-180=move || code_visible.get()
                    />
                </button>
            </div>

            // Preview container
            <div class="demo-preview bg-base-200 border border-base-300 rounded-lg p-4 mb-2">
                <div class="flex gap-2" class:flex-col=col class:flex-row=row>
                    {children()}
                </div>
            </div>

            // Code block (collapsible)
            <div class=move || if code_visible.get() { "" } else { "hidden" }>
                <CodeBlock code=code title=format!("{title} Code").to_string() />
            </div>
        </div>
    }
}

/// Demo section variant that supports multiple examples with their own code snippets
#[component]
pub fn MultiDemoSection(
    /// Unique identifier for hash navigation
    id: &'static str,
    /// Section title
    title: &'static str,
    /// List of demo examples with their titles and code
    examples: Vec<DemoExample>,
    /// Whether to show items in a row
    #[prop(default = false)]
    row: bool,
    /// Whether to show items in a column
    #[prop(default = false)]
    col: bool,
) -> impl IntoView {
    view! {
        <div id=id class="demo-section scroll-mt-24">
            <div class="flex items-center gap-2 mb-3">
                <a
                    href=format!("#{id}")
                    class="text-xl font-semibold hover:text-primary transition-colors flex items-center gap-2"
                    on:click=|e| {
                        e.prevent_default();
                        let id = e.current_target().unwrap().unchecked_ref::<web_sys::HtmlElement>();
                        let href = id.get_attribute("href").unwrap();
                        let hash = href.strip_prefix('#').unwrap();
                        if let Some(el) = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.get_element_by_id(hash))
                        {
                            el.scroll_into_view_with_bool(true);
                        }
                    }
                >
                    {title}
                    <span class="text-base-content/40 text-lg">#</span>
                </a>
            </div>

            <div class="space-y-4">
                {examples
                    .into_iter()
                    .enumerate()
                    .map(|(idx, example)| {
                        let example_id = format!("{}-{}", id, idx);
                        let (code_visible, set_code_visible) = signal(false);

                        view! {
                            <div id=example_id class="demo-subsection">
                                <div class="flex items-center gap-2 mb-2">
                                    <h3 class="text-lg font-medium">{example.title}</h3>
                                    <button
                                        on:click=move |_| set_code_visible.update(|v| *v = !v)
                                        class="btn btn-ghost btn-xs ml-auto gap-1"
                                        aria-label="Toggle code"
                                    >
                                        <span class="text-sm">
                                            {move || if code_visible.get() { "Hide" } else { "Show" }}
                                        </span>
                                        <Icon
                                            icon=if code_visible.get() {
                                                icondata::AiEyeInvisibleOutlined
                                            } else {
                                                icondata::AiCodeOutlined
                                            }
                                            class=move || if code_visible.get() { "rotate-180" }
                                        />
                                    </button>
                                </div>

                                <div class="demo-preview bg-base-200 border border-base-300 rounded-lg p-4 mb-2">
                                    <div class="flex gap-2" class:flex-col=col class:flex-row=row>
                                        {example.content}
                                    </div>
                                </div>

                                <div class=move || if code_visible.get() { "" } else { "hidden" }>
                                    <CodeBlock code=example.code title=format!("{} Code", example.title).to_string() />
                                </div>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Example data for MultiDemoSection
#[derive(Clone, Debug)]
pub struct DemoExample {
    pub title: String,
    pub code: &'static str,
    pub content: AnyView,
}
