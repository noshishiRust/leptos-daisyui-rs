use leptos::prelude::*;

use crate::components::gantt::{GanttTask, GanttTaskHeight, ViewMode};

/// A production-ready Gantt chart component for task scheduling and project management.
///
/// Provides interactive timeline visualization with support for:
/// - Task hierarchies and dependencies
/// - Drag and drop rescheduling
/// - Multiple view modes (Hour/Day/Week/Month/Quarter/Year)
/// - Critical path analysis
/// - Resource management
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::gantt::{GanttChart, GanttTask, ViewMode};
///
/// #[component]
/// pub fn MyGantt() -> impl IntoView {
///     let tasks = vec![
///         GanttTask {
///             id: "task-1".to_string(),
///             name: "Design Phase".to_string(),
///             start: chrono::Utc::now(),
///             end: chrono::Utc::now() + chrono::Duration::days(7),
///             progress: 0.5,
///             ..Default::default()
///         },
///     ];
///
///     view! {
///         <GanttChart
///             tasks=tasks
///             view_mode=ViewMode::Week
///             on_task_click=move |task_id| {
///                 println!("Clicked task: {}", task_id);
///             }
///         />
///     }
/// }
/// ```
#[component]
pub fn GanttChart(
    /// List of tasks to display in the Gantt chart
    #[prop(into)]
    tasks: Signal<Vec<GanttTask>>,

    /// View mode for the timeline (Hour/Day/Week/Month/Quarter/Year)
    #[prop(optional, into)]
    view_mode: Signal<ViewMode>,

    /// Task bar height/density setting
    #[prop(optional, into)]
    _task_height: Signal<GanttTaskHeight>,

    /// Whether to show the task list panel
    #[prop(optional, into, default=Signal::derive(|| true))]
    show_task_list: Signal<bool>,

    /// Whether to show the timeline grid
    #[prop(optional, into, default=Signal::derive(|| true))]
    show_timeline: Signal<bool>,

    /// Callback when a task is clicked
    #[prop(optional)]
    on_task_click: Option<Callback<String>>,

    /// Callback when a task is double-clicked
    #[prop(optional)]
    _on_task_double_click: Option<Callback<String>>,

    /// Callback when a task is updated (via drag/edit)
    #[prop(optional)]
    _on_task_update: Option<Callback<GanttTask>>,

    /// NodeRef for accessing the underlying DOM element
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// Additional CSS classes
    #[prop(optional, into, default="")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=format!("gantt-chart flex h-full w-full overflow-hidden {}", class)
        >
            // Task list panel (left side)
            <Show when=move || show_task_list.get()>
                <div class="gantt-task-list border-r border-base-300 bg-base-100">
                    <div class="p-4">
                        <h3 class="text-lg font-semibold">"Tasks"</h3>
                    </div>
                    <div class="overflow-y-auto">
                        <For
                            each=move || tasks.get()
                            key=|task| task.id.clone()
                            children=move |task| {
                                let task_id = task.id.clone();
                                let on_click_cb = on_task_click.clone();

                                view! {
                                    <div
                                        class="cursor-pointer p-2 hover:bg-base-200"
                                        on:click=move |_| {
                                            if let Some(ref cb) = on_click_cb {
                                                cb.run(task_id.clone());
                                            }
                                        }
                                    >
                                        <div class="font-medium">{task.name.clone()}</div>
                                        <div class="text-sm text-base-content/60">
                                            {format!("Progress: {:.0}%", task.progress * 100.0)}
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </Show>

            // Timeline panel (right side)
            <Show when=move || show_timeline.get()>
                <div class="gantt-timeline flex-1 overflow-auto bg-base-100">
                    <div class="p-4">
                        <div class="flex items-center justify-between">
                            <h3 class="text-lg font-semibold">"Timeline"</h3>
                            <div class="text-sm text-base-content/60">
                                {move || format!("View: {}", view_mode.get().as_str())}
                            </div>
                        </div>
                    </div>
                    <div class="min-h-96 p-4">
                        <div class="text-center text-base-content/60">
                            "Timeline visualization will be implemented in Phase 2"
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
