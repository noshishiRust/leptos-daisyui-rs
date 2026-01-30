use leptos::prelude::*;
use crate::components::data_table::types::DataTableTexts;

/// DataTable pagination controls
#[component]
pub fn DataTableControls(
    /// Current page index (0-based)
    #[prop(into)]
    current_page: Signal<usize>,

    /// Callback to set current page
    set_current_page: WriteSignal<usize>,

    /// Total number of pages
    #[prop(into)]
    total_pages: Signal<usize>,

    /// Custom text strings
    texts: DataTableTexts,

    /// Pagination container class
    #[prop(optional, into)]
    pagination_class: &'static str,

    /// Pagination button class
    #[prop(optional, into)]
    button_class: &'static str,

    /// Page indicator class
    #[prop(optional, into)]
    indicator_class: &'static str,
) -> impl IntoView {
    let on_previous = move |_ev: leptos::ev::MouseEvent| {
        if current_page.get() > 0 {
            set_current_page.set(current_page.get() - 1);
        }
    };

    #[allow(unused_variables)]
    let on_next = move |_ev: leptos::ev::MouseEvent| {
        if current_page.get() + 1 < total_pages.get() {
            set_current_page.set(current_page.get() + 1);
        }
    };

    view! {
        <div class=pagination_class>
            <button
                class=button_class
                disabled=move || current_page.get() == 0
                on:click=on_previous
            >
                {texts.previous}
            </button>

            <span class=indicator_class>
                {move || {
                    let current = current_page.get() + 1; // Display as 1-based
                    let total = total_pages.get();
                    texts.page_indicator
                        .replace("{current}", &current.to_string())
                        .replace("{total}", &total.to_string())
                }}
            </span>

            <button
                class=button_class
                disabled=move || current_page.get() + 1 >= total_pages.get()
                on:click=on_next
            >
                {texts.next}
            </button>
        </div>
    }
}
