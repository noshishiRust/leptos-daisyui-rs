use leptos::prelude::*;

use super::types::*;
use super::column::KanbanColumnView;
use super::drag::*;

/// Main Kanban board component with drag-and-drop support
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

    /// Callback when a card is moved between columns
    #[prop(optional)]
    on_card_move: Option<Callback<DragOperation>>,

    /// Validation callback for drag operations
    #[prop(optional)]
    on_drag_validate: Option<Callback<DragOperation, DragValidation>>,

    /// NodeRef for accessing the DOM element
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// Additional CSS classes
    #[prop(optional, into, default="")]
    class: &'static str,
) -> impl IntoView {
    // Drag state management
    let (drag_state, set_drag_state) = signal(DragState::default());
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
                        let column_id = column.column_id.clone();
                        let col_id_for_start = column_id.clone();
                        let col_id_for_over = column_id.clone();
                        let col_id_for_drop = column_id.clone();

                        view! {
                            <KanbanColumnView
                                column=Signal::derive(move || column.clone())
                                drag_state=drag_state
                                on_drag_start=Some(Callback::new(move |card_id: String| {
                                    set_drag_state.update(|state| {
                                        state.start_drag(card_id, col_id_for_start.clone());
                                    });
                                }))
                                on_drag_over=Some(Callback::new(move |position: Option<usize>| {
                                    set_drag_state.update(|state| {
                                        state.update_target(col_id_for_over.clone(), position);
                                    });
                                }))
                                on_drag_leave=Some(Callback::new(move |_: ()| {
                                    set_drag_state.update(|state| {
                                        state.clear_target();
                                    });
                                }))
                                on_drop=Some(Callback::new(move |position: usize| {
                                    let current_state = drag_state.get();
                                    if let (Some(card_id), Some(source_col)) =
                                        (current_state.dragged_card.clone(), current_state.source_column.clone()) {
                                        let operation = DragOperation::new(
                                            card_id,
                                            source_col,
                                            col_id_for_drop.clone(),
                                            position,
                                        );

                                        // Validate if validator is provided
                                        let is_valid = if let Some(ref validator) = on_drag_validate {
                                            matches!(validator.run(operation.clone()), DragValidation::Allow | DragValidation::Warn(_))
                                        } else {
                                            true
                                        };

                                        if is_valid {
                                            if let Some(ref callback) = on_card_move {
                                                callback.run(operation);
                                            }
                                        }
                                    }

                                    set_drag_state.update(|state| {
                                        state.end_drag();
                                    });
                                }))
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
