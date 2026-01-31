/// Timeline markers for annotating important dates in Gantt chart
///
/// Provides vertical markers to highlight key dates like releases, sprints,
/// holidays, milestones, and custom events with labels, icons, and styling.
use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::MouseEvent;

/// Type of timeline marker
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkerType {
    /// Today marker (auto-updating)
    Today,
    /// Product release milestone
    Release,
    /// Sprint/iteration boundary
    Sprint,
    /// Holiday or company event
    Holiday,
    /// Deadline
    Deadline,
    /// Custom marker type
    Custom(String),
}

impl MarkerType {
    /// Get the default color for this marker type
    pub fn default_color(&self) -> &'static str {
        match self {
            MarkerType::Today => "#3b82f6",     // Blue
            MarkerType::Release => "#10b981",   // Green
            MarkerType::Sprint => "#8b5cf6",    // Purple
            MarkerType::Holiday => "#f59e0b",   // Amber
            MarkerType::Deadline => "#ef4444",  // Red
            MarkerType::Custom(_) => "#6b7280", // Gray
        }
    }

    /// Get the default icon for this marker type
    pub fn default_icon(&self) -> MarkerIcon {
        match self {
            MarkerType::Today => MarkerIcon::Circle,
            MarkerType::Release => MarkerIcon::Flag,
            MarkerType::Sprint => MarkerIcon::Star,
            MarkerType::Holiday => MarkerIcon::Calendar,
            MarkerType::Deadline => MarkerIcon::Alert,
            MarkerType::Custom(_) => MarkerIcon::Diamond,
        }
    }
}

/// Visual style of the marker line
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkerStyle {
    /// Solid line
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
    /// Custom SVG stroke-dasharray
    Custom(String),
}

impl MarkerStyle {
    /// Get SVG stroke-dasharray attribute value
    pub fn to_dasharray(&self) -> Option<String> {
        match self {
            MarkerStyle::Solid => None,
            MarkerStyle::Dashed => Some("10,5".to_string()),
            MarkerStyle::Dotted => Some("2,3".to_string()),
            MarkerStyle::Custom(pattern) => Some(pattern.clone()),
        }
    }
}

/// Icon shape for marker
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkerIcon {
    /// No icon
    None,
    /// Circle
    Circle,
    /// Flag
    Flag,
    /// Star
    Star,
    /// Diamond
    Diamond,
    /// Calendar
    Calendar,
    /// Alert/warning triangle
    Alert,
    /// Custom SVG path
    Custom(String),
}

/// Label positioning for marker
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LabelPosition {
    /// Above the timeline
    Top,
    /// Below the timeline
    Bottom,
    /// Rotated 90 degrees along the marker line
    Vertical,
    /// No label
    None,
}

/// Timeline marker definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineMarker {
    /// Unique identifier
    pub id: String,

    /// Date/time of the marker
    pub date: DateTime<Utc>,

    /// Display label
    pub label: String,

    /// Marker type
    pub marker_type: MarkerType,

    /// Line color (hex)
    pub color: Option<String>,

    /// Line style
    pub style: MarkerStyle,

    /// Icon to display
    pub icon: MarkerIcon,

    /// Label position
    pub label_position: LabelPosition,

    /// Z-index for layering (higher = on top)
    pub z_index: i32,

    /// Whether the marker is visible
    pub visible: bool,

    /// Optional tooltip text
    pub tooltip: Option<String>,

    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl TimelineMarker {
    /// Create a new timeline marker
    pub fn new(
        id: impl Into<String>,
        date: DateTime<Utc>,
        label: impl Into<String>,
        marker_type: MarkerType,
    ) -> Self {
        let color = Some(marker_type.default_color().to_string());
        let icon = marker_type.default_icon();

        Self {
            id: id.into(),
            date,
            label: label.into(),
            marker_type,
            color,
            style: MarkerStyle::Solid,
            icon,
            label_position: LabelPosition::Top,
            z_index: 0,
            visible: true,
            tooltip: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a today marker (auto-updates to current date)
    pub fn today(id: impl Into<String>) -> Self {
        Self::new(id, Utc::now(), "Today", MarkerType::Today)
    }

    /// Create a release marker
    pub fn release(id: impl Into<String>, date: DateTime<Utc>, version: impl Into<String>) -> Self {
        Self::new(
            id,
            date,
            format!("Release {}", version.into()),
            MarkerType::Release,
        )
    }

    /// Create a sprint marker
    pub fn sprint(
        id: impl Into<String>,
        date: DateTime<Utc>,
        sprint_name: impl Into<String>,
    ) -> Self {
        Self::new(id, date, sprint_name, MarkerType::Sprint)
    }

    /// Create a holiday marker
    pub fn holiday(
        id: impl Into<String>,
        date: DateTime<Utc>,
        holiday_name: impl Into<String>,
    ) -> Self {
        Self::new(id, date, holiday_name, MarkerType::Holiday)
    }

    /// Create a deadline marker
    pub fn deadline(id: impl Into<String>, date: DateTime<Utc>, name: impl Into<String>) -> Self {
        Self::new(id, date, name, MarkerType::Deadline)
    }

    /// Builder: Set custom color
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Builder: Set line style
    pub fn with_style(mut self, style: MarkerStyle) -> Self {
        self.style = style;
        self
    }

    /// Builder: Set icon
    pub fn with_icon(mut self, icon: MarkerIcon) -> Self {
        self.icon = icon;
        self
    }

    /// Builder: Set label position
    pub fn with_label_position(mut self, position: LabelPosition) -> Self {
        self.label_position = position;
        self
    }

    /// Builder: Set z-index
    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    /// Builder: Set tooltip
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Builder: Set visibility
    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

/// Collection of markers with collision detection
#[derive(Clone, Debug, Default)]
pub struct MarkerCollection {
    markers: Vec<TimelineMarker>,
}

impl MarkerCollection {
    /// Create a new marker collection
    pub fn new() -> Self {
        Self {
            markers: Vec::new(),
        }
    }

    /// Add a marker to the collection
    pub fn add(&mut self, marker: TimelineMarker) {
        self.markers.push(marker);
        self.sort_markers();
    }

    /// Remove a marker by ID
    pub fn remove(&mut self, id: &str) -> Option<TimelineMarker> {
        if let Some(pos) = self.markers.iter().position(|m| m.id == id) {
            Some(self.markers.remove(pos))
        } else {
            None
        }
    }

    /// Get a marker by ID
    pub fn get(&self, id: &str) -> Option<&TimelineMarker> {
        self.markers.iter().find(|m| m.id == id)
    }

    /// Get a mutable marker by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut TimelineMarker> {
        self.markers.iter_mut().find(|m| m.id == id)
    }

    /// Get all visible markers
    pub fn visible_markers(&self) -> Vec<&TimelineMarker> {
        self.markers.iter().filter(|m| m.visible).collect()
    }

    /// Get markers in a date range
    pub fn markers_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&TimelineMarker> {
        self.markers
            .iter()
            .filter(|m| m.visible && m.date >= start && m.date <= end)
            .collect()
    }

    /// Get markers by type
    pub fn markers_by_type(&self, marker_type: &MarkerType) -> Vec<&TimelineMarker> {
        self.markers
            .iter()
            .filter(|m| &m.marker_type == marker_type)
            .collect()
    }

    /// Detect markers that are too close together (collision detection)
    ///
    /// Returns groups of marker IDs that are within `threshold_pixels` of each other
    pub fn detect_collisions(
        &self,
        threshold_pixels: f64,
        pixels_per_day: f64,
    ) -> Vec<Vec<String>> {
        let threshold_days = threshold_pixels / pixels_per_day;
        let mut groups = Vec::new();
        let mut sorted_markers: Vec<_> = self.visible_markers();
        sorted_markers.sort_by_key(|m| m.date);

        let mut current_group = Vec::new();
        let mut last_date: Option<DateTime<Utc>> = None;

        for marker in sorted_markers {
            if let Some(last) = last_date {
                let days_diff = marker.date.signed_duration_since(last).num_days().abs() as f64;

                if days_diff <= threshold_days {
                    // Add to current group
                    if current_group.is_empty() {
                        // Find the previous marker and add it
                        if let Some(prev) = self.markers.iter().rev().find(|m| m.date == last) {
                            current_group.push(prev.id.clone());
                        }
                    }
                    current_group.push(marker.id.clone());
                } else {
                    // Start new group
                    if current_group.len() > 1 {
                        groups.push(current_group.clone());
                    }
                    current_group.clear();
                }
            }

            last_date = Some(marker.date);
        }

        if current_group.len() > 1 {
            groups.push(current_group);
        }

        groups
    }

    /// Sort markers by date and z-index
    fn sort_markers(&mut self) {
        self.markers
            .sort_by(|a, b| a.date.cmp(&b.date).then(a.z_index.cmp(&b.z_index)));
    }

    /// Get all markers (sorted)
    pub fn all(&self) -> &[TimelineMarker] {
        &self.markers
    }

    /// Get count of markers
    pub fn len(&self) -> usize {
        self.markers.len()
    }

    /// Check if collection is empty
    pub fn is_empty(&self) -> bool {
        self.markers.is_empty()
    }
}

/// Calculate X position for a marker in the timeline
pub fn calculate_marker_position(
    marker_date: DateTime<Utc>,
    timeline_start: DateTime<Utc>,
    column_width: f64,
    time_unit_days: i64,
) -> f64 {
    let days_from_start = marker_date.signed_duration_since(timeline_start).num_days();

    let time_units_from_start = days_from_start as f64 / time_unit_days as f64;
    time_units_from_start * column_width
}

/// Timeline marker component for rendering
#[component]
pub fn TimelineMarkerComponent(
    /// The marker to render
    #[prop(into)]
    marker: Signal<TimelineMarker>,

    /// X position in timeline (pixels)
    #[prop(into)]
    x_position: Signal<f64>,

    /// Height of the timeline area
    #[prop(into)]
    timeline_height: Signal<f64>,

    /// Callback when marker is clicked
    #[prop(optional)]
    on_click: Option<Callback<String>>,

    /// Callback when marker is right-clicked
    #[prop(optional)]
    on_context_menu: Option<Callback<String>>,
) -> impl IntoView {
    let handle_click = move |e: MouseEvent| {
        e.stop_propagation();
        if let Some(ref cb) = on_click {
            cb.run(marker.get().id.clone());
        }
    };

    let handle_context_menu = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        if let Some(ref cb) = on_context_menu {
            cb.run(marker.get().id.clone());
        }
    };

    view! {
        <Show when=move || marker.get().visible>
            <g
                class="timeline-marker"
                style=move || format!("z-index: {}", marker.get().z_index)
                on:click=handle_click
                on:contextmenu=handle_context_menu
            >
                // Vertical line
                <line
                    x1=move || x_position.get()
                    y1="0"
                    x2=move || x_position.get()
                    y2=move || timeline_height.get()
                    stroke=move || marker.get().color.clone().unwrap_or_else(|| "#6b7280".to_string())
                    stroke-width="2"
                    stroke-dasharray=move || marker.get().style.to_dasharray()
                    class="marker-line"
                    style="cursor: pointer; opacity: 0.7;"
                />

                // Label (if positioned at top)
                <Show when=move || matches!(marker.get().label_position, LabelPosition::Top)>
                    <text
                        x=move || x_position.get()
                        y="10"
                        class="marker-label"
                        text-anchor="middle"
                        fill=move || marker.get().color.clone().unwrap_or_else(|| "#6b7280".to_string())
                        font-size="12"
                        font-weight="600"
                        style="cursor: pointer; user-select: none;"
                    >
                        {move || marker.get().label.clone()}
                    </text>
                </Show>

                // Tooltip on hover (title attribute)
                {move || {
                    if let Some(tooltip) = marker.get().tooltip {
                        view! { <title>{tooltip}</title> }.into_any()
                    } else {
                        view! { <title>{marker.get().label.clone()}</title> }.into_any()
                    }
                }}
            </g>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_marker_type_default_color() {
        assert_eq!(MarkerType::Today.default_color(), "#3b82f6");
        assert_eq!(MarkerType::Release.default_color(), "#10b981");
        assert_eq!(MarkerType::Sprint.default_color(), "#8b5cf6");
        assert_eq!(MarkerType::Holiday.default_color(), "#f59e0b");
        assert_eq!(MarkerType::Deadline.default_color(), "#ef4444");
    }

    #[test]
    fn test_marker_style_dasharray() {
        assert_eq!(MarkerStyle::Solid.to_dasharray(), None);
        assert_eq!(MarkerStyle::Dashed.to_dasharray(), Some("10,5".to_string()));
        assert_eq!(MarkerStyle::Dotted.to_dasharray(), Some("2,3".to_string()));
        assert_eq!(
            MarkerStyle::Custom("5,10,5".to_string()).to_dasharray(),
            Some("5,10,5".to_string())
        );
    }

    #[test]
    fn test_timeline_marker_creation() {
        let date = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();
        let marker = TimelineMarker::new("marker1", date, "Test Marker", MarkerType::Release);

        assert_eq!(marker.id, "marker1");
        assert_eq!(marker.date, date);
        assert_eq!(marker.label, "Test Marker");
        assert!(matches!(marker.marker_type, MarkerType::Release));
        assert!(marker.visible);
        assert_eq!(marker.z_index, 0);
    }

    #[test]
    fn test_timeline_marker_today() {
        let marker = TimelineMarker::today("today-marker");

        assert_eq!(marker.id, "today-marker");
        assert_eq!(marker.label, "Today");
        assert!(matches!(marker.marker_type, MarkerType::Today));
    }

    #[test]
    fn test_timeline_marker_builder() {
        let date = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();
        let marker =
            TimelineMarker::new("marker1", date, "Test", MarkerType::Custom("test".into()))
                .with_color("#ff0000")
                .with_style(MarkerStyle::Dashed)
                .with_z_index(5)
                .with_tooltip("Test tooltip")
                .with_visibility(false);

        assert_eq!(marker.color, Some("#ff0000".to_string()));
        assert!(matches!(marker.style, MarkerStyle::Dashed));
        assert_eq!(marker.z_index, 5);
        assert_eq!(marker.tooltip, Some("Test tooltip".to_string()));
        assert!(!marker.visible);
    }

    #[test]
    fn test_marker_collection_add_remove() {
        let mut collection = MarkerCollection::new();
        let date = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();
        let marker = TimelineMarker::new("marker1", date, "Test", MarkerType::Release);

        collection.add(marker);
        assert_eq!(collection.len(), 1);

        let removed = collection.remove("marker1");
        assert!(removed.is_some());
        assert_eq!(collection.len(), 0);

        let removed_again = collection.remove("marker1");
        assert!(removed_again.is_none());
    }

    #[test]
    fn test_marker_collection_visible_markers() {
        let mut collection = MarkerCollection::new();
        let date = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();

        collection.add(TimelineMarker::new(
            "marker1",
            date,
            "Visible",
            MarkerType::Release,
        ));
        collection.add(
            TimelineMarker::new("marker2", date, "Hidden", MarkerType::Sprint)
                .with_visibility(false),
        );

        let visible = collection.visible_markers();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].id, "marker1");
    }

    #[test]
    fn test_marker_collection_range() {
        let mut collection = MarkerCollection::new();
        let date1 = Utc.with_ymd_and_hms(2024, 6, 10, 0, 0, 0).unwrap();
        let date2 = Utc.with_ymd_and_hms(2024, 6, 20, 0, 0, 0).unwrap();
        let date3 = Utc.with_ymd_and_hms(2024, 6, 30, 0, 0, 0).unwrap();

        collection.add(TimelineMarker::new(
            "marker1",
            date1,
            "M1",
            MarkerType::Release,
        ));
        collection.add(TimelineMarker::new(
            "marker2",
            date2,
            "M2",
            MarkerType::Sprint,
        ));
        collection.add(TimelineMarker::new(
            "marker3",
            date3,
            "M3",
            MarkerType::Holiday,
        ));

        let start = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 6, 25, 0, 0, 0).unwrap();

        let in_range = collection.markers_in_range(start, end);
        assert_eq!(in_range.len(), 1);
        assert_eq!(in_range[0].id, "marker2");
    }

    #[test]
    fn test_marker_collection_by_type() {
        let mut collection = MarkerCollection::new();
        let date = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();

        collection.add(TimelineMarker::new(
            "marker1",
            date,
            "R1",
            MarkerType::Release,
        ));
        collection.add(TimelineMarker::new(
            "marker2",
            date,
            "R2",
            MarkerType::Release,
        ));
        collection.add(TimelineMarker::new(
            "marker3",
            date,
            "S1",
            MarkerType::Sprint,
        ));

        let releases = collection.markers_by_type(&MarkerType::Release);
        assert_eq!(releases.len(), 2);
    }

    #[test]
    fn test_collision_detection() {
        let mut collection = MarkerCollection::new();
        let date1 = Utc.with_ymd_and_hms(2024, 6, 10, 0, 0, 0).unwrap();
        let date2 = Utc.with_ymd_and_hms(2024, 6, 11, 0, 0, 0).unwrap(); // 1 day apart
        let date3 = Utc.with_ymd_and_hms(2024, 6, 20, 0, 0, 0).unwrap(); // Far apart

        collection.add(TimelineMarker::new(
            "marker1",
            date1,
            "M1",
            MarkerType::Release,
        ));
        collection.add(TimelineMarker::new(
            "marker2",
            date2,
            "M2",
            MarkerType::Sprint,
        ));
        collection.add(TimelineMarker::new(
            "marker3",
            date3,
            "M3",
            MarkerType::Holiday,
        ));

        // With 60 pixels per day and 30 pixel threshold, markers within 0.5 days collide
        let collisions = collection.detect_collisions(30.0, 60.0);

        // marker1 and marker2 are 1 day apart, should not collide with 30px threshold
        assert_eq!(collisions.len(), 0);

        // With 100 pixel threshold, markers within 1.67 days collide
        let collisions = collection.detect_collisions(100.0, 60.0);
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0].len(), 2);
    }

    #[test]
    fn test_calculate_marker_position() {
        let timeline_start = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
        let marker_date = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();
        let column_width = 60.0;
        let time_unit_days = 1;

        let position =
            calculate_marker_position(marker_date, timeline_start, column_width, time_unit_days);

        // 14 days from start
        assert_eq!(position, 14.0 * 60.0);
    }
}
