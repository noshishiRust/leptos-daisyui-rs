use chrono::{DateTime, Utc};
use leptos::prelude::*;

use crate::components::gantt::GanttTask;

/// Task bar component that renders a horizontal bar on the timeline
#[component]
pub fn TaskBar(
    /// The task to render
    #[prop(into)]
    task: Signal<GanttTask>,

    /// Start date of the timeline (for calculating position)
    #[prop(into)]
    timeline_start: Signal<DateTime<Utc>>,

    /// Width per day in pixels
    #[prop(into, default=Signal::derive(|| 60))]
    column_width: Signal<u32>,

    /// Height of the task bar in pixels
    #[prop(into, default=Signal::derive(|| 30))]
    bar_height: Signal<u32>,

    /// Y-position of the task bar
    #[prop(into, default=Signal::derive(|| 0))]
    y_position: Signal<u32>,

    /// Callback when the task bar is clicked
    #[prop(into, default=None)]
    on_click: Option<Callback<String>>,

    /// Whether this task is currently selected
    #[prop(into, default=Signal::derive(|| false))]
    is_selected: Signal<bool>,

    /// Whether this task is read-only (non-editable)
    #[prop(into, default=Signal::derive(|| false))]
    is_read_only: Signal<bool>,
) -> impl IntoView {
    let position_width = Signal::derive(move || {
        let t = task.get();
        let timeline_start_val = timeline_start.get();
        let col_width = column_width.get();

        calculate_bar_position(&t, timeline_start_val, col_width)
    });

    let bar_color = Signal::derive(move || {
        let t = task.get();
        t.color
            .clone()
            .unwrap_or_else(|| "#3b82f6".to_string()) // Default blue
    });

    view! {
        <div
            class="task-bar absolute rounded transition-all"
            class:cursor-pointer=move || !is_read_only.get()
            class:cursor-not-allowed=move || is_read_only.get()
            class:opacity-60=move || is_read_only.get()
            class:hover:shadow-lg=move || !is_read_only.get()
            class:ring-2=move || is_selected.get()
            class:ring-primary=move || is_selected.get()
            class:ring-offset-2=move || is_selected.get()
            class:shadow-xl=move || is_selected.get()
            style:left=move || format!("{}px", position_width.get().0)
            style:width=move || format!("{}px", position_width.get().1)
            style:top=move || format!("{}px", y_position.get())
            style:height=move || format!("{}px", bar_height.get())
            style:background-color=move || bar_color.get()
            attr:aria-readonly=move || is_read_only.get().to_string()
            on:click=move |_| {
                if !is_read_only.get() {
                    if let Some(ref cb) = on_click {
                        cb.run(task.get().id.clone());
                    }
                }
            }
        >
            <div class="relative h-full">
                // Progress bar overlay
                <div
                    class="progress-overlay absolute left-0 top-0 h-full rounded bg-black/20"
                    style:width=move || format!("{}%", task.get().progress * 100.0)
                />

                // Task name label
                <div class="task-label absolute left-0 top-0 flex h-full items-center px-2 text-xs font-medium text-white">
                    {move || task.get().name.clone()}
                </div>

                // Read-only lock icon indicator
                <Show when=move || is_read_only.get()>
                    <div class="absolute right-1 top-1/2 -translate-y-1/2 text-white/80">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="12"
                            height="12"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <rect width="18" height="11" x="3" y="11" rx="2" ry="2"/>
                            <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                        </svg>
                    </div>
                </Show>
            </div>
        </div>
    }
}

/// Calculate the X position and width of a task bar based on its dates
fn calculate_bar_position(
    task: &GanttTask,
    timeline_start: DateTime<Utc>,
    column_width: u32,
) -> (u32, u32) {
    // Calculate days from timeline start to task start
    let days_to_start = (task.start - timeline_start).num_days();
    let x_position = (days_to_start as u32) * column_width;

    // Calculate task duration in days
    let duration_days = (task.end - task.start).num_days();
    let width = (duration_days as u32) * column_width;

    (x_position, width.max(column_width / 2)) // Minimum width for visibility
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_calculate_bar_position() {
        let timeline_start = chrono::Utc
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .unwrap();

        let task = GanttTask {
            id: "task1".to_string(),
            name: "Test Task".to_string(),
            start: chrono::Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap(),
            end: chrono::Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap(),
            progress: 0.5,
            ..Default::default()
        };

        let (x, width) = calculate_bar_position(&task, timeline_start, 60);

        // Task starts 4 days after timeline start
        assert_eq!(x, 4 * 60);

        // Task duration is 5 days
        assert_eq!(width, 5 * 60);
    }

    #[test]
    fn test_calculate_bar_position_minimum_width() {
        let timeline_start = chrono::Utc
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .unwrap();

        let task = GanttTask {
            id: "task1".to_string(),
            name: "Milestone".to_string(),
            start: chrono::Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap(),
            end: chrono::Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap(), // Same day
            progress: 0.0,
            ..Default::default()
        };

        let (_, width) = calculate_bar_position(&task, timeline_start, 60);

        // Minimum width is half of column width
        assert_eq!(width, 30);
    }
}
