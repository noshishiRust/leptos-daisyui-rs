use leptos::prelude::*;
use leptos::logging;
use leptos_daisyui_rs::components::kanban::*;

#[component]
pub fn KanbanDemo() -> impl IntoView {
    // Create sample data
    let (columns, set_columns) = signal(vec![
        KanbanColumn::new("todo", "To Do")
            .with_cards(vec![
                KanbanCard::new("card-1", "Implement login")
                    .with_description("Add user authentication with JWT tokens")
                    .with_priority(Priority::High)
                    .with_label(Label::new("feature", "Feature").with_color("#3b82f6"))
                    .with_assignee(Assignee::new("user-1", "Alice Smith")),
                KanbanCard::new("card-2", "Fix navigation bug")
                    .with_description("Fix the broken link on the homepage")
                    .with_priority(Priority::Critical)
                    .with_label(Label::new("bug", "Bug").with_color("#ef4444")),
            ]),
        KanbanColumn::new("in-progress", "In Progress")
            .with_cards(vec![
                KanbanCard::new("card-3", "Design homepage")
                    .with_description("Create mockups for the new landing page")
                    .with_priority(Priority::Medium)
                    .with_label(Label::new("design", "Design").with_color("#8b5cf6"))
                    .with_assignee(Assignee::new("user-2", "Bob Johnson")),
            ]),
        KanbanColumn::new("review", "Review")
            .with_cards(vec![
                KanbanCard::new("card-4", "Update documentation")
                    .with_description("Add API reference docs")
                    .with_priority(Priority::Low)
                    .with_label(Label::new("docs", "Documentation").with_color("#10b981")),
            ]),
        KanbanColumn::new("done", "Done")
            .with_cards(vec![
                KanbanCard::new("card-5", "Setup CI/CD")
                    .with_description("Configure GitHub Actions")
                    .with_priority(Priority::Medium)
                    .with_label(Label::new("devops", "DevOps").with_color("#f59e0b")),
            ]),
    ]);

    // Handle card movement
    let handle_card_move = Callback::new(move |operation: DragOperation| {
        logging::log!("Moving card {} from {} to {} at position {}",
            operation.card_id,
            operation.from_column,
            operation.to_column,
            operation.to_position
        );

        // Update columns state
        set_columns.update(|cols| {
            // Find source and target columns
            let mut card_to_move = None;

            // Remove card from source column
            for col in cols.iter_mut() {
                if col.column_id == operation.from_column {
                    if let Some(index) = col.cards.iter().position(|c| c.card_id == operation.card_id) {
                        card_to_move = Some(col.cards.remove(index));
                    }
                    break;
                }
            }

            // Add card to target column
            if let Some(card) = card_to_move {
                for col in cols.iter_mut() {
                    if col.column_id == operation.to_column {
                        let insert_pos = operation.to_position.min(col.cards.len());
                        col.cards.insert(insert_pos, card);
                        break;
                    }
                }
            }
        });
    });

    // Handle card click
    let handle_card_click = Callback::new(move |card_id: String| {
        logging::log!("Card clicked: {}", card_id);
    });

    // Handle card delete
    let handle_card_delete = Callback::new(move |card_id: String| {
        logging::log!("Deleting card: {}", card_id);
        set_columns.update(|cols| {
            for col in cols.iter_mut() {
                col.cards.retain(|c| c.card_id != card_id);
            }
        });
    });

    view! {
        <div class="demo-section">
            <h2 class="text-2xl font-bold mb-4">"Kanban Board"</h2>
            <p class="mb-4 text-base-content/70">
                "A production-ready Kanban board with drag-and-drop support, WIP limits, and collapsible columns."
            </p>

            <div class="mockup-window border bg-base-300 mb-6">
                <div class="bg-base-200 px-4 py-8">
                    <KanbanBoard
                        _board_id="demo-board"
                        title="My Project"
                        columns=columns
                        on_card_move=handle_card_move
                        on_card_click=handle_card_click
                        on_card_delete=handle_card_delete
                    />
                </div>
            </div>

            <div class="alert alert-info">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <div>
                    <h3 class="font-bold">"Drag and Drop Tips:"</h3>
                    <ul class="list-disc list-inside text-sm mt-2">
                        <li>"Drag cards between columns to change their status"</li>
                        <li>"The \"In Progress\" column has a WIP limit of 3 cards"</li>
                        <li>"Overdue cards show a red border (e.g., \"Fix navigation bug\")"</li>
                        <li>"Columns highlight when you drag a card over them"</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
