/// Direction variants for timeline layout orientation.
///
/// The `TimelineDirection` enum controls how the timeline is oriented,
/// affecting the flow and positioning of timeline items and their content.
///
/// # Variants
///
/// - `Vertical` - Timeline flows vertically (default, top to bottom)
/// - `Horizontal` - Timeline flows horizontally (left to right)
///
/// # CSS Class Mapping
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Vertical` | `timeline-vertical` | Top-to-bottom flow |
/// | `Horizontal` | `timeline-horizontal` | Left-to-right flow |
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         // Vertical timeline (default)
///         <Timeline direction=TimelineDirection::Vertical>
///             // ... timeline items
///         </Timeline>
///
///         // Horizontal timeline
///         <Timeline direction=TimelineDirection::Horizontal>
///             // ... timeline items
///         </Timeline>
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum TimelineDirection {
    /// Vertical timeline flowing from top to bottom (default)
    #[default]
    Vertical,
    /// Horizontal timeline flowing from left to right
    Horizontal,
}

impl TimelineDirection {
    /// Returns the corresponding CSS class for the timeline direction.
    ///
    /// # Returns
    ///
    /// A static string slice containing the CSS class name.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TimelineDirection::Vertical => "timeline-vertical",
            TimelineDirection::Horizontal => "timeline-horizontal",
        }
    }
}

/// Position variants for timeline items affecting connector line display.
///
/// The `TimelineItemPosition` enum determines which connector lines are shown
/// for a timeline item, enabling proper visual flow between timeline events.
/// This is crucial for creating cohesive timeline layouts.
///
/// # Variants
///
/// - `Start` - First item in timeline (shows connector after)
/// - `End` - Last item in timeline (shows connector before)
/// - `Between` - Middle item (shows connectors before and after)
///
/// # Position Logic
///
/// The position affects which `<hr>` elements are rendered:
/// - `Start` and `Between` show an ending connector line
/// - `End` and `Between` show a starting connector line
/// - Only `Between` shows both connector lines
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Timeline>
///             // First timeline item
///             <TimelineItem position=TimelineItemPosition::Start>
///                 // ... content
///             </TimelineItem>
///             
///             // Middle timeline items
///             <TimelineItem position=TimelineItemPosition::Between>
///                 // ... content
///             </TimelineItem>
///             
///             // Last timeline item
///             <TimelineItem position=TimelineItemPosition::End>
///                 // ... content
///             </TimelineItem>
///         </Timeline>
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum TimelineItemPosition {
    /// First item in the timeline (shows connector line after)
    #[default]
    Start,
    /// Last item in the timeline (shows connector line before)
    End,
    /// Middle item in the timeline (shows connector lines before and after)
    Between,
}

impl TimelineItemPosition {
    /// Returns true if this is the start position.
    ///
    /// Used internally to determine connector line rendering.
    pub fn is_start(&self) -> bool {
        matches!(self, TimelineItemPosition::Start)
    }

    /// Returns true if this is the end position.
    ///
    /// Used internally to determine connector line rendering.
    pub fn is_end(&self) -> bool {
        matches!(self, TimelineItemPosition::End)
    }

    /// Returns true if this is the between position.
    ///
    /// Used internally to determine connector line rendering.
    pub fn is_between(&self) -> bool {
        matches!(self, TimelineItemPosition::Between)
    }
}
