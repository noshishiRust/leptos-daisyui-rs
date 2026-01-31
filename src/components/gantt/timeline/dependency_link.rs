/// SVG rendering for task dependency links
///
/// Renders visual connections between dependent tasks as curved bezier paths
/// with appropriate start/end points based on dependency type.
use leptos::prelude::*;

use crate::components::gantt::models::DependencyType;

/// Calculate the SVG path for a dependency link
///
/// # Arguments
///
/// * `source_task_id` - ID of the source task (the dependency)
/// * `target_task_id` - ID of the target task (the dependent task)
/// * `dep_type` - Type of dependency (FS, SS, FF, SF)
/// * `source_x` - X position of source task bar (pixels)
/// * `source_y` - Y position of source task bar (pixels)
/// * `source_width` - Width of source task bar (pixels)
/// * `target_x` - X position of target task bar (pixels)
/// * `target_y` - Y position of target task bar (pixels)
/// * `target_width` - Width of target task bar (pixels)
/// * `bar_height` - Height of task bars (pixels)
#[allow(clippy::too_many_arguments)]
pub fn calculate_dependency_path(
    dep_type: DependencyType,
    source_x: f64,
    source_y: f64,
    source_width: f64,
    target_x: f64,
    target_y: f64,
    target_width: f64,
    bar_height: f64,
) -> String {
    let source_center_y = source_y + bar_height / 2.0;
    let target_center_y = target_y + bar_height / 2.0;

    match dep_type {
        DependencyType::FS => {
            // Finish-to-Start: source end to target start
            let start_x = source_x + source_width;
            let start_y = source_center_y;
            let end_x = target_x;
            let end_y = target_center_y;

            create_curved_path(start_x, start_y, end_x, end_y)
        }
        DependencyType::SS => {
            // Start-to-Start: source start to target start
            let start_x = source_x;
            let start_y = source_center_y;
            let end_x = target_x;
            let end_y = target_center_y;

            create_curved_path(start_x, start_y, end_x, end_y)
        }
        DependencyType::FF => {
            // Finish-to-Finish: source end to target end
            let start_x = source_x + source_width;
            let start_y = source_center_y;
            let end_x = target_x + target_width;
            let end_y = target_center_y;

            create_curved_path(start_x, start_y, end_x, end_y)
        }
        DependencyType::SF => {
            // Start-to-Finish: source start to target end
            let start_x = source_x;
            let start_y = source_center_y;
            let end_x = target_x + target_width;
            let end_y = target_center_y;

            create_curved_path(start_x, start_y, end_x, end_y)
        }
    }
}

/// Create a curved bezier path between two points
fn create_curved_path(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> String {
    // Calculate control points for smooth curve
    let dx = end_x - start_x;

    // Use quadratic bezier curve with control point offset
    let control_offset = dx.abs() * 0.5;

    let control_x = start_x + control_offset;
    let control_y = start_y;

    format!(
        "M {} {} Q {} {} {} {}",
        start_x, start_y, control_x, control_y, end_x, end_y
    )
}

/// Component for rendering a dependency link
#[component]
pub fn DependencyLink(
    /// Source task ID
    #[prop(into)]
    source_task_id: String,

    /// Target task ID
    #[prop(into)]
    target_task_id: String,

    /// Dependency type
    #[prop(into)]
    dep_type: DependencyType,

    /// Source task X position (pixels)
    #[prop(into)]
    source_x: Signal<f64>,

    /// Source task Y position (pixels)
    #[prop(into)]
    source_y: Signal<f64>,

    /// Source task width (pixels)
    #[prop(into)]
    source_width: Signal<f64>,

    /// Target task X position (pixels)
    #[prop(into)]
    target_x: Signal<f64>,

    /// Target task Y position (pixels)
    #[prop(into)]
    target_y: Signal<f64>,

    /// Target task width (pixels)
    #[prop(into)]
    target_width: Signal<f64>,

    /// Task bar height (pixels)
    #[prop(optional, into, default=Signal::derive(|| 40.0))]
    bar_height: Signal<f64>,

    /// Whether this dependency link is selected
    #[prop(optional, into, default=Signal::derive(|| false))]
    is_selected: Signal<bool>,

    /// Callback when dependency is clicked
    #[prop(optional)]
    on_click: Option<Callback<(String, String)>>,

    /// Callback when dependency is right-clicked for context menu
    #[prop(optional)]
    on_context_menu: Option<Callback<(String, String)>>,
) -> impl IntoView {
    let path_d = Signal::derive(move || {
        calculate_dependency_path(
            dep_type,
            source_x.get(),
            source_y.get(),
            source_width.get(),
            target_x.get(),
            target_y.get(),
            target_width.get(),
            bar_height.get(),
        )
    });

    let source_id = source_task_id.clone();
    let target_id = target_task_id.clone();
    let source_id_ctx = source_task_id.clone();
    let target_id_ctx = target_task_id;

    view! {
        <g class="dependency-link">
            <path
                d=move || path_d.get()
                class="dependency-path"
                class:dependency-selected=move || is_selected.get()
                stroke="currentColor"
                stroke-width=move || if is_selected.get() { "3" } else { "2" }
                fill="none"
                marker-end="url(#arrowhead)"
                on:click=move |e| {
                    e.stop_propagation();
                    if let Some(ref cb) = on_click {
                        cb.run((source_id.clone(), target_id.clone()));
                    }
                }
                on:contextmenu=move |e| {
                    e.prevent_default();
                    e.stop_propagation();
                    if let Some(ref cb) = on_context_menu {
                        cb.run((source_id_ctx.clone(), target_id_ctx.clone()));
                    }
                }
                style="cursor: pointer; opacity: 0.7;"
                style:opacity=move || if is_selected.get() { "1.0" } else { "0.7" }
                style:stroke=move || if is_selected.get() { "var(--fallback-p,oklch(var(--p)/1))" } else { "currentColor" }
            />
        </g>
    }
}

/// Arrow marker definition for dependency links
/// Should be included once in the SVG defs section
#[component]
pub fn DependencyArrowMarker() -> impl IntoView {
    view! {
        <defs>
            <marker
                id="arrowhead"
                markerWidth="10"
                markerHeight="10"
                refX="9"
                refY="3"
                orient="auto"
                markerUnits="strokeWidth"
            >
                <path
                    d="M0,0 L0,6 L9,3 z"
                    fill="currentColor"
                />
            </marker>
        </defs>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fs_dependency() {
        let path = calculate_dependency_path(
            DependencyType::FS,
            0.0,   // source_x
            0.0,   // source_y
            100.0, // source_width
            150.0, // target_x
            50.0,  // target_y
            100.0, // target_width
            40.0,  // bar_height
        );

        // Should start at source end (100, 20) and end at target start (150, 70)
        assert!(path.contains("M 100"));
        assert!(path.contains("150 70"));
    }

    #[test]
    fn test_calculate_ss_dependency() {
        let path = calculate_dependency_path(
            DependencyType::SS,
            0.0,   // source_x
            0.0,   // source_y
            100.0, // source_width
            150.0, // target_x
            50.0,  // target_y
            100.0, // target_width
            40.0,  // bar_height
        );

        // Should start at source start (0, 20) and end at target start (150, 70)
        assert!(path.contains("M 0"));
        assert!(path.contains("150 70"));
    }

    #[test]
    fn test_calculate_ff_dependency() {
        let path = calculate_dependency_path(
            DependencyType::FF,
            0.0,   // source_x
            0.0,   // source_y
            100.0, // source_width
            150.0, // target_x
            50.0,  // target_y
            100.0, // target_width
            40.0,  // bar_height
        );

        // Should start at source end (100, 20) and end at target end (250, 70)
        assert!(path.contains("M 100"));
        assert!(path.contains("250 70"));
    }

    #[test]
    fn test_calculate_sf_dependency() {
        let path = calculate_dependency_path(
            DependencyType::SF,
            0.0,   // source_x
            0.0,   // source_y
            100.0, // source_width
            150.0, // target_x
            50.0,  // target_y
            100.0, // target_width
            40.0,  // bar_height
        );

        // Should start at source start (0, 20) and end at target end (250, 70)
        assert!(path.contains("M 0"));
        assert!(path.contains("250 70"));
    }

    #[test]
    fn test_curved_path_format() {
        let path = create_curved_path(0.0, 0.0, 100.0, 50.0);

        // Should be a quadratic bezier curve
        assert!(path.starts_with("M 0 0"));
        assert!(path.contains("Q"));
        assert!(path.contains("100 50"));
    }
}
