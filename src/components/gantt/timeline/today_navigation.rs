/// Today navigation utilities for Gantt chart timeline
///
/// Provides navigation to current date with visual highlighting and
/// keyboard shortcuts for quick temporal orientation.
use chrono::{DateTime, Datelike, Utc};
use leptos::prelude::*;

/// Configuration for today navigation behavior
#[derive(Clone, Debug, PartialEq)]
pub struct TodayNavigationConfig {
    /// Whether to enable today navigation button
    pub show_today_button: bool,

    /// Whether to enable keyboard shortcut (T key)
    pub enable_keyboard_shortcut: bool,

    /// Whether to highlight today column
    pub highlight_today_column: bool,

    /// Duration of highlight pulse animation (in milliseconds)
    pub highlight_pulse_duration_ms: u32,

    /// Whether to auto-scroll to center today in viewport
    pub center_in_viewport: bool,

    /// Whether to disable button when today is already visible
    pub disable_when_visible: bool,
}

impl Default for TodayNavigationConfig {
    fn default() -> Self {
        Self {
            show_today_button: true,
            enable_keyboard_shortcut: true,
            highlight_today_column: true,
            highlight_pulse_duration_ms: 1500,
            center_in_viewport: true,
            disable_when_visible: true,
        }
    }
}

/// Calculate if today is within the visible timeline range
///
/// # Arguments
/// * `timeline_start` - Start date of visible timeline
/// * `timeline_end` - End date of visible timeline
/// * `today` - Today's date (defaults to Utc::now())
pub fn is_today_visible(
    timeline_start: DateTime<Utc>,
    timeline_end: DateTime<Utc>,
    today: Option<DateTime<Utc>>,
) -> bool {
    let today = today.unwrap_or_else(Utc::now);
    today >= timeline_start && today <= timeline_end
}

/// Calculate the pixel offset for today's date in the timeline
///
/// # Arguments
/// * `timeline_start` - Start date of visible timeline
/// * `column_width` - Width of one time unit column in pixels
/// * `today` - Today's date (defaults to Utc::now())
/// * `time_unit_days` - Number of days per time unit (1 for day view, 7 for week, etc.)
pub fn calculate_today_offset(
    timeline_start: DateTime<Utc>,
    column_width: f64,
    today: Option<DateTime<Utc>>,
    time_unit_days: i64,
) -> f64 {
    let today = today.unwrap_or_else(Utc::now);
    let days_from_start = today
        .signed_duration_since(timeline_start)
        .num_days();

    let time_units_from_start = days_from_start as f64 / time_unit_days as f64;
    time_units_from_start * column_width
}

/// Calculate scroll position to center today in the viewport
///
/// # Arguments
/// * `today_offset` - Pixel offset of today from timeline start
/// * `viewport_width` - Width of the visible viewport in pixels
pub fn calculate_scroll_to_center_today(today_offset: f64, viewport_width: f64) -> f64 {
    (today_offset - (viewport_width / 2.0)).max(0.0)
}

/// Get CSS classes for today column highlighting
///
/// # Arguments
/// * `is_today` - Whether this column represents today
/// * `is_pulsing` - Whether the pulse animation is active
pub fn get_today_column_classes(is_today: bool, is_pulsing: bool) -> Vec<&'static str> {
    let mut classes = Vec::new();

    if is_today {
        classes.push("gantt-today-column");

        if is_pulsing {
            classes.push("gantt-today-pulse");
        }
    }

    classes
}

/// Today navigation button component
#[component]
pub fn TodayButton(
    /// Whether today is currently visible in the timeline
    #[prop(into)]
    is_today_visible: Signal<bool>,

    /// Callback when today button is clicked
    #[prop(optional)]
    on_navigate_to_today: Option<Callback<()>>,

    /// Configuration for today navigation
    #[prop(optional, into, default=Signal::derive(|| TodayNavigationConfig::default()))]
    config: Signal<TodayNavigationConfig>,

    /// Custom CSS class for the button
    #[prop(optional, into, default=String::new())]
    class: String,
) -> impl IntoView {
    let is_disabled = Signal::derive(move || {
        let cfg = config.get();
        cfg.disable_when_visible && is_today_visible.get()
    });

    let handle_click = move |_| {
        if !is_disabled.get()
            && let Some(ref cb) = on_navigate_to_today {
                cb.run(());
            }
    };

    view! {
        <Show when=move || config.get().show_today_button>
            <button
                class=format!("btn btn-sm btn-ghost {}", class)
                class:btn-disabled=move || is_disabled.get()
                disabled=move || is_disabled.get()
                on:click=handle_click
                aria-label="Navigate to today"
                title="Navigate to today (T)"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                    />
                </svg>
                <span class="hidden sm:inline ml-1">"Today"</span>
            </button>
        </Show>
    }
}

/// Today column marker component for timeline
#[component]
pub fn TodayMarker(
    /// X position of today in the timeline (pixels)
    #[prop(into)]
    today_x: Signal<f64>,

    /// Height of the timeline area
    #[prop(into)]
    timeline_height: Signal<f64>,

    /// Whether to show the pulse animation
    #[prop(into, default=Signal::derive(|| false))]
    is_pulsing: Signal<bool>,

    /// Configuration for today navigation
    #[prop(optional, into, default=Signal::derive(|| TodayNavigationConfig::default()))]
    config: Signal<TodayNavigationConfig>,
) -> impl IntoView {
    view! {
        <Show when=move || config.get().highlight_today_column>
            <g class="gantt-today-marker">
                // Vertical line for today
                <line
                    x1=move || today_x.get()
                    y1="0"
                    x2=move || today_x.get()
                    y2=move || timeline_height.get()
                    class="gantt-today-line"
                    class:gantt-today-pulse=move || is_pulsing.get()
                    stroke="var(--fallback-p,oklch(var(--p)/0.5))"
                    stroke-width="2"
                    stroke-dasharray="5,5"
                    style="opacity: 0.6;"
                />

                // Optional: Background highlight column
                <rect
                    x=move || today_x.get() - 15.0
                    y="0"
                    width="30"
                    height=move || timeline_height.get()
                    class="gantt-today-background"
                    class:gantt-today-pulse=move || is_pulsing.get()
                    fill="var(--fallback-p,oklch(var(--p)/0.1))"
                    style="opacity: 0.3; pointer-events: none;"
                />
            </g>
        </Show>
    }
}

/// Check if a given date is today
pub fn is_date_today(date: DateTime<Utc>) -> bool {
    let today = Utc::now();
    date.year() == today.year()
        && date.month() == today.month()
        && date.day() == today.day()
}

/// Get the start of today (midnight UTC)
pub fn get_today_start() -> DateTime<Utc> {
    let now = Utc::now();
    now.date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
}

/// Get the end of today (23:59:59 UTC)
pub fn get_today_end() -> DateTime<Utc> {
    let now = Utc::now();
    now.date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_utc()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike};

    #[test]
    fn test_is_today_visible_within_range() {
        let today = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        let start = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 6, 30, 0, 0, 0).unwrap();

        assert!(is_today_visible(start, end, Some(today)));
    }

    #[test]
    fn test_is_today_visible_before_range() {
        let today = Utc.with_ymd_and_hms(2024, 5, 31, 12, 0, 0).unwrap();
        let start = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 6, 30, 0, 0, 0).unwrap();

        assert!(!is_today_visible(start, end, Some(today)));
    }

    #[test]
    fn test_is_today_visible_after_range() {
        let today = Utc.with_ymd_and_hms(2024, 7, 1, 12, 0, 0).unwrap();
        let start = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 6, 30, 0, 0, 0).unwrap();

        assert!(!is_today_visible(start, end, Some(today)));
    }

    #[test]
    fn test_calculate_today_offset_day_view() {
        let start = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
        let today = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        let column_width = 60.0;
        let time_unit_days = 1;

        let offset = calculate_today_offset(start, column_width, Some(today), time_unit_days);

        // 14 days from June 1 to June 15
        assert_eq!(offset, 14.0 * 60.0);
    }

    #[test]
    fn test_calculate_today_offset_week_view() {
        let start = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
        let today = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        let column_width = 120.0;
        let time_unit_days = 7;

        let offset = calculate_today_offset(start, column_width, Some(today), time_unit_days);

        // 14 days = 2 weeks
        assert_eq!(offset, 2.0 * 120.0);
    }

    #[test]
    fn test_calculate_scroll_to_center_today() {
        let today_offset = 1000.0;
        let viewport_width = 800.0;

        let scroll_pos = calculate_scroll_to_center_today(today_offset, viewport_width);

        // Should scroll to center today: 1000 - 400 = 600
        assert_eq!(scroll_pos, 600.0);
    }

    #[test]
    fn test_calculate_scroll_to_center_today_at_start() {
        let today_offset = 100.0;
        let viewport_width = 800.0;

        let scroll_pos = calculate_scroll_to_center_today(today_offset, viewport_width);

        // Should not scroll past 0
        assert_eq!(scroll_pos, 0.0);
    }

    #[test]
    fn test_get_today_column_classes_normal() {
        let classes = get_today_column_classes(true, false);

        assert_eq!(classes.len(), 1);
        assert!(classes.contains(&"gantt-today-column"));
    }

    #[test]
    fn test_get_today_column_classes_pulsing() {
        let classes = get_today_column_classes(true, true);

        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&"gantt-today-column"));
        assert!(classes.contains(&"gantt-today-pulse"));
    }

    #[test]
    fn test_get_today_column_classes_not_today() {
        let classes = get_today_column_classes(false, false);

        assert_eq!(classes.len(), 0);
    }

    #[test]
    fn test_is_date_today() {
        let now = Utc::now();
        assert!(is_date_today(now));

        let yesterday = now - chrono::Duration::days(1);
        assert!(!is_date_today(yesterday));

        let tomorrow = now + chrono::Duration::days(1);
        assert!(!is_date_today(tomorrow));
    }

    #[test]
    fn test_get_today_start() {
        let today_start = get_today_start();

        assert_eq!(today_start.hour(), 0);
        assert_eq!(today_start.minute(), 0);
        assert_eq!(today_start.second(), 0);
    }

    #[test]
    fn test_get_today_end() {
        let today_end = get_today_end();

        assert_eq!(today_end.hour(), 23);
        assert_eq!(today_end.minute(), 59);
        assert_eq!(today_end.second(), 59);
    }

    #[test]
    fn test_today_navigation_config_default() {
        let config = TodayNavigationConfig::default();

        assert!(config.show_today_button);
        assert!(config.enable_keyboard_shortcut);
        assert!(config.highlight_today_column);
        assert_eq!(config.highlight_pulse_duration_ms, 1500);
        assert!(config.center_in_viewport);
        assert!(config.disable_when_visible);
    }
}
