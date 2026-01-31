use leptos::prelude::*;
use web_sys::DragEvent;

use super::card::KanbanCardView;
use super::drag::*;
use super::types::*;

/// Kanban column component with drag-and-drop support
#[component]
pub fn KanbanColumnView(
    /// Column data
    #[prop(into)]
    column: Signal<KanbanColumn>,

    /// Drag state signal
    #[prop(into)]
    drag_state: Signal<DragState>,

    /// Callback when drag starts from this column
    #[prop(into)]
    on_drag_start: Option<Callback<String>>,

    /// Callback when something is dragged over this column
    #[prop(into)]
    on_drag_over: Option<Callback<Option<usize>>>,

    /// Callback when drag leaves this column
    #[prop(into)]
    on_drag_leave: Option<Callback<()>>,

    /// Callback when something is dropped in this column
    #[prop(into)]
    on_drop: Option<Callback<usize>>,

    /// Callback when a card is clicked
    #[prop(into)]
    on_card_click: Option<Callback<String>>,

    /// Callback when a card should be deleted
    #[prop(into)]
    on_card_delete: Option<Callback<String>>,

    /// Callback when a new card should be created in this column
    #[prop(into)]
    on_card_create: Option<Callback<String>>,

    /// Callback when column collapse state should toggle
    #[prop(into)]
    on_toggle_collapse: Option<Callback<String>>,
) -> impl IntoView {
    let column_id = Signal::derive(move || column.get().column_id.clone());
    let is_collapsed = Signal::derive(move || column.get().collapsed);
    let is_over_limit = Signal::derive(move || column.get().is_over_limit());

    view! {
        <div
            class="kanban-column flex-shrink-0 bg-base-200 rounded-lg"
            class:w-12=move || is_collapsed.get()
            class:w-80=move || !is_collapsed.get()
        >
            // Column header
            <div class="kanban-column-header p-4 border-b border-base-300 flex items-center justify-between">
                <Show
                    when=move || !is_collapsed.get()
                    fallback=move || {
                        view! {
                            <div class="writing-mode-vertical text-sm font-semibold">
                                {move || column.get().title}
                            </div>
                        }
                    }
                >
                    <div class="flex-1">
                        <h3 class="text-lg font-semibold flex items-center gap-2">
                            {move || column.get().title}
                            <span class="badge badge-sm">{move || column.get().card_count()}</span>
                            <Show when=move || is_over_limit.get()>
                                <span class="badge badge-error badge-sm">{"WIP Limit!"}</span>
                            </Show>
                        </h3>
                    </div>
                </Show>

                // Collapse toggle button
                <Show when=move || column.get().collapsible>
                    <button
                        class="btn btn-ghost btn-sm btn-circle"
                        aria-label="Toggle column"
                        on:click=move |_| {
                            if let Some(ref cb) = on_toggle_collapse {
                                cb.run(column_id.get());
                            }
                        }
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-5 h-5"
                        >
                            <Show
                                when=move || is_collapsed.get()
                                fallback=move || {
                                    view! {
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M18.75 19.5l-7.5-7.5 7.5-7.5m-6 15L5.25 12l7.5-7.5"
                                        />
                                    }
                                }
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M11.25 4.5l7.5 7.5-7.5 7.5m-6-15l7.5 7.5-7.5 7.5"
                                />
                            </Show>
                        </svg>
                    </button>
                </Show>
            </div>

            // Cards container (only show when not collapsed)
            <Show when=move || !is_collapsed.get()>
                <div
                    class="kanban-cards p-4 space-y-2"
                    class:overflow-y-auto=move || column.get().scrollable
                    class:bg-primary=move || {
                        drag_state.get().is_drop_target(&column_id.get())
                    }
                    class:bg-opacity-10=move || {
                        drag_state.get().is_drop_target(&column_id.get())
                    }
                    style="max-height: 500px; min-height: 200px;"
                    on:dragover=move |ev: DragEvent| {
                        ev.prevent_default();
                        if let Some(ref cb) = on_drag_over {
                            // For now, we'll drop at the end of the column
                            cb.run(Some(column.get().cards.len()));
                        }
                        if let Some(dt) = ev.data_transfer() {
                            dt.set_drop_effect("move");
                        }
                    }
                    on:dragleave=move |_ev: DragEvent| {
                        if let Some(ref cb) = on_drag_leave {
                            cb.run(());
                        }
                    }
                    on:drop=move |ev: DragEvent| {
                        ev.prevent_default();
                        if let Some(ref cb) = on_drop {
                            // Drop at the end of the column
                            cb.run(column.get().cards.len());
                        }
                    }
                >
                    <For
                        each=move || column.get().cards
                        key=|card| card.card_id.clone()
                        children=move |card| {
                            let card_id = card.card_id.clone();
                            let is_dragging = Signal::derive(move || {
                                drag_state.get().dragged_card.as_ref() == Some(&card_id)
                            });

                            view! {
                                <KanbanCardView
                                    card=Signal::derive(move || card.clone())
                                    is_dragging=is_dragging
                                    on_drag_start=on_drag_start
                                    on_click=on_card_click
                                    on_delete=on_card_delete
                                />
                            }
                        }
                    />

                    // Empty state
                    <Show when=move || column.get().cards.is_empty()>
                        <div class="text-center text-base-content/50 py-8">
                            "No cards"
                        </div>
                    </Show>
                </div>

                // Add card button
                <Show when=move || on_card_create.is_some()>
                    <div class="p-4 border-t border-base-300">
                        <button
                            class="btn btn-ghost btn-sm w-full"
                            on:click=move |_| {
                                if let Some(ref cb) = on_card_create {
                                    cb.run(column_id.get());
                                }
                            }
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
                                    d="M12 4.5v15m7.5-7.5h-15"
                                />
                            </svg>
                            "Add Card"
                        </button>
                    </div>
                </Show>
            </Show>
        </div>
    }
}
