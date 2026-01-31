use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use leptos::prelude::*;

use crate::components::gantt::{utils::is_weekend, ViewMode};

/// Timeline grid component that renders the background grid with time scales
#[component]
pub fn TimelineGrid(
    /// Start date of the timeline
    #[prop(into)]
    start_date: Signal<DateTime<Utc>>,

    /// End date of the timeline
    #[prop(into)]
    end_date: Signal<DateTime<Utc>>,

    /// View mode for the timeline (Day/Week/Month/etc.)
    #[prop(into)]
    view_mode: Signal<ViewMode>,

    /// Width per time unit in pixels
    #[prop(into, default=Signal::derive(|| 60))]
    column_width: Signal<u32>,

    /// Height of the grid
    #[prop(into, default=Signal::derive(|| 600))]
    height: Signal<u32>,
) -> impl IntoView {
    let grid_lines = Signal::derive(move || {
        let start = start_date.get();
        let end = end_date.get();
        let mode = view_mode.get();
        let width = column_width.get();

        generate_grid_lines(start, end, mode, width)
    });

    let weekend_shading = Signal::derive(move || {
        let start = start_date.get();
        let end = end_date.get();
        let width = column_width.get();

        generate_weekend_shading(start, end, width)
    });

    let total_width = Signal::derive(move || {
        let lines = grid_lines.get();
        if let Some(last) = lines.last() {
            last.x + column_width.get()
        } else {
            800
        }
    });

    view! {
        <svg
            class="timeline-grid"
            width=move || total_width.get()
            height=move || height.get()
            xmlns="http://www.w3.org/2000/svg"
        >
            // Weekend shading rectangles
            <For
                each=move || weekend_shading.get()
                key=|shade| shade.x
                children=move |shade| {
                    view! {
                        <rect
                            x=shade.x
                            y=0
                            width=shade.width
                            height=move || height.get()
                            fill="currentColor"
                            opacity="0.05"
                            class="text-base-content"
                        />
                    }
                }
            />

            // Vertical grid lines
            <For
                each=move || grid_lines.get()
                key=|line| line.x
                children=move |line| {
                    let opacity = if line.is_major { 0.2 } else { 0.1 };
                    view! {
                        <line
                            x1=line.x
                            y1=0
                            x2=line.x
                            y2=move || height.get()
                            stroke="currentColor"
                            stroke-width=if line.is_major { 2 } else { 1 }
                            opacity=opacity
                            class="text-base-content"
                        />
                    }
                }
            />
        </svg>
    }
}

#[derive(Clone, Debug)]
struct GridLine {
    x: u32,
    is_major: bool,
}

#[derive(Clone, Debug)]
struct WeekendShade {
    x: u32,
    width: u32,
}

fn generate_grid_lines(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    mode: ViewMode,
    column_width: u32,
) -> Vec<GridLine> {
    let mut lines = Vec::new();
    let mut current = start;
    let mut x = 0;

    while current <= end {
        let is_major = match mode {
            ViewMode::Hour => current.hour() == 0,
            ViewMode::Day => current.day() == 1,
            ViewMode::Week => current.day() == 1,
            ViewMode::Month => current.month() == 1,
            ViewMode::Quarter => current.month() % 3 == 1 && current.day() == 1,
            ViewMode::Year => current.month() == 1 && current.day() == 1,
        };

        lines.push(GridLine { x, is_major });

        current = advance_by_view_mode(current, mode);
        x += column_width;
    }

    lines
}

fn generate_weekend_shading(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    column_width: u32,
) -> Vec<WeekendShade> {
    let mut shading = Vec::new();
    let mut current = start;
    let mut x = 0;

    while current <= end {
        if is_weekend(current.weekday()) {
            shading.push(WeekendShade {
                x,
                width: column_width,
            });
        }

        current = current + chrono::Duration::days(1);
        x += column_width;
    }

    shading
}

fn advance_by_view_mode(dt: DateTime<Utc>, mode: ViewMode) -> DateTime<Utc> {
    use chrono::Duration;

    match mode {
        ViewMode::Hour => dt + Duration::hours(1),
        ViewMode::Day => dt + Duration::days(1),
        ViewMode::Week => dt + Duration::weeks(1),
        ViewMode::Month => {
            let next_month = if dt.month() == 12 {
                chrono::Utc
                    .with_ymd_and_hms(dt.year() + 1, 1, 1, 0, 0, 0)
                    .unwrap()
            } else {
                chrono::Utc
                    .with_ymd_and_hms(dt.year(), dt.month() + 1, 1, 0, 0, 0)
                    .unwrap()
            };
            next_month
        }
        ViewMode::Quarter => dt + Duration::days(90),
        ViewMode::Year => chrono::Utc
            .with_ymd_and_hms(dt.year() + 1, 1, 1, 0, 0, 0)
            .unwrap(),
    }
}
