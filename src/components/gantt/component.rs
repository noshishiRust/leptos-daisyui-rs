use chrono::Utc;
use leptos::prelude::*;

use crate::components::gantt::{
    timeline::{TaskBar, TimelineGrid, TimelineScale},
    GanttTask, GanttTaskHeight, ViewMode,
};

/// A production-ready Gantt chart component for task scheduling and project management.
#[component]
pub fn GanttChart(
    /// List of tasks to display in the Gantt chart
    #[prop(into)]
    tasks: Signal<Vec<GanttTask>>,

    /// View mode for the timeline (Hour/Day/Week/Month/Quarter/Year)
    #[prop(optional, into, default=Signal::derive(|| ViewMode::Day))]
    view_mode: Signal<ViewMode>,

    /// Task bar height/density setting
    #[prop(optional, into, default=Signal::derive(|| GanttTaskHeight::Medium))]
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

    /// NodeRef for accessing the underlying DOM element
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// Additional CSS classes
    #[prop(optional, into, default="")]
    class: &'static str,
) -> impl IntoView {
    // Calculate timeline date range from tasks
    let date_range = Signal::derive(move || {
        let task_list = tasks.get();
        if task_list.is_empty() {
            let now = Utc::now();
            (now, now + chrono::Duration::days(30))
        } else {
            let start = task_list
                .iter()
                .map(|t| t.start)
                .min()
                .unwrap_or_else(Utc::now);
            let end = task_list
                .iter()
                .map(|t| t.end)
                .max()
                .unwrap_or_else(|| Utc::now() + chrono::Duration::days(30));

            (start, end)
        }
    });

    let start_date = Signal::derive(move || date_range.get().0);
    let end_date = Signal::derive(move || date_range.get().1);

    view! {
        <div
            node_ref=node_ref
            class=format!("gantt-chart flex h-full w-full overflow-hidden {}", class)
        >
            // Task list panel (left side)
            <Show when=move || show_task_list.get()>
                <div class="gantt-task-list w-64 border-r border-base-300 bg-base-100">
                    <div class="border-b border-base-300 bg-base-200 p-4">
                        <h3 class="text-lg font-semibold">"Tasks"</h3>
                    </div>
                    <div class="overflow-y-auto" style="height: calc(100vh - 200px)">
                        <For
                            each=move || tasks.get()
                            key=|task| task.id.clone()
                            children=move |task| {
                                let task_id = task.id.clone();
                                let on_click_cb = on_task_click;

                                view! {
                                    <div
                                        class="cursor-pointer border-b border-base-200 p-3 hover:bg-base-200"
                                        style="height: 50px"
                                        on:click=move |_| {
                                            if let Some(ref cb) = on_click_cb {
                                                cb.run(task_id.clone());
                                            }
                                        }
                                    >
                                        <div class="font-medium">{task.name.clone()}</div>
                                        <div class="text-xs text-base-content/60">
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
                <div class="gantt-timeline flex-1 bg-base-100">
                    // Timeline scale headers
                    <TimelineScale
                        start_date=start_date
                        end_date=end_date
                        view_mode=view_mode
                    />

                    // Timeline content with grid and task bars
                    <div class="overflow-auto" style="height: calc(100vh - 200px)">
                        <div class="relative">
                            // Background grid
                            <TimelineGrid
                                start_date=start_date
                                end_date=end_date
                                view_mode=view_mode
                            />

                            // Task bars overlaid on grid
                            <div class="absolute left-0 top-0">
                                <For
                                    each=move || tasks.get()
                                    key=|task| task.id.clone()
                                    children=move |task| {
                                        let task_signal = Signal::derive(move || task.clone());
                                        let tasks_list = tasks.get();
                                        let task_idx = tasks_list
                                            .iter()
                                            .position(|t| t.id == task.id)
                                            .unwrap_or(0);
                                        let y_pos = Signal::derive(move || (task_idx as u32) * 50);

                                        view! {
                                            <TaskBar
                                                task=task_signal
                                                timeline_start=start_date
                                                y_position=y_pos
                                                on_click=on_task_click
                                            />
                                        }
                                    }
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
