use chrono::Utc;
use leptos::prelude::*;
use leptos::ev::MouseEvent;
use web_sys::WheelEvent;

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

    /// Initial view mode for the timeline (Hour/Day/Week/Month/Quarter/Year)
    #[prop(optional, into, default=Signal::derive(|| ViewMode::Day))]
    initial_view_mode: Signal<ViewMode>,

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

    /// Callback when a task is selected
    #[prop(optional)]
    on_task_select: Option<Callback<String>>,

    /// Callback when the view mode changes (zoom)
    #[prop(optional)]
    on_view_mode_change: Option<Callback<ViewMode>>,

    /// NodeRef for accessing the underlying DOM element
    #[prop(optional)]
    _node_ref: NodeRef<leptos::html::Div>,

    /// Additional CSS classes
    #[prop(optional, into, default="")]
    class: &'static str,

    /// Initial split ratio for task list panel (0.0-1.0, default 0.3 = 30%)
    #[prop(optional, default=0.3)]
    initial_split_ratio: f64,
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

    // Selection state management
    let (selected_task_id, set_selected_task_id) = signal::<Option<String>>(None);

    // View mode state management
    let (view_mode, set_view_mode) = signal(initial_view_mode.get());

    // Zoom controls
    let zoom_in = move |_| {
        let current = view_mode.get();
        if current.can_zoom_in() {
            let new_mode = current.zoom_in();
            set_view_mode.set(new_mode);
            if let Some(ref cb) = on_view_mode_change {
                cb.run(new_mode);
            }
        }
    };

    let zoom_out = move |_| {
        let current = view_mode.get();
        if current.can_zoom_out() {
            let new_mode = current.zoom_out();
            set_view_mode.set(new_mode);
            if let Some(ref cb) = on_view_mode_change {
                cb.run(new_mode);
            }
        }
    };

    // Split panel state management
    let (split_ratio, set_split_ratio) = signal(initial_split_ratio);
    let (is_dragging, set_is_dragging) = signal(false);
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Minimum panel widths in pixels
    const MIN_TASK_LIST_WIDTH: f64 = 200.0;
    const MIN_TIMELINE_WIDTH: f64 = 200.0;

    // Mouse event handlers for splitter dragging
    let on_splitter_mousedown = move |e: MouseEvent| {
        e.prevent_default();
        set_is_dragging.set(true);
    };

    let on_container_mousemove = move |e: MouseEvent| {
        if is_dragging.get()
            && let Some(container) = container_ref.get() {
                let rect = container.get_bounding_client_rect();
                let container_width = rect.width();
                let mouse_x = e.client_x() as f64 - rect.left();

                // Calculate new ratio with constraints
                let mut new_ratio = mouse_x / container_width;

                // Apply minimum width constraints
                let min_left_ratio = MIN_TASK_LIST_WIDTH / container_width;
                let max_left_ratio = 1.0 - (MIN_TIMELINE_WIDTH / container_width);

                new_ratio = new_ratio.clamp(min_left_ratio, max_left_ratio);

                set_split_ratio.set(new_ratio);
            }
    };

    let on_container_mouseup = move |_e: MouseEvent| {
        set_is_dragging.set(false);
    };

    let on_container_mouseleave = move |_e: MouseEvent| {
        set_is_dragging.set(false);
    };

    // Mouse wheel zoom handler
    let on_wheel_zoom = move |e: WheelEvent| {
        // Only zoom when Ctrl is held (standard zoom behavior)
        if e.ctrl_key() {
            e.prevent_default();

            let delta_y = e.delta_y();
            let current = view_mode.get();

            if delta_y < 0.0 && current.can_zoom_in() {
                // Scroll up = zoom in
                let new_mode = current.zoom_in();
                set_view_mode.set(new_mode);
                if let Some(ref cb) = on_view_mode_change {
                    cb.run(new_mode);
                }
            } else if delta_y > 0.0 && current.can_zoom_out() {
                // Scroll down = zoom out
                let new_mode = current.zoom_out();
                set_view_mode.set(new_mode);
                if let Some(ref cb) = on_view_mode_change {
                    cb.run(new_mode);
                }
            }
        }
    };

    view! {
        <div
            node_ref=container_ref
            class=format!("gantt-chart flex h-full w-full overflow-hidden select-none {}", class)
            on:mousemove=on_container_mousemove
            on:mouseup=on_container_mouseup
            on:mouseleave=on_container_mouseleave
        >
            // Task list panel (left side)
            <Show when=move || show_task_list.get()>
                <div
                    class="gantt-task-list border-r border-base-300 bg-base-100"
                    style=move || format!("width: {}%; flex-shrink: 0;", split_ratio.get() * 100.0)
                >
                    <div class="border-b border-base-300 bg-base-200 p-4">
                        <h3 class="text-lg font-semibold">"Tasks"</h3>
                    </div>
                    <div class="overflow-y-auto" style="height: calc(100vh - 200px)">
                        <For
                            each=move || tasks.get()
                            key=|task| task.id.clone()
                            children=move |task| {
                                let task_id = task.id.clone();
                                let task_id_for_select = task_id.clone();
                                let task_id_for_check = task_id.clone();
                                let on_click_cb = on_task_click;
                                let on_select_cb = on_task_select;

                                let is_selected = Signal::derive(move || {
                                    selected_task_id.get().as_ref() == Some(&task_id_for_check)
                                });

                                view! {
                                    <div
                                        class="cursor-pointer border-b border-base-200 p-3 transition-colors"
                                        class:bg-primary=move || is_selected.get()
                                        class:text-primary-content=move || is_selected.get()
                                        class:hover:bg-base-200=move || !is_selected.get()
                                        style="height: 50px"
                                        on:click=move |_| {
                                            set_selected_task_id.set(Some(task_id_for_select.clone()));
                                            if let Some(ref cb) = on_select_cb {
                                                cb.run(task_id.clone());
                                            }
                                            if let Some(ref cb) = on_click_cb {
                                                cb.run(task_id.clone());
                                            }
                                        }
                                    >
                                        <div class="font-medium">{task.name.clone()}</div>
                                        <div class="text-xs"
                                            class:text-base-content=move || !is_selected.get()
                                            class:opacity-60=move || !is_selected.get()
                                            class:text-primary-content=move || is_selected.get()
                                        >
                                            {format!("Progress: {:.0}%", task.progress * 100.0)}
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>

                // Draggable splitter bar
                <div
                    class="gantt-splitter group relative cursor-col-resize"
                    class:bg-primary=move || is_dragging.get()
                    style="width: 4px; flex-shrink: 0; z-index: 10;"
                    on:mousedown=on_splitter_mousedown
                >
                    <div
                        class="absolute inset-y-0 -inset-x-1 bg-base-content opacity-0 transition-opacity group-hover:opacity-10"
                        class:opacity-20=move || is_dragging.get()
                    />
                </div>
            </Show>

            // Timeline panel (right side)
            <Show when=move || show_timeline.get()>
                <div
                    class="gantt-timeline flex-1 bg-base-100 flex flex-col"
                    on:wheel=on_wheel_zoom
                >
                    // Zoom controls and timeline scale container
                    <div class="relative">
                        // Zoom controls (floating on top-right)
                        <div class="absolute top-2 right-2 z-20 flex gap-1">
                            <button
                                class="btn btn-sm btn-circle"
                                class:btn-disabled=move || !view_mode.get().can_zoom_in()
                                disabled=move || !view_mode.get().can_zoom_in()
                                on:click=zoom_in
                                title="Zoom in"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0zM10 7v3m0 0v3m0-3h3m-3 0H7" />
                                </svg>
                            </button>
                            <button
                                class="btn btn-sm btn-circle"
                                class:btn-disabled=move || !view_mode.get().can_zoom_out()
                                disabled=move || !view_mode.get().can_zoom_out()
                                on:click=zoom_out
                                title="Zoom out"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0zM13 10H7" />
                                </svg>
                            </button>
                            <div class="badge badge-sm ml-1 self-center">
                                {move || view_mode.get().as_str().to_uppercase()}
                            </div>
                        </div>

                        // Timeline scale headers
                        <TimelineScale
                            start_date=start_date
                            end_date=end_date
                            view_mode=Signal::derive(move || view_mode.get())
                        />
                    </div>

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
                                        let task_id = task.id.clone();
                                        let task_id_for_check = task_id.clone();
                                        let task_signal = Signal::derive(move || task.clone());
                                        let tasks_list = tasks.get();
                                        let task_idx = tasks_list
                                            .iter()
                                            .position(|t| t.id == task_id)
                                            .unwrap_or(0);
                                        let y_pos = Signal::derive(move || (task_idx as u32) * 50);

                                        let is_selected = Signal::derive(move || {
                                            selected_task_id.get().as_ref() == Some(&task_id_for_check)
                                        });

                                        let on_click_cb = on_task_click;
                                        let on_select_cb = on_task_select;

                                        view! {
                                            <TaskBar
                                                task=task_signal
                                                timeline_start=start_date
                                                y_position=y_pos
                                                is_selected=is_selected
                                                on_click=Some(Callback::new(move |id: String| {
                                                    set_selected_task_id.set(Some(id.clone()));
                                                    if let Some(ref cb) = on_select_cb {
                                                        cb.run(id.clone());
                                                    }
                                                    if let Some(ref cb) = on_click_cb {
                                                        cb.run(id);
                                                    }
                                                }))
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
