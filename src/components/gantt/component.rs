use chrono::Utc;
use leptos::prelude::*;
use leptos::ev::{KeyboardEvent, MouseEvent};
use web_sys::WheelEvent;

use crate::components::gantt::{
    timeline::{TaskBar, TimelineGrid, TimelineScale},
    utils::{AccessibleAnnouncement, LiveRegion, task_aria_label, zoom_aria_label},
    EditContext, EditType, GanttTask, GanttTaskHeight, ReadOnlyMode, ViewMode,
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

    /// Callback when a task should be deleted
    #[prop(optional)]
    on_task_delete: Option<Callback<String>>,

    /// NodeRef for accessing the underlying DOM element
    #[prop(optional)]
    _node_ref: NodeRef<leptos::html::Div>,

    /// Additional CSS classes
    #[prop(optional, into, default="")]
    class: &'static str,

    /// Initial split ratio for task list panel (0.0-1.0, default 0.3 = 30%)
    #[prop(optional, default=0.3)]
    initial_split_ratio: f64,

    /// Read-only mode configuration
    #[prop(optional, into, default=Signal::derive(|| ReadOnlyMode::default()))]
    read_only_mode: Signal<ReadOnlyMode>,

    /// Simple boolean to enable full read-only mode (convenience prop)
    #[prop(optional, into, default=Signal::derive(|| false))]
    read_only: Signal<bool>,
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
    let (focused_task_index, set_focused_task_index) = signal::<Option<usize>>(None);

    // Screen reader announcements
    let (announcement, set_announcement) = signal::<Option<AccessibleAnnouncement>>(None);

    // View mode state management
    let (view_mode, set_view_mode) = signal(initial_view_mode.get());

    // Read-only mode state management
    let effective_read_only_mode = Signal::derive(move || {
        if read_only.get() {
            ReadOnlyMode::Full
        } else {
            read_only_mode.get()
        }
    });

    // Helper to check if a task edit is allowed
    let is_edit_allowed = move |task_id: &str, edit_type: EditType| {
        // Check per-task read_only flag first
        if let Some(task) = tasks.get().iter().find(|t| t.id == task_id)
            && task.read_only {
                return false;
            }

        // Check global read-only mode
        let context = EditContext::new(task_id.to_string(), edit_type);
        effective_read_only_mode.get().is_edit_allowed(&context)
    };

    // Zoom controls
    let zoom_in = move |_| {
        let current = view_mode.get();
        if current.can_zoom_in() {
            let new_mode = current.zoom_in();
            set_view_mode.set(new_mode);

            // Announce zoom change to screen readers
            let message = format!("Zoomed in to {} view", new_mode.as_str());
            set_announcement.set(Some(AccessibleAnnouncement::polite(message)));

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

            // Announce zoom change to screen readers
            let message = format!("Zoomed out to {} view", new_mode.as_str());
            set_announcement.set(Some(AccessibleAnnouncement::polite(message)));

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

    // Keyboard navigation handler
    let on_keydown = move |e: KeyboardEvent| {
        let task_list = tasks.get();
        if task_list.is_empty() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "ArrowDown" => {
                e.prevent_default();
                let current_idx = focused_task_index.get().unwrap_or(0);
                let new_idx = (current_idx + 1).min(task_list.len() - 1);
                set_focused_task_index.set(Some(new_idx));

                // Announce focused task
                if let Some(task) = task_list.get(new_idx) {
                    let message = format!("Focused on task: {}", task.name);
                    set_announcement.set(Some(AccessibleAnnouncement::polite(message)));
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if let Some(current_idx) = focused_task_index.get() {
                    let new_idx = current_idx.saturating_sub(1);
                    set_focused_task_index.set(Some(new_idx));

                    // Announce focused task
                    if let Some(task) = task_list.get(new_idx) {
                        let message = format!("Focused on task: {}", task.name);
                        set_announcement.set(Some(AccessibleAnnouncement::polite(message)));
                    }
                }
            }
            "Home" => {
                // Home key - focus first task
                e.prevent_default();
                set_focused_task_index.set(Some(0));
                if let Some(task) = task_list.first() {
                    let message = format!("Focused on first task: {}", task.name);
                    set_announcement.set(Some(AccessibleAnnouncement::polite(message)));
                }
            }
            "End" => {
                // End key - focus last task
                e.prevent_default();
                let last_idx = task_list.len().saturating_sub(1);
                set_focused_task_index.set(Some(last_idx));
                if let Some(task) = task_list.get(last_idx) {
                    let message = format!("Focused on last task: {}", task.name);
                    set_announcement.set(Some(AccessibleAnnouncement::polite(message)));
                }
            }
            " " => {
                // Space key - select focused task
                e.prevent_default();
                if let Some(idx) = focused_task_index.get()
                    && let Some(task) = task_list.get(idx) {
                        let task_id = task.id.clone();
                        set_selected_task_id.set(Some(task_id.clone()));

                        // Announce selection
                        let message = format!("Selected task: {}", task.name);
                        set_announcement.set(Some(AccessibleAnnouncement::polite(message)));

                        if let Some(ref cb) = on_task_select {
                            cb.run(task_id);
                        }
                    }
            }
            "Enter" => {
                // Enter key - trigger click callback on focused task
                e.prevent_default();
                if let Some(idx) = focused_task_index.get()
                    && let Some(task) = task_list.get(idx) {
                        let task_id = task.id.clone();

                        // Announce action
                        let message = format!("Opening task: {}", task.name);
                        set_announcement.set(Some(AccessibleAnnouncement::polite(message)));

                        if let Some(ref cb) = on_task_click {
                            cb.run(task_id);
                        }
                    }
            }
            "Escape" => {
                // Escape key - deselect task
                e.prevent_default();
                set_selected_task_id.set(None);
                set_announcement.set(Some(AccessibleAnnouncement::polite("Selection cleared")));
            }
            "Delete" => {
                // Delete key - remove focused task
                e.prevent_default();
                if let Some(idx) = focused_task_index.get()
                    && let Some(task) = task_list.get(idx) {
                        let task_id = task.id.clone();
                        let task_name = task.name.clone();

                        // Announce deletion
                        let message = format!("Deleting task: {}", task_name);
                        set_announcement.set(Some(AccessibleAnnouncement::assertive(message)));

                        if let Some(ref cb) = on_task_delete {
                            cb.run(task_id);
                        }
                    }
            }
            _ => {}
        }
    };

    view! {
        <div
            node_ref=container_ref
            class=format!("gantt-chart flex h-full w-full overflow-hidden select-none {}", class)
            role="application"
            aria-label="Gantt Chart"
            aria-describedby="gantt-instructions"
            tabindex="0"
            on:mousemove=on_container_mousemove
            on:mouseup=on_container_mouseup
            on:mouseleave=on_container_mouseleave
            on:keydown=on_keydown
        >
            // Screen reader live region for announcements
            <LiveRegion announcement=Signal::derive(move || announcement.get()) />

            // Hidden instructions for screen readers
            <div id="gantt-instructions" class="sr-only">
                "Gantt chart for project planning. Use arrow keys to navigate tasks, Space to select, Enter to open, Delete to remove, Home and End to jump to first or last task, Escape to deselect."
            </div>
            // Task list panel (left side)
            <Show when=move || show_task_list.get()>
                <div
                    class="gantt-task-list border-r border-base-300 bg-base-100"
                    style=move || format!("width: {}%; flex-shrink: 0;", split_ratio.get() * 100.0)
                >
                    <div class="border-b border-base-300 bg-base-200 p-4">
                        <h3 id="task-list-heading" class="text-lg font-semibold">"Tasks"</h3>
                    </div>
                    <div
                        class="overflow-y-auto"
                        role="list"
                        aria-labelledby="task-list-heading"
                        style="height: calc(100vh - 200px)"
                    >
                        <For
                            each=move || tasks.get()
                            key=|task| task.id.clone()
                            children=move |task| {
                                let task_id = task.id.clone();
                                let task_id_for_select = task_id.clone();
                                let task_id_for_check = task_id.clone();
                                let task_id_for_aria1 = task_id.clone();
                                let task_id_for_aria2 = task_id.clone();
                                let on_click_cb = on_task_click;
                                let on_select_cb = on_task_select;

                                let is_selected = Signal::derive(move || {
                                    selected_task_id.get().as_ref() == Some(&task_id_for_check)
                                });

                                // Generate ARIA label for this task
                                let task_name = task.name.clone();
                                let task_start = task.start.format("%Y-%m-%d").to_string();
                                let task_end = task.end.format("%Y-%m-%d").to_string();
                                let task_progress = task.progress;
                                let task_readonly = task.read_only;

                                view! {
                                    <div
                                        class="cursor-pointer border-b border-base-200 p-3 transition-colors"
                                        class:bg-primary=move || is_selected.get()
                                        class:text-primary-content=move || is_selected.get()
                                        class:hover:bg-base-200=move || !is_selected.get()
                                        class:ring-2=move || {
                                            if let Some(focused_idx) = focused_task_index.get() {
                                                tasks.get().get(focused_idx).map(|t| t.id == task_id_for_aria1).unwrap_or(false)
                                            } else {
                                                false
                                            }
                                        }
                                        class:ring-offset-2=move || {
                                            if let Some(focused_idx) = focused_task_index.get() {
                                                tasks.get().get(focused_idx).map(|t| t.id == task_id_for_aria2).unwrap_or(false)
                                            } else {
                                                false
                                            }
                                        }
                                        role="listitem"
                                        aria-label=task_aria_label(
                                            &task_name,
                                            &task_start,
                                            &task_end,
                                            task_progress,
                                            is_selected.get_untracked(),
                                            task_readonly
                                        )
                                        aria-selected=move || is_selected.get().to_string()
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
                                aria-label=move || zoom_aria_label("in", view_mode.get().can_zoom_in(), view_mode.get().as_str())
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
                                aria-label=move || zoom_aria_label("out", view_mode.get().can_zoom_out(), view_mode.get().as_str())
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

                                        let task_id_for_readonly = task_id.clone();
                                        let is_task_read_only = Signal::derive(move || {
                                            !is_edit_allowed(&task_id_for_readonly, EditType::Timeline)
                                        });

                                        let on_click_cb = on_task_click;
                                        let on_select_cb = on_task_select;

                                        view! {
                                            <TaskBar
                                                task=task_signal
                                                timeline_start=start_date
                                                y_position=y_pos
                                                is_selected=is_selected
                                                is_read_only=is_task_read_only
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
