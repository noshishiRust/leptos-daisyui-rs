/// Accessibility utilities for Gantt chart
///
/// Provides screen reader announcements, ARIA helpers, and accessibility configuration.
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// Accessible announcement for screen readers
#[derive(Clone, Debug)]
pub struct AccessibleAnnouncement {
    /// The message to announce
    pub message: String,
    /// Whether this is a polite or assertive announcement
    pub assertive: bool,
}

impl AccessibleAnnouncement {
    /// Create a new polite announcement
    pub fn polite(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            assertive: false,
        }
    }

    /// Create a new assertive announcement (interrupts screen reader)
    pub fn assertive(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            assertive: true,
        }
    }
}

/// Screen reader live region component
#[component]
pub fn LiveRegion(
    /// Current announcement to read
    #[prop(into)]
    announcement: Signal<Option<AccessibleAnnouncement>>,
) -> impl IntoView {
    view! {
        <div class="sr-only" aria-live=move || {
            announcement.get().map(|a| {
                if a.assertive { "assertive" } else { "polite" }
            }).unwrap_or("polite")
        } aria-atomic="true">
            {move || announcement.get().map(|a| a.message).unwrap_or_default()}
        </div>
    }
}

/// Check if user prefers reduced motion
/// This will be checked via CSS media query in the component styles
pub fn prefers_reduced_motion() -> bool {
    // Will be implemented via CSS @media (prefers-reduced-motion: reduce)
    // For now, return false - components will handle this via CSS
    false
}

/// Check if high contrast mode is active
/// This will be checked via CSS media query in the component styles
pub fn is_high_contrast_mode() -> bool {
    // Will be implemented via CSS @media (forced-colors: active) and (prefers-contrast: more)
    // For now, return false - components will handle this via CSS
    false
}

/// Generate ARIA label for a task
pub fn task_aria_label(
    name: &str,
    start: &str,
    end: &str,
    progress: f32,
    is_selected: bool,
    is_readonly: bool,
) -> String {
    let selected_text = if is_selected { ", selected" } else { "" };
    let readonly_text = if is_readonly { ", read-only" } else { "" };
    format!(
        "{}, from {} to {}, {:.0}% complete{}{}",
        name,
        start,
        end,
        progress * 100.0,
        selected_text,
        readonly_text
    )
}

/// Generate ARIA label for zoom controls
pub fn zoom_aria_label(direction: &str, can_zoom: bool, current_mode: &str) -> String {
    if can_zoom {
        format!("Zoom {}, current view: {}", direction, current_mode)
    } else {
        format!("Cannot zoom {} from {} view", direction, current_mode)
    }
}

/// Focus element by ID
pub fn focus_element_by_id(id: &str) {
    if let Some(document) = web_sys::window().and_then(|w| w.document())
        && let Some(element) = document.get_element_by_id(id)
        && let Some(html_element) = element.dyn_ref::<HtmlElement>()
    {
        let _ = html_element.focus();
    }
}

/// Announce a message to screen readers
pub fn announce(message: &str, assertive: bool) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        // Find or create live region
        let live_region = if let Some(region) = document.get_element_by_id("gantt-live-region") {
            region
        } else {
            let region = document.create_element("div").unwrap();
            region.set_id("gantt-live-region");
            region.set_attribute("class", "sr-only").unwrap();
            region.set_attribute("aria-atomic", "true").unwrap();
            if let Some(body) = document.body() {
                let _ = body.append_child(&region);
            }
            region
        };

        // Set aria-live attribute
        let aria_live = if assertive { "assertive" } else { "polite" };
        let _ = live_region.set_attribute("aria-live", aria_live);

        // Set message
        live_region.set_text_content(Some(message));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessible_announcement_polite() {
        let announcement = AccessibleAnnouncement::polite("Test message");
        assert_eq!(announcement.message, "Test message");
        assert!(!announcement.assertive);
    }

    #[test]
    fn test_accessible_announcement_assertive() {
        let announcement = AccessibleAnnouncement::assertive("Urgent message");
        assert_eq!(announcement.message, "Urgent message");
        assert!(announcement.assertive);
    }

    #[test]
    fn test_task_aria_label_basic() {
        let label = task_aria_label("Task 1", "2024-01-01", "2024-01-05", 0.5, false, false);
        assert_eq!(label, "Task 1, from 2024-01-01 to 2024-01-05, 50% complete");
    }

    #[test]
    fn test_task_aria_label_selected() {
        let label = task_aria_label("Task 1", "2024-01-01", "2024-01-05", 0.75, true, false);
        assert_eq!(
            label,
            "Task 1, from 2024-01-01 to 2024-01-05, 75% complete, selected"
        );
    }

    #[test]
    fn test_task_aria_label_readonly() {
        let label = task_aria_label("Task 1", "2024-01-01", "2024-01-05", 1.0, false, true);
        assert_eq!(
            label,
            "Task 1, from 2024-01-01 to 2024-01-05, 100% complete, read-only"
        );
    }

    #[test]
    fn test_zoom_aria_label_can_zoom() {
        let label = zoom_aria_label("in", true, "day");
        assert_eq!(label, "Zoom in, current view: day");
    }

    #[test]
    fn test_zoom_aria_label_cannot_zoom() {
        let label = zoom_aria_label("out", false, "year");
        assert_eq!(label, "Cannot zoom out from year view");
    }
}
