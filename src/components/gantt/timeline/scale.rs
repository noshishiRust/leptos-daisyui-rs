use chrono::{DateTime, Datelike, TimeZone, Utc};
use leptos::prelude::*;

use crate::components::gantt::ViewMode;

/// Timeline scale header component that displays time periods
#[component]
pub fn TimelineScale(
    /// Start date of the timeline
    #[prop(into)]
    start_date: Signal<DateTime<Utc>>,

    /// End date of the timeline
    #[prop(into)]
    end_date: Signal<DateTime<Utc>>,

    /// View mode for the timeline
    #[prop(into)]
    view_mode: Signal<ViewMode>,

    /// Width per time unit in pixels
    #[prop(into, default=Signal::derive(|| 60))]
    column_width: Signal<u32>,
) -> impl IntoView {
    let major_headers = Signal::derive(move || {
        let start = start_date.get();
        let end = end_date.get();
        let mode = view_mode.get();
        let width = column_width.get();

        generate_major_headers(start, end, mode, width)
    });

    let minor_headers = Signal::derive(move || {
        let start = start_date.get();
        let end = end_date.get();
        let mode = view_mode.get();
        let width = column_width.get();

        generate_minor_headers(start, end, mode, width)
    });

    view! {
        <div class="timeline-scale border-b border-base-300">
            // Major scale (e.g., Months for Day view)
            <div class="major-scale flex border-b border-base-300 bg-base-200">
                <For
                    each=move || major_headers.get()
                    key=|header| header.label.clone()
                    children=move |header| {
                        view! {
                            <div
                                class="scale-cell border-r border-base-300 px-2 py-1 text-center text-sm font-semibold"
                                style:width=format!("{}px", header.width)
                            >
                                {header.label}
                            </div>
                        }
                    }
                />
            </div>

            // Minor scale (e.g., Days for Day view)
            <div class="minor-scale flex bg-base-100">
                <For
                    each=move || minor_headers.get()
                    key=|header| header.label.clone()
                    children=move |header| {
                        view! {
                            <div
                                class="scale-cell border-r border-base-300 px-1 py-1 text-center text-xs"
                                style:width=format!("{}px", header.width)
                            >
                                {header.label}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[derive(Clone, Debug)]
struct ScaleHeader {
    label: String,
    width: u32,
}

fn generate_major_headers(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    mode: ViewMode,
    column_width: u32,
) -> Vec<ScaleHeader> {
    let mut headers = Vec::new();
    let mut current = start;

    match mode {
        ViewMode::Hour => {
            // Major: Days
            while current <= end {
                let day_label = current.format("%b %d").to_string();
                let hours_in_day = 24;
                headers.push(ScaleHeader {
                    label: day_label,
                    width: column_width * hours_in_day,
                });
                current += chrono::Duration::days(1);
            }
        }
        ViewMode::Day => {
            // Major: Months
            while current <= end {
                let month_label = current.format("%B %Y").to_string();
                let days_in_month = days_in_month(current.year(), current.month());
                headers.push(ScaleHeader {
                    label: month_label,
                    width: column_width * days_in_month,
                });
                current = next_month(current);
            }
        }
        ViewMode::Week => {
            // Major: Quarters
            while current <= end {
                let quarter = (current.month() - 1) / 3 + 1;
                let quarter_label = format!("Q{} {}", quarter, current.year());
                headers.push(ScaleHeader {
                    label: quarter_label,
                    width: column_width * 13, // ~13 weeks per quarter
                });
                current += chrono::Duration::days(90);
            }
        }
        ViewMode::Month => {
            // Major: Years
            while current <= end {
                let year_label = current.format("%Y").to_string();
                headers.push(ScaleHeader {
                    label: year_label,
                    width: column_width * 12,
                });
                current = chrono::Utc
                    .with_ymd_and_hms(current.year() + 1, 1, 1, 0, 0, 0)
                    .unwrap();
            }
        }
        ViewMode::Quarter => {
            // Major: Years
            while current <= end {
                let year_label = current.format("%Y").to_string();
                headers.push(ScaleHeader {
                    label: year_label,
                    width: column_width * 4,
                });
                current = chrono::Utc
                    .with_ymd_and_hms(current.year() + 1, 1, 1, 0, 0, 0)
                    .unwrap();
            }
        }
        ViewMode::Year => {
            // Major: Decades
            let decade_start = (start.year() / 10) * 10;
            let mut year = decade_start;
            while year <= end.year() {
                headers.push(ScaleHeader {
                    label: format!("{}-{}", year, year + 9),
                    width: column_width * 10,
                });
                year += 10;
            }
        }
    }

    headers
}

fn generate_minor_headers(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    mode: ViewMode,
    column_width: u32,
) -> Vec<ScaleHeader> {
    let mut headers = Vec::new();
    let mut current = start;

    match mode {
        ViewMode::Hour => {
            while current <= end {
                headers.push(ScaleHeader {
                    label: current.format("%H:%M").to_string(),
                    width: column_width,
                });
                current += chrono::Duration::hours(1);
            }
        }
        ViewMode::Day => {
            while current <= end {
                headers.push(ScaleHeader {
                    label: current.format("%d").to_string(),
                    width: column_width,
                });
                current += chrono::Duration::days(1);
            }
        }
        ViewMode::Week => {
            while current <= end {
                let week_num = current.iso_week().week();
                headers.push(ScaleHeader {
                    label: format!("W{}", week_num),
                    width: column_width,
                });
                current += chrono::Duration::weeks(1);
            }
        }
        ViewMode::Month => {
            while current <= end {
                headers.push(ScaleHeader {
                    label: current.format("%b").to_string(),
                    width: column_width,
                });
                current = next_month(current);
            }
        }
        ViewMode::Quarter => {
            while current <= end {
                let quarter = (current.month() - 1) / 3 + 1;
                headers.push(ScaleHeader {
                    label: format!("Q{}", quarter),
                    width: column_width,
                });
                current += chrono::Duration::days(90);
            }
        }
        ViewMode::Year => {
            while current <= end {
                headers.push(ScaleHeader {
                    label: current.format("%Y").to_string(),
                    width: column_width,
                });
                current = chrono::Utc
                    .with_ymd_and_hms(current.year() + 1, 1, 1, 0, 0, 0)
                    .unwrap();
            }
        }
    }

    headers
}

fn next_month(dt: DateTime<Utc>) -> DateTime<Utc> {
    if dt.month() == 12 {
        chrono::Utc
            .with_ymd_and_hms(dt.year() + 1, 1, 1, 0, 0, 0)
            .unwrap()
    } else {
        chrono::Utc
            .with_ymd_and_hms(dt.year(), dt.month() + 1, 1, 0, 0, 0)
            .unwrap()
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let next_month = if month == 12 {
        chrono::Utc
            .with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0)
            .unwrap()
    } else {
        chrono::Utc
            .with_ymd_and_hms(year, month + 1, 1, 0, 0, 0)
            .unwrap()
    };

    let this_month = chrono::Utc
        .with_ymd_and_hms(year, month, 1, 0, 0, 0)
        .unwrap();

    (next_month - this_month).num_days() as u32
}
