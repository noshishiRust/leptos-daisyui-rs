use chrono::{Duration, Utc};
use leptos::prelude::*;
use leptos_daisyui_rs::components::gantt::*;

#[component]
pub fn GanttDemo() -> impl IntoView {
    // Create sample project data showcasing all features
    let (tasks, set_tasks) = signal(create_sample_tasks());
    let (view_mode, set_view_mode) = signal(ViewMode::Day);

    view! {
        <div class="p-6 space-y-6">
            <div class="prose max-w-none">
                <h1>"Gantt Chart Component"</h1>
                <p>"A production-ready Gantt chart for project management and task scheduling."</p>
            </div>

            // View mode selector
            <div class="card bg-base-200 p-4">
                <h3 class="font-semibold mb-3">"View Mode"</h3>
                <div class="join">
                    <button
                        class="btn btn-sm join-item"
                        class:btn-active=move || view_mode.get() == ViewMode::Hour
                        on:click=move |_| set_view_mode.set(ViewMode::Hour)
                    >
                        "Hour"
                    </button>
                    <button
                        class="btn btn-sm join-item"
                        class:btn-active=move || view_mode.get() == ViewMode::Day
                        on:click=move |_| set_view_mode.set(ViewMode::Day)
                    >
                        "Day"
                    </button>
                    <button
                        class="btn btn-sm join-item"
                        class:btn-active=move || view_mode.get() == ViewMode::Week
                        on:click=move |_| set_view_mode.set(ViewMode::Week)
                    >
                        "Week"
                    </button>
                    <button
                        class="btn btn-sm join-item"
                        class:btn-active=move || view_mode.get() == ViewMode::Month
                        on:click=move |_| set_view_mode.set(ViewMode::Month)
                    >
                        "Month"
                    </button>
                    <button
                        class="btn btn-sm join-item"
                        class:btn-active=move || view_mode.get() == ViewMode::Quarter
                        on:click=move |_| set_view_mode.set(ViewMode::Quarter)
                    >
                        "Quarter"
                    </button>
                </div>
            </div>

            // Gantt chart
            <div class="card bg-base-100 shadow-xl">
                <div class="card-body p-0">
                    <div style="height: 600px">
                        <GanttChart
                            tasks=tasks
                            view_mode=view_mode
                            show_task_list=true
                            show_dependencies=false
                            show_today_marker=true
                        />
                    </div>
                </div>
            </div>

            // Features showcase
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="card bg-base-200">
                    <div class="card-body">
                        <h3 class="card-title text-lg">"Features"</h3>
                        <ul class="list-disc list-inside space-y-1 text-sm">
                            <li>"Interactive timeline with multiple view modes"</li>
                            <li>"Hierarchical task tree structure"</li>
                            <li>"Drag-and-drop task scheduling"</li>
                            <li>"Task dependencies (FS, SS, FF, SF)"</li>
                            <li>"Progress tracking and milestones"</li>
                            <li>"Zoom and pan controls"</li>
                            <li>"Today marker and weekend shading"</li>
                            <li>"Responsive split-panel layout"</li>
                        </ul>
                    </div>
                </div>

                <div class="card bg-base-200">
                    <div class="card-body">
                        <h3 class="card-title text-lg">"Sample Data"</h3>
                        <div class="text-sm space-y-2">
                            <p><strong>"Project:"</strong> " Website Redesign"</p>
                            <p><strong>"Tasks:"</strong> {move || tasks.get().len()}</p>
                            <p><strong>"Task Types:"</strong> " Regular, Milestones, Projects"</p>
                            <p><strong>"Dependencies:"</strong> " Finish-to-Start relationships"</p>
                        </div>
                    </div>
                </div>
            </div>

            // Code example
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Usage Example"</h3>
                    <div class="mockup-code text-xs">
                        <pre data-prefix="1"><code>"use leptos::prelude::*;"</code></pre>
                        <pre data-prefix="2"><code>"use leptos_daisyui_rs::components::gantt::*;"</code></pre>
                        <pre data-prefix="3"><code></code></pre>
                        <pre data-prefix="4"><code>"#[component]"</code></pre>
                        <pre data-prefix="5"><code>"pub fn App() -> impl IntoView {"</code></pre>
                        <pre data-prefix="6"><code>"    let (tasks, set_tasks) = signal(vec!["</code></pre>
                        <pre data-prefix="7"><code>"        GanttTask {"</code></pre>
                        <pre data-prefix="8"><code>"            id: \"task-1\".to_string(),"</code></pre>
                        <pre data-prefix="9"><code>"            name: \"Implement feature\".to_string(),"</code></pre>
                        <pre data-prefix="10"><code>"            start: Utc::now(),"</code></pre>
                        <pre data-prefix="11"><code>"            end: Utc::now() + Duration::days(5),"</code></pre>
                        <pre data-prefix="12"><code>"            progress: 0.5,"</code></pre>
                        <pre data-prefix="13"><code>"            task_type: TaskType::Task,"</code></pre>
                        <pre data-prefix="14"><code>"            // ... other fields"</code></pre>
                        <pre data-prefix="15"><code>"        },"</code></pre>
                        <pre data-prefix="16"><code>"    ]);"</code></pre>
                        <pre data-prefix="17"><code></code></pre>
                        <pre data-prefix="18"><code>"    view! {"</code></pre>
                        <pre data-prefix="19"><code>"        <GanttChart"</code></pre>
                        <pre data-prefix="20"><code>"            tasks=tasks"</code></pre>
                        <pre data-prefix="21"><code>"            view_mode=ViewMode::Day"</code></pre>
                        <pre data-prefix="22"><code>"            show_today_marker=true"</code></pre>
                        <pre data-prefix="23"><code>"        />"</code></pre>
                        <pre data-prefix="24"><code>"    }"</code></pre>
                        <pre data-prefix="25"><code>"}"</code></pre>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Create comprehensive sample tasks showcasing all features
fn create_sample_tasks() -> Vec<GanttTask> {
    let now = Utc::now();
    let start_date = now - Duration::days(5);

    vec![
        // Project container
        GanttTask {
            id: "project-1".to_string(),
            name: "Website Redesign".to_string(),
            start: start_date,
            end: start_date + Duration::days(30),
            progress: 0.35,
            task_type: TaskType::Project,
            parent_id: None,
            dependencies: vec![],
            assignees: vec!["team@company.com".to_string()],
            color: Some("#8b5cf6".to_string()),
            read_only: false,
            metadata: Default::default(),
        },
        // Phase 1: Planning
        GanttTask {
            id: "task-1".to_string(),
            name: "Planning Phase".to_string(),
            start: start_date,
            end: start_date + Duration::days(5),
            progress: 1.0,
            task_type: TaskType::Task,
            parent_id: Some("project-1".to_string()),
            dependencies: vec![],
            assignees: vec!["pm@company.com".to_string()],
            color: Some("#3b82f6".to_string()),
            read_only: false,
            metadata: Default::default(),
        },
        GanttTask {
            id: "task-2".to_string(),
            name: "Requirements Gathering".to_string(),
            start: start_date,
            end: start_date + Duration::days(3),
            progress: 1.0,
            task_type: TaskType::Task,
            parent_id: Some("task-1".to_string()),
            dependencies: vec![],
            assignees: vec!["analyst@company.com".to_string()],
            color: None,
            read_only: false,
            metadata: Default::default(),
        },
        GanttTask {
            id: "milestone-1".to_string(),
            name: "Requirements Approved".to_string(),
            start: start_date + Duration::days(5),
            end: start_date + Duration::days(5),
            progress: 1.0,
            task_type: TaskType::Milestone,
            parent_id: Some("task-1".to_string()),
            dependencies: vec!["task-2".to_string()],
            assignees: vec![],
            color: Some("#10b981".to_string()),
            read_only: false,
            metadata: Default::default(),
        },
        // Phase 2: Design
        GanttTask {
            id: "task-3".to_string(),
            name: "Design Phase".to_string(),
            start: start_date + Duration::days(5),
            end: start_date + Duration::days(15),
            progress: 0.6,
            task_type: TaskType::Task,
            parent_id: Some("project-1".to_string()),
            dependencies: vec!["milestone-1".to_string()],
            assignees: vec!["designer@company.com".to_string()],
            color: Some("#ec4899".to_string()),
            read_only: false,
            metadata: Default::default(),
        },
        GanttTask {
            id: "task-4".to_string(),
            name: "UI Mockups".to_string(),
            start: start_date + Duration::days(6),
            end: start_date + Duration::days(11),
            progress: 1.0,
            task_type: TaskType::Task,
            parent_id: Some("task-3".to_string()),
            dependencies: vec![],
            assignees: vec!["ui-designer@company.com".to_string()],
            color: None,
            read_only: false,
            metadata: Default::default(),
        },
        GanttTask {
            id: "task-5".to_string(),
            name: "Design System".to_string(),
            start: start_date + Duration::days(10),
            end: start_date + Duration::days(15),
            progress: 0.4,
            task_type: TaskType::Task,
            parent_id: Some("task-3".to_string()),
            dependencies: vec!["task-4".to_string()],
            assignees: vec!["designer@company.com".to_string()],
            color: None,
            read_only: false,
            metadata: Default::default(),
        },
        // Phase 3: Development
        GanttTask {
            id: "task-6".to_string(),
            name: "Development Phase".to_string(),
            start: start_date + Duration::days(15),
            end: start_date + Duration::days(28),
            progress: 0.2,
            task_type: TaskType::Task,
            parent_id: Some("project-1".to_string()),
            dependencies: vec!["task-3".to_string()],
            assignees: vec!["dev-team@company.com".to_string()],
            color: Some("#f59e0b".to_string()),
            read_only: false,
            metadata: Default::default(),
        },
        GanttTask {
            id: "task-7".to_string(),
            name: "Frontend Development".to_string(),
            start: start_date + Duration::days(16),
            end: start_date + Duration::days(24),
            progress: 0.3,
            task_type: TaskType::Task,
            parent_id: Some("task-6".to_string()),
            dependencies: vec![],
            assignees: vec!["frontend-dev@company.com".to_string()],
            color: None,
            read_only: false,
            metadata: Default::default(),
        },
        GanttTask {
            id: "task-8".to_string(),
            name: "Backend API".to_string(),
            start: start_date + Duration::days(16),
            end: start_date + Duration::days(22),
            progress: 0.5,
            task_type: TaskType::Task,
            parent_id: Some("task-6".to_string()),
            dependencies: vec![],
            assignees: vec!["backend-dev@company.com".to_string()],
            color: None,
            read_only: false,
            metadata: Default::default(),
        },
        GanttTask {
            id: "milestone-2".to_string(),
            name: "Development Complete".to_string(),
            start: start_date + Duration::days(28),
            end: start_date + Duration::days(28),
            progress: 0.0,
            task_type: TaskType::Milestone,
            parent_id: Some("task-6".to_string()),
            dependencies: vec!["task-7".to_string(), "task-8".to_string()],
            assignees: vec![],
            color: Some("#10b981".to_string()),
            read_only: false,
            metadata: Default::default(),
        },
        // Phase 4: Testing
        GanttTask {
            id: "task-9".to_string(),
            name: "Testing & QA".to_string(),
            start: start_date + Duration::days(25),
            end: start_date + Duration::days(30),
            progress: 0.0,
            task_type: TaskType::Task,
            parent_id: Some("project-1".to_string()),
            dependencies: vec!["milestone-2".to_string()],
            assignees: vec!["qa@company.com".to_string()],
            color: Some("#ef4444".to_string()),
            read_only: false,
            metadata: Default::default(),
        },
        GanttTask {
            id: "milestone-3".to_string(),
            name: "Launch Ready".to_string(),
            start: start_date + Duration::days(30),
            end: start_date + Duration::days(30),
            progress: 0.0,
            task_type: TaskType::Milestone,
            parent_id: Some("project-1".to_string()),
            dependencies: vec!["task-9".to_string()],
            assignees: vec![],
            color: Some("#10b981".to_string()),
            read_only: false,
            metadata: Default::default(),
        },
    ]
}
