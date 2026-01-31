use leptos::prelude::*;

use super::types::*;
use super::column::KanbanColumnView;

/// Main Kanban board component (Phase 1: Basic structure)
#[component]
pub fn KanbanBoard(
    /// Unique identifier for the board
    #[prop(into)]
    _board_id: String,

    /// Board title (optional)
    #[prop(optional, into)]
    title: Option<String>,

    /// Columns to display
    #[prop(into)]
    columns: Signal<Vec<KanbanColumn>>,

    /// Callback when a card is clicked
    #[prop(optional)]
    on_card_click: Option<Callback<String>>,

    /// Callback when a card should be deleted
    #[prop(optional)]
    on_card_delete: Option<Callback<String>>,

    /// NodeRef for accessing the DOM element
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// Additional CSS classes
    #[prop(optional, into, default="")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=format!("kanban-board w-full {}", class)
        >
            // Board header
            {move || title.clone().map(|t| {
                view! {
                    <div class="kanban-board-header mb-4">
                        <h2 class="text-2xl font-bold">{t}</h2>
                    </div>
                }
            })}

            // Columns container
            <div class="kanban-columns flex gap-4 overflow-x-auto pb-4">
                <For
                    each=move || columns.get()
                    key=|col| col.column_id.clone()
                    children=move |column| {
                        view! {
                            <KanbanColumnView
                                column=Signal::derive(move || column.clone())
                                on_card_click=on_card_click.clone()
                                on_card_delete=on_card_delete.clone()
                                on_card_create=None
                            />
                        }
                    }
                />
            </div>
        </div>
    }
}
