use leptos::prelude::*;

use super::types::*;

/// Filter bar component for kanban board
#[component]
pub fn KanbanFilterBar(
    /// Current filter state
    #[prop(into)]
    filters: Signal<KanbanFilters>,

    /// Setter for filter state
    set_filters: WriteSignal<KanbanFilters>,

    /// Enable search functionality
    #[prop(optional, default = true)]
    enable_search: bool,

    /// Search debounce in milliseconds
    #[prop(optional, default = 300)]
    _search_debounce_ms: u64,
) -> impl IntoView {
    // Search input value
    let (search_input, set_search_input) = signal(String::new());

    // Update filters when search changes (with debounce in real implementation)
    Effect::new(move || {
        let query = search_input.get();
        set_filters.update(|f| {
            f.search_query = if query.is_empty() {
                None
            } else {
                Some(query)
            };
        });
    });

    view! {
        <div class="kanban-filters flex flex-wrap gap-4 items-center mb-4">
            // Search input
            <Show when=move || enable_search>
                <div class="form-control flex-1 min-w-64">
                    <div class="input-group">
                        <input
                            type="text"
                            placeholder="Search cards..."
                            class="input input-bordered w-full"
                            prop:value=move || search_input.get()
                            on:input=move |ev| {
                                set_search_input.set(event_target_value(&ev));
                            }
                        />
                        <Show when=move || !search_input.get().is_empty()>
                            <button
                                class="btn btn-square"
                                on:click=move |_| set_search_input.set(String::new())
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="w-5 h-5"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M6 18L18 6M6 6l12 12"
                                    />
                                </svg>
                            </button>
                        </Show>
                    </div>
                </div>
            </Show>

            // Filter logic toggle
            <div class="join">
                <button
                    class="btn btn-sm join-item"
                    class:btn-active=move || filters.get().filter_logic == FilterLogic::And
                    on:click=move |_| {
                        set_filters.update(|f| f.filter_logic = FilterLogic::And);
                    }
                >
                    "AND"
                </button>
                <button
                    class="btn btn-sm join-item"
                    class:btn-active=move || filters.get().filter_logic == FilterLogic::Or
                    on:click=move |_| {
                        set_filters.update(|f| f.filter_logic = FilterLogic::Or);
                    }
                >
                    "OR"
                </button>
            </div>

            // Clear all filters button
            <button
                class="btn btn-sm btn-ghost"
                on:click=move |_| {
                    set_search_input.set(String::new());
                    set_filters.set(KanbanFilters::new());
                }
            >
                "Clear Filters"
            </button>
        </div>
    }
}
