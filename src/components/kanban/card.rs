use leptos::prelude::*;

use super::types::*;

/// Kanban card component
#[component]
pub fn KanbanCardView(
    /// Card data
    #[prop(into)]
    card: Signal<KanbanCard>,

    /// Callback when card is clicked
    #[prop(into)]
    on_click: Option<Callback<String>>,

    /// Callback when card should be deleted
    #[prop(into)]
    on_delete: Option<Callback<String>>,
) -> impl IntoView {
    let card_id = Signal::derive(move || card.get().card_id.clone());
    let is_overdue = Signal::derive(move || card.get().is_overdue());

    view! {
        <div
            class="kanban-card bg-base-100 rounded-lg p-3 shadow-sm hover:shadow-md transition-shadow cursor-pointer"
            class:border-l-4=move || is_overdue.get()
            class:border-error=move || is_overdue.get()
            on:click=move |_| {
                if let Some(ref cb) = on_click {
                    cb.run(card_id.get());
                }
            }
        >
            // Card header with title and delete button
            <div class="flex items-start justify-between gap-2 mb-2">
                <h4 class="font-semibold text-sm flex-1">
                    {move || card.get().title}
                </h4>

                <Show when=move || on_delete.is_some()>
                    <button
                        class="btn btn-ghost btn-xs btn-circle opacity-0 group-hover:opacity-100"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            if let Some(ref cb) = on_delete {
                                cb.run(card_id.get());
                            }
                        }
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-4 h-4"
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

            // Description (if present)
            <Show when=move || card.get().description.is_some()>
                <p class="text-xs text-base-content/70 mb-2">
                    {move || card.get().description.clone().unwrap_or_default()}
                </p>
            </Show>

            // Labels
            <Show when=move || !card.get().labels.is_empty()>
                <div class="flex flex-wrap gap-1 mb-2">
                    <For
                        each=move || card.get().labels
                        key=|label| label.id.clone()
                        children=move |label| {
                            view! {
                                <span class="badge badge-sm" style=move || {
                                    label.color.as_ref().map(|c| format!("background-color: {}", c))
                                }>
                                    {label.name}
                                </span>
                            }
                        }
                    />
                </div>
            </Show>

            // Card footer with metadata
            <div class="flex items-center justify-between text-xs">
                // Priority badge
                <span class=move || {
                    format!("badge badge-xs {}", card.get().priority.color_class())
                }>
                    {move || card.get().priority.as_str()}
                </span>

                // Due date
                <Show when=move || card.get().due_date.is_some()>
                    <span
                        class=move || {
                            if is_overdue.get() {
                                "text-xs text-error"
                            } else {
                                "text-xs opacity-60"
                            }
                        }
                    >
                        {move || card.get().due_date.map(|d| d.to_string()).unwrap_or_default()}
                    </span>
                </Show>
            </div>

            // Assignees (avatars)
            <Show when=move || !card.get().assignees.is_empty()>
                <div class="flex -space-x-2 mt-2">
                    <For
                        each=move || card.get().assignees.into_iter().take(3)
                        key=|assignee| assignee.id.clone()
                        children=move |assignee| {
                            let avatar_url = assignee.avatar_url.clone();
                            let initials = assignee.initials.clone();
                            let name = assignee.name.clone();
                            let avatar = avatar_url.clone().unwrap_or_default();
                            let person_name = name.clone();
                            let person_initials = initials.clone();
                            view! {
                                <div class="avatar placeholder">
                                    <div class="bg-neutral text-neutral-content rounded-full w-6">
                                        {if !avatar.is_empty() {
                                            view! {
                                                <img src=avatar alt=person_name />
                                            }.into_any()
                                        } else {
                                            view! {
                                                <span class="text-xs">{person_initials}</span>
                                            }.into_any()
                                        }}
                                    </div>
                                </div>
                            }
                        }
                    />

                    // Show count if more than 3 assignees
                    <Show when=move || { card.get().assignees.len() > 3 }>
                        <div class="avatar placeholder">
                            <div class="bg-neutral text-neutral-content rounded-full w-6">
                                <span class="text-xs">
                                    {move || format!("+{}", card.get().assignees.len() - 3)}
                                </span>
                            </div>
                        </div>
                    </Show>
                </div>
            </Show>
        </div>
    }
}
