# Gantt Chart Component Specification

**Document Version:** 1.0
**Created:** 2026-01-30
**Target Framework:** Leptos + daisyUI
**Component Status:** 🔴 Not Started

---

## Executive Summary

This document outlines the requirements and development plan for a production-ready Gantt chart component for the leptos-daisyui-rs library. The component will enable users to visualize and manage project timelines with interactive task scheduling, dependencies, and resource allocation.

The design is based on analysis of best-in-class implementations including [Frappe Gantt](https://github.com/frappe/gantt), [DHTMLX Gantt](https://github.com/DHTMLX/gantt), [SVAR React Gantt](https://svar.dev/react/gantt/), and [gantt-task-react](https://github.com/MaTeMaTuK/gantt-task-react).

---

## Table of Contents

1. [Functional Requirements](#functional-requirements)
2. [Non-Functional Requirements](#non-functional-requirements)
3. [Interactivity Requirements](#interactivity-requirements)
4. [Visual UI Requirements](#visual-ui-requirements)
5. [Accessibility Requirements](#accessibility-requirements)
6. [API Design](#api-design)
7. [Development Plan](#development-plan)
8. [References](#references)

---

## Functional Requirements

### FR-1: Core Data Model

**Priority:** P0 (Critical)

The Gantt chart must support the following data model:

#### Task Properties
- **id** (String) - Unique task identifier
- **name** (String) - Task display name
- **start_date** (DateTime) - Task start date/time
- **end_date** (DateTime) - Task end date/time
- **duration** (Option<Duration>) - Calculated or explicit duration
- **progress** (f32, 0.0-1.0) - Task completion percentage
- **type** (TaskType) - Task, Milestone, or Project
- **parent_id** (Option<String>) - Parent task for hierarchical structure
- **dependencies** (Vec<TaskDependency>) - Task relationships
- **assignees** (Vec<String>) - Assigned resources
- **metadata** (HashMap<String, Value>) - Custom properties

#### Task Dependencies
- **Finish-to-Start (FS)** - Task B starts when Task A finishes
- **Start-to-Start (SS)** - Task B starts when Task A starts
- **Finish-to-Finish (FF)** - Task B finishes when Task A finishes
- **Start-to-Finish (SF)** - Task B finishes when Task A starts
- **Lag Time** - Optional delay between dependent tasks

**Reference:** [Gantt Chart Dependencies Guide](https://chisellabs.com/blog/gantt-chart-dependencies/)

### FR-2: Timeline Visualization

**Priority:** P0 (Critical)

- Display tasks as horizontal bars on a time-scaled axis
- Support multiple time scales: Hour, Day, Week, Month, Quarter, Year
- Auto-calculate optimal time scale based on date range
- Display today marker line
- Show weekends and non-working days with distinct styling
- Support custom working hours and holidays

### FR-3: Hierarchical Task Structure

**Priority:** P0 (Critical)

- Display tasks in collapsible tree structure
- Parent tasks automatically calculated from child tasks:
  - Start date = earliest child start
  - End date = latest child end
  - Progress = weighted average of children
- Expand/collapse functionality for task groups
- Visual indentation for nested tasks
- Parent task bars span entire child duration

### FR-4: Task Progress Tracking

**Priority:** P1 (High)

- Display progress as filled portion of task bar
- Visual progress indicator (0-100%)
- Different styling for:
  - Not started (0%)
  - In progress (1-99%)
  - Completed (100%)
  - Overdue tasks (past end date, not 100%)

### FR-5: Milestones

**Priority:** P1 (High)

- Display milestones as diamond markers
- Zero-duration events on timeline
- Distinct visual styling from regular tasks
- Support for milestone dependencies

### FR-6: Critical Path Visualization

**Priority:** P2 (Medium)

- Calculate and highlight critical path
- Show tasks with zero slack time
- Visual indicator for critical tasks
- Toggle critical path display on/off

### FR-7: Resource Management

**Priority:** P2 (Medium)

- Assign multiple resources to tasks
- Display resource avatars/initials on task bars
- Resource utilization view (separate feature)
- Filter tasks by assigned resource

### FR-8: Baseline Comparison

**Priority:** P3 (Low)

- Save baseline schedule snapshots
- Display baseline vs. actual timeline
- Visual indicators for schedule variance
- Compare multiple baselines

---

## Non-Functional Requirements

### NFR-1: Performance

**Priority:** P0 (Critical)

Based on [SVAR Gantt performance capabilities](https://svar.dev/react/gantt/):

- **Large Dataset Support:** Handle 10,000+ tasks without performance degradation
- **Virtual Scrolling:** Render only visible tasks (viewport + buffer)
- **Dynamic Loading:** Lazy-load task data as user scrolls
- **Smooth Interactions:** 60 FPS during drag operations
- **Initial Render:** < 500ms for 1,000 tasks
- **Memory Usage:** < 100MB for 5,000 tasks

**Implementation Strategy:**
- Use virtual DOM for task list
- Implement canvas-based timeline rendering
- Debounce scroll/resize events
- Optimize dependency calculation with caching

### NFR-2: Responsive Design

**Priority:** P0 (Critical)

- **Desktop:** Full-featured experience (1920x1080 optimal)
- **Tablet:** Simplified controls, touch-optimized (768px min)
- **Mobile:** Timeline view with horizontal scroll (not recommended < 768px)
- Responsive grid/timeline split ratio
- Collapsible task list on small screens
- Touch gestures for pan/zoom on mobile

### NFR-3: Browser Compatibility

**Priority:** P1 (High)

- Modern browsers (Chrome, Firefox, Safari, Edge)
- WebAssembly support required (Leptos)
- CSS Grid and Flexbox support
- SVG rendering support
- Local storage for preferences

### NFR-4: Data Integrity

**Priority:** P0 (Critical)

- Prevent circular dependencies
- Validate date ranges (start < end)
- Enforce minimum task duration
- Auto-adjust dependent tasks on date changes
- Maintain data consistency during edits

### NFR-5: Extensibility

**Priority:** P1 (High)

- Custom task renderers via slots
- Plugin system for extensions
- Event hooks for all interactions
- Theming through daisyUI/CSS variables
- Export API for custom views

---

## Interactivity Requirements

### INT-1: Drag and Drop

**Priority:** P0 (Critical)

Based on [DHTMLX drag-and-drop implementation](https://docs.dhtmlx.com/gantt/desktop__dnd.html):

#### Task Bar Dragging
- **Move Task:** Drag entire bar to change dates
- **Resize Left:** Drag start edge to change start date
- **Resize Right:** Drag end edge to change end date
- **Progress Drag:** Drag progress handle to update completion
- **Constraints:**
  - Respect min/max duration settings
  - Auto-adjust dependent tasks (optional)
  - Snap to time grid (configurable)
  - Prevent invalid date ranges

#### Task List Reordering
- Drag tasks within tree to change order
- Drag to change parent/child relationships
- Visual drop indicators
- Undo/redo support

#### Dependency Creation
- Drag from task connector to create dependency link
- Visual link preview during drag
- Validate dependency type
- Prevent circular dependencies

**Callbacks:**
```rust
on_task_drag: Callback<TaskDragEvent>
on_task_drop: Callback<TaskDropEvent>
on_before_task_drag: Callback<BeforeTaskDragEvent> // can prevent drag
```

### INT-2: Inline Editing

**Priority:** P1 (High)

- **Double-click task bar:** Open quick edit popover
- **Editable fields:**
  - Task name
  - Start/end dates
  - Duration
  - Progress percentage
  - Assignees
- **Click task name:** Inline text editing
- **Enter/Escape:** Save/cancel edit
- **Validation:** Real-time error feedback

### INT-3: Context Menus

**Priority:** P1 (High)

- **Right-click task bar:**
  - Edit task details
  - Delete task
  - Add dependency
  - Convert to milestone
  - Mark complete
  - Add subtask
- **Right-click timeline:**
  - Add new task
  - Zoom to fit
  - Export view
- **Keyboard shortcut equivalents**

### INT-4: Selection and Multi-Selection

**Priority:** P1 (High)

- **Single selection:** Click task
- **Multi-selection:**
  - Ctrl/Cmd + Click for multiple tasks
  - Shift + Click for range selection
  - Drag to select (rubber band)
- **Bulk operations:**
  - Batch delete
  - Batch reschedule
  - Batch assign
- **Visual selection state**

### INT-5: Zoom and Pan

**Priority:** P0 (Critical)

Based on modern [Gantt chart UX patterns](https://pageflows.com/resources/gantt-chart-example/):

#### Zoom Controls
- **Zoom In/Out:** Buttons or mouse wheel
- **Zoom to Fit:** Auto-scale to show all tasks
- **Zoom to Selection:** Focus on selected tasks
- **Preset Zoom Levels:**
  - Hours (for short projects)
  - Days (default)
  - Weeks
  - Months
  - Quarters
  - Years

#### Pan Controls
- **Horizontal Scroll:** Mouse wheel or scrollbar
- **Vertical Scroll:** Task list scrolling
- **Drag Timeline:** Pan by dragging background
- **Minimap:** Optional overview with viewport indicator
- **Keyboard:** Arrow keys for fine control

### INT-6: Filtering and Search

**Priority:** P2 (Medium)

- **Text Search:** Filter tasks by name
- **Status Filter:** Not started, In progress, Completed, Overdue
- **Resource Filter:** Show tasks for specific assignees
- **Date Range Filter:** Show tasks within date range
- **Critical Path Filter:** Show only critical tasks
- **Custom Filters:** User-defined filter expressions
- **Clear All Filters:** Reset to full view

### INT-7: Sorting

**Priority:** P2 (Medium)

- Sort by:
  - Task name (A-Z)
  - Start date (earliest/latest)
  - End date (earliest/latest)
  - Duration (shortest/longest)
  - Progress (least/most complete)
- Maintain parent-child relationships during sort
- Multi-column sorting

---

## Visual UI Requirements

### VIS-1: Design System Integration

**Priority:** P0 (Critical)

Based on [Gantt chart design best practices](https://blog.netronic.com/9-basic-design-tips-for-user-friendly-gantt-charts-part-1):

#### daisyUI Theme Compliance
- Use daisyUI color variables for theming
- Support light/dark mode toggle
- Respect theme color palette:
  - `primary` - Selected tasks
  - `secondary` - Parent tasks
  - `accent` - Milestones
  - `success` - Completed tasks (100%)
  - `warning` - At-risk tasks (80-99%, near deadline)
  - `error` - Overdue tasks
  - `base-100/200/300` - Background layers
- Use daisyUI spacing and border radius variables

#### Modern Flat Design
- Flat, monochrome rectangles (no 3D gradients)
- Minimal use of borders
- Soft shadows for elevation
- Clean typography (system fonts)
- Dark grey text (#374151) instead of pure black
- Generous white space between elements

### VIS-2: Task Bar Styling

**Priority:** P0 (Critical)

Inspired by [Asana Timeline design](https://medium.com/asana-design/designing-timeline-lessons-learned-from-our-journey-beyond-gantt-charts-645e80177aaa):

#### Task Bar Components
```
┌─────────────────────────────────────┐
│ [●●] Task Name         50%  [AV AV] │ ← Task bar
└─────────────────────────────────────┘
  ↑     ↑                ↑      ↑
  Grip  Name          Progress Avatars
```

- **Height:** 32px (medium), 24px (compact), 40px (comfortable)
- **Corners:** Rounded (--rounded-box from daisyUI)
- **Progress Fill:** Semi-transparent overlay
- **Hover State:** Slight elevation, border highlight
- **Selected State:** Border highlight, shadow
- **Text:** Left-aligned, truncate with ellipsis

#### Task Types
- **Regular Task:** Solid colored bar
- **Parent Task:** Taller bar with subtle pattern
- **Milestone:** Diamond shape (◆)
- **Summary Task:** Top/bottom borders only

#### Dependency Links
- **Line Style:** Curved bezier paths (not straight lines)
- **Colors:**
  - Normal: `base-content` at 30% opacity
  - Hovered: `primary` at 70% opacity
  - Critical path: `error` at 100%
- **Arrow heads:** Small triangular markers
- **Link types:**
  - Finish-to-Start: → arrow
  - Start-to-Start: ← arrow
  - Finish-to-Finish: ← arrow
  - Start-to-Finish: ↔ arrow

### VIS-3: Timeline Grid

**Priority:** P0 (Critical)

#### Grid Layout
```
┌─────────────────────────────────────────────────────┐
│  Jan 2026    │   Feb 2026    │   Mar 2026           │ ← Major scale
├──────────────┼───────────────┼──────────────────────┤
│ 1│ 8│15│22│29│ 5│12│19│26│ 5│12│19│26│              │ ← Minor scale
├──┼──┼──┼──┼──┼──┼──┼──┼──┼──┼──┼──┼──┼──────────────┤
│▓▓│  │  │  │  │▓▓│  │  │  │  │▓▓│  │  │  │  Today    │ ← Weekend shading
└──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──────────────┘
```

- **Major Scale:** Month/Quarter/Year headers
- **Minor Scale:** Day/Week subdivisions
- **Grid Lines:**
  - Major: 1px solid at 20% opacity
  - Minor: 1px solid at 10% opacity
- **Weekend Shading:** Subtle background tint (5% darker)
- **Today Marker:** Vertical red line with label
- **Hover Column:** Highlight on mouse over

### VIS-4: Task List Panel

**Priority:** P0 (Critical)

#### Column Layout
```
┌───────────────────────────────────┐
│ ▼ Task Name          │ Start │ End│
├───────────────────────────────────┤
│ ▼ Project A          │ 01/01 │ 03/31
│   ▸ Phase 1          │ 01/01 │ 01/31
│     └─ Task 1.1      │ 01/01 │ 01/15
│     └─ Task 1.2      │ 01/16 │ 01/31
│   ▾ Phase 2          │ 02/01 │ 03/31
│     └─ Task 2.1      │ 02/01 │ 02/15
└───────────────────────────────────┘
```

- **Expand/Collapse Icons:** ▸ (collapsed) / ▾ (expanded)
- **Indentation:** 20px per level, max 5 levels
- **Tree Lines:** Optional connectors (└─)
- **Columns:**
  - Task name (expandable, min 200px)
  - Start date (80px)
  - End date (80px)
  - Duration (60px)
  - Progress (80px with bar)
  - Assignees (100px with avatars)
  - Custom columns (user-defined)
- **Column Resize:** Drag column dividers
- **Column Reorder:** Drag column headers
- **Sticky Header:** Fixed during scroll

### VIS-5: Visual Indicators

**Priority:** P1 (High)

#### Icons and Badges
- **Progress Badge:** Circular badge showing percentage
- **Dependency Icon:** Chain link icon (🔗)
- **Blocked Icon:** Warning triangle (⚠️) for blocked tasks
- **Critical Icon:** Exclamation (!) for critical path
- **Overdue Icon:** Clock icon (🕐) in red
- **Comment Icon:** Speech bubble with count
- **Attachment Icon:** Paperclip with count

#### Color Coding
- **Status Colors:**
  - Not Started: `base-300` (neutral grey)
  - In Progress: `primary` (blue)
  - Completed: `success` (green)
  - Overdue: `error` (red)
  - At Risk: `warning` (yellow/orange)
- **Priority Colors** (optional border accent):
  - P0/Critical: Red
  - P1/High: Orange
  - P2/Medium: Yellow
  - P3/Low: Green

### VIS-6: Dark Mode Support

**Priority:** P1 (High)

Following [modern UI design patterns](https://pageflows.com/resources/gantt-chart-example/):

- Auto-switch with daisyUI theme
- Adjusted opacity for grid lines (higher contrast)
- Softer task bar colors in dark mode
- Inverted weekend shading
- Maintained color hierarchy and contrast ratios (WCAG AA)

### VIS-7: Loading and Empty States

**Priority:** P1 (High)

- **Loading State:**
  - Skeleton screen with placeholder bars
  - Pulsing animation
  - Loading spinner for long operations
- **Empty State:**
  - Centered illustration + message
  - "Add your first task" CTA button
  - Getting started tips
- **Error State:**
  - Error icon + message
  - Retry button
  - Error details (collapsible)

---

## Accessibility Requirements

### ACC-1: ARIA Implementation

**Priority:** P0 (Critical)

Based on [WCAG 2.2 standards](https://www.telerik.com/design-system/docs/components/gantt/accessibility/):

#### ARIA Roles
```html
<div role="treegrid" aria-label="Project Gantt Chart">
  <div role="rowgroup">
    <div role="row" aria-level="1" aria-expanded="true">
      <div role="gridcell">Task Name</div>
      <div role="gridcell">Timeline Bar</div>
    </div>
  </div>
</div>
```

#### ARIA Attributes
- **aria-label:** Descriptive labels for all components
- **aria-expanded:** Expand/collapse state for parent tasks
- **aria-selected:** Selection state for tasks
- **aria-valuenow/min/max:** Progress percentage
- **aria-controls:** Relationship between controls and targets
- **aria-describedby:** Additional task information
- **aria-live:** Announce dynamic updates to screen readers
- **aria-hidden:** Hide decorative elements from assistive tech

### ACC-2: Keyboard Navigation

**Priority:** P0 (Critical)

Following [WAI-ARIA keyboard patterns](https://docs.dhtmlx.com/gantt/desktop__accessibility.html):

#### Navigation Keys
| Key | Action |
|-----|--------|
| **Tab** | Move focus between major components (list, timeline, toolbar) |
| **Shift+Tab** | Move focus backward |
| **Arrow Up/Down** | Navigate between tasks in list |
| **Arrow Left/Right** | Navigate timeline (pan), or collapse/expand tree nodes |
| **Enter** | Open task edit dialog |
| **Space** | Select/deselect task |
| **Ctrl/Cmd+A** | Select all tasks |
| **Delete/Backspace** | Delete selected tasks (with confirmation) |
| **Ctrl/Cmd+Z** | Undo last action |
| **Ctrl/Cmd+Y** | Redo last action |
| **Ctrl/Cmd+F** | Open search/filter |
| **Escape** | Close dialogs, cancel operations, clear selection |
| **Home/End** | Jump to first/last task |
| **Page Up/Down** | Scroll page in task list |
| **+/-** | Zoom in/out timeline |
| **0** | Reset zoom to default |

#### Focus Management
- Visible focus indicators (2px outline, high contrast)
- Logical tab order (left-to-right, top-to-bottom)
- Focus trap in modal dialogs
- Focus restoration after operations
- Skip links for long task lists

### ACC-3: Screen Reader Support

**Priority:** P0 (Critical)

- **Announce task properties:** "Task A, 50% complete, January 1 to January 15"
- **Announce relationships:** "Task B depends on Task A, finish-to-start"
- **Live regions:** Announce changes during drag/drop
- **Status messages:** Confirm actions (task created, deleted, updated)
- **Error messages:** Clear, actionable error descriptions
- **Landmark regions:** Header, navigation, main content, toolbar

### ACC-4: Color Contrast

**Priority:** P0 (Critical)

- **WCAG AA compliance:** 4.5:1 contrast for normal text, 3:1 for large text
- **Do not rely on color alone:** Use icons + text labels
- **High contrast mode:** Support Windows High Contrast
- **Focus indicators:** Distinct from selection state

### ACC-5: Alternative Input Methods

**Priority:** P1 (High)

- **Touch screen:** Larger touch targets (44x44px minimum)
- **Voice control:** Proper labeling for voice commands
- **Switch control:** Support for assistive devices
- **Reduced motion:** Respect `prefers-reduced-motion` setting

---

## API Design

### Component Props

```rust
#[component]
pub fn GanttChart(
    /// Task data (reactive signal for updates)
    #[prop(into)]
    tasks: Signal<Vec<GanttTask>>,

    /// Column configuration for task list
    #[prop(optional, into)]
    columns: Option<Signal<Vec<GanttColumn>>>,

    /// Initial timeline view mode
    #[prop(optional, default = ViewMode::Day)]
    view_mode: ViewMode,

    /// Enable/disable task editing
    #[prop(optional, default = true)]
    editable: bool,

    /// Show/hide task dependencies
    #[prop(optional, default = true)]
    show_dependencies: bool,

    /// Show/hide critical path
    #[prop(optional, default = false)]
    show_critical_path: bool,

    /// Show/hide progress indicators
    #[prop(optional, default = true)]
    show_progress: bool,

    /// Enable drag and drop
    #[prop(optional, default = true)]
    drag_and_drop: bool,

    /// Working days (Mon-Fri = vec![1,2,3,4,5])
    #[prop(optional, default = vec![1,2,3,4,5])]
    working_days: Vec<u8>,

    /// Custom holidays (dates when work doesn't happen)
    #[prop(optional)]
    holidays: Option<Vec<NaiveDate>>,

    /// Task height in pixels
    #[prop(optional, default = GanttTaskHeight::Medium)]
    task_height: GanttTaskHeight,

    /// Split ratio between list and timeline (0.0-1.0)
    #[prop(optional, default = 0.3)]
    list_width_ratio: f32,

    /// Minimum task duration in minutes
    #[prop(optional, default = 60)]
    min_task_duration: u32,

    /// Auto-schedule dependent tasks on changes
    #[prop(optional, default = false)]
    auto_schedule: bool,

    /// Theme variant
    #[prop(optional, into)]
    theme: Option<GanttTheme>,

    // Event Callbacks
    /// Fires when task is selected
    #[prop(optional, into)]
    on_task_select: Option<Callback<GanttTask>>,

    /// Fires when task is updated (drag, edit, etc.)
    #[prop(optional, into)]
    on_task_update: Option<Callback<TaskUpdateEvent>>,

    /// Fires when task is created
    #[prop(optional, into)]
    on_task_create: Option<Callback<GanttTask>>,

    /// Fires when task is deleted
    #[prop(optional, into)]
    on_task_delete: Option<Callback<String>>, // task_id

    /// Fires when dependency is created
    #[prop(optional, into)]
    on_dependency_create: Option<Callback<TaskDependency>>,

    /// Fires when dependency is deleted
    #[prop(optional, into)]
    on_dependency_delete: Option<Callback<String>>, // link_id

    /// Fires before task drag starts (can prevent)
    #[prop(optional, into)]
    on_before_task_drag: Option<Callback<BeforeTaskDragEvent>>,

    /// Fires during task drag
    #[prop(optional, into)]
    on_task_drag: Option<Callback<TaskDragEvent>>,

    /// Fires when task drag ends
    #[prop(optional, into)]
    on_task_drop: Option<Callback<TaskDropEvent>>,

    /// Fires when view mode changes (zoom)
    #[prop(optional, into)]
    on_view_mode_change: Option<Callback<ViewMode>>,

    /// Spread attributes for root element
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,

    /// Node reference for root element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView
```

### Data Structures

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GanttTask {
    pub id: String,
    pub name: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub progress: f32, // 0.0 to 1.0
    pub task_type: TaskType,
    pub parent_id: Option<String>,
    pub dependencies: Vec<TaskDependency>,
    pub assignees: Vec<String>,
    pub color: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskType {
    Task,
    Milestone,
    Project,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskDependency {
    pub source_id: String,
    pub target_id: String,
    pub dependency_type: DependencyType,
    pub lag_days: f32, // can be negative for lead time
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DependencyType {
    FinishToStart,
    StartToStart,
    FinishToFinish,
    StartToFinish,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ViewMode {
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GanttTaskHeight {
    Compact,  // 24px
    Medium,   // 32px
    Comfortable, // 40px
}

#[derive(Clone, Debug)]
pub struct TaskUpdateEvent {
    pub task_id: String,
    pub field: TaskField,
    pub old_value: serde_json::Value,
    pub new_value: serde_json::Value,
}

#[derive(Clone, Debug)]
pub enum TaskField {
    Name,
    StartDate,
    EndDate,
    Progress,
    Assignees,
    Custom(String),
}

#[derive(Clone, Debug)]
pub struct BeforeTaskDragEvent {
    pub task_id: String,
    pub drag_mode: DragMode,
    pub can_proceed: RwSignal<bool>, // Set to false to prevent drag
}

#[derive(Clone, Debug)]
pub enum DragMode {
    Move,
    ResizeStart,
    ResizeEnd,
    Progress,
}

#[derive(Clone, Debug)]
pub struct TaskDragEvent {
    pub task_id: String,
    pub drag_mode: DragMode,
    pub new_start: NaiveDateTime,
    pub new_end: NaiveDateTime,
    pub new_progress: f32,
}

#[derive(Clone, Debug)]
pub struct TaskDropEvent {
    pub task_id: String,
    pub drag_mode: DragMode,
    pub final_start: NaiveDateTime,
    pub final_end: NaiveDateTime,
    pub final_progress: f32,
    pub affected_tasks: Vec<String>, // If auto-schedule enabled
}

#[derive(Clone, Debug)]
pub struct GanttColumn {
    pub id: String,
    pub title: String,
    pub width: u32,
    pub field: ColumnField,
    pub sortable: bool,
    pub resizable: bool,
}

#[derive(Clone, Debug)]
pub enum ColumnField {
    Name,
    StartDate,
    EndDate,
    Duration,
    Progress,
    Assignees,
    Custom(String),
}
```

### Public Methods (via NodeRef)

```rust
impl GanttChart {
    /// Get all tasks
    pub fn get_tasks(&self) -> Vec<GanttTask>;

    /// Get task by ID
    pub fn get_task(&self, id: &str) -> Option<GanttTask>;

    /// Add new task
    pub fn add_task(&self, task: GanttTask);

    /// Update existing task
    pub fn update_task(&self, id: &str, task: GanttTask);

    /// Delete task
    pub fn delete_task(&self, id: &str);

    /// Add dependency link
    pub fn add_dependency(&self, dependency: TaskDependency);

    /// Remove dependency link
    pub fn remove_dependency(&self, source_id: &str, target_id: &str);

    /// Calculate critical path
    pub fn calculate_critical_path(&self) -> Vec<String>; // task_ids

    /// Zoom to fit all tasks
    pub fn zoom_to_fit(&self);

    /// Zoom to selected tasks
    pub fn zoom_to_selection(&self);

    /// Export to various formats
    pub fn export_to_json(&self) -> String;
    pub fn export_to_csv(&self) -> String;
    pub fn export_to_png(&self) -> Vec<u8>; // Canvas snapshot

    /// Undo last action
    pub fn undo(&self);

    /// Redo last undone action
    pub fn redo(&self);

    /// Clear selection
    pub fn clear_selection(&self);

    /// Select task(s)
    pub fn select_task(&self, id: &str, add_to_selection: bool);
    pub fn select_tasks(&self, ids: Vec<String>);

    /// Expand/collapse task
    pub fn expand_task(&self, id: &str);
    pub fn collapse_task(&self, id: &str);
    pub fn expand_all(&self);
    pub fn collapse_all(&self);
}
```

---

## Development Plan

### Phase 1: Foundation (Week 1-2)

**Goal:** Core data structures and basic rendering

#### Deliverables
- [ ] Create `gantt/` module structure
- [ ] Implement data models (`GanttTask`, `TaskDependency`, etc.)
- [ ] Build date/time utilities for timeline calculations
- [ ] Create basic component structure with props
- [ ] Implement simple task list view (no tree, no timeline yet)
- [ ] Add unit tests for data models
- [ ] Add unit tests for date calculations

#### Technical Approach
- Use `chrono` crate for date/time handling
- Store tasks in `RwSignal<Vec<GanttTask>>`
- Create separate modules:
  - `gantt/models.rs` - Data structures
  - `gantt/utils.rs` - Date calculations, dependency validation
  - `gantt/component.rs` - Main component
  - `gantt/task_list.rs` - Task list panel
  - `gantt/timeline.rs` - Timeline panel

**Reference Implementation:** [gantt-task-react data model](https://github.com/MaTeMaTuK/gantt-task-react)

### Phase 2: Static Timeline (Week 3-4)

**Goal:** Render timeline grid and task bars

#### Deliverables
- [ ] Implement timeline grid rendering (SVG)
- [ ] Calculate time scales (day, week, month)
- [ ] Render task bars on timeline
- [ ] Implement weekend/holiday shading
- [ ] Add today marker
- [ ] Synchronize task list and timeline scrolling
- [ ] Add horizontal scrollbar for timeline
- [ ] Responsive split panel with resize handle

#### Technical Approach
- Use SVG for timeline rendering (better for scaling)
- Calculate pixel positions from dates:
  ```rust
  fn date_to_x(date: NaiveDateTime, scale: ViewMode) -> f64
  fn x_to_date(x: f64, scale: ViewMode) -> NaiveDateTime
  ```
- Implement virtual scrolling for task bars
- Use CSS Grid for time scale headers

**Reference Implementation:** [Frappe Gantt timeline rendering](https://github.com/frappe/gantt)

### Phase 3: Tree Structure (Week 5)

**Goal:** Hierarchical task display with expand/collapse

#### Deliverables
- [ ] Implement tree data structure
- [ ] Add expand/collapse functionality
- [ ] Calculate parent task dates from children
- [ ] Add visual indentation
- [ ] Implement tree lines (optional)
- [ ] Parent task bars span child duration
- [ ] Preserve expand/collapse state

#### Technical Approach
- Build task tree from flat list using `parent_id`
- Recursive component for nested tasks
- Store expanded state in `HashMap<String, bool>`
- Auto-calculate parent dates using:
  ```rust
  fn calculate_parent_dates(task_id: &str) -> (NaiveDateTime, NaiveDateTime)
  ```

**Reference Implementation:** [DHTMLX Gantt tree structure](https://docs.dhtmlx.com/gantt/)

### Phase 4: Basic Interactivity (Week 6-7)

**Goal:** Selection, zoom, and basic editing

#### Deliverables
- [ ] Task selection (single/multi)
- [ ] Zoom in/out controls
- [ ] Zoom to fit functionality
- [ ] Pan timeline with mouse drag
- [ ] Keyboard navigation (arrow keys)
- [ ] Context menu (right-click)
- [ ] Inline task name editing
- [ ] Double-click to open edit dialog

#### Technical Approach
- Track selected tasks in `RwSignal<HashSet<String>>`
- Implement zoom levels with scale multiplier
- Use `on:wheel` for zoom on mouse wheel
- Use `on:mousedown` + `on:mousemove` for panning
- Create `ContextMenu` sub-component
- Create `TaskEditDialog` modal

**Reference Implementation:** [SVAR Gantt interactions](https://svar.dev/react/gantt/)

### Phase 5: Drag and Drop (Week 8-9)

**Goal:** Full drag-and-drop functionality

#### Deliverables
- [ ] Drag to move task (change dates)
- [ ] Drag to resize task start
- [ ] Drag to resize task end
- [ ] Drag progress indicator
- [ ] Visual drag preview
- [ ] Snap to grid during drag
- [ ] Enforce constraints (min duration, working days)
- [ ] Undo/redo support for drag operations

#### Technical Approach
- Use Leptos event handlers: `on:mousedown`, `on:mousemove`, `on:mouseup`
- Track drag state:
  ```rust
  struct DragState {
      is_dragging: bool,
      task_id: String,
      drag_mode: DragMode,
      start_x: f64,
      original_start: NaiveDateTime,
      original_end: NaiveDateTime,
  }
  ```
- Calculate new dates from mouse position
- Update task dates reactively during drag
- Commit changes on mouse up
- Store undo history in vector of operations

**Reference Implementation:** [DHTMLX Gantt drag-and-drop](https://docs.dhtmlx.com/gantt/desktop__dnd.html)

### Phase 6: Dependencies (Week 10-11)

**Goal:** Task dependencies and links

#### Deliverables
- [ ] Render dependency links (SVG paths)
- [ ] Calculate link paths (curved bezier)
- [ ] Support 4 dependency types (FS, SS, FF, SF)
- [ ] Drag to create dependency
- [ ] Validate dependencies (no circular)
- [ ] Delete dependency (context menu)
- [ ] Dependency hover/selection
- [ ] Auto-schedule dependent tasks (optional)

#### Technical Approach
- Store dependencies in `Vec<TaskDependency>`
- Render as SVG `<path>` elements
- Use quadratic bezier curves for smooth links:
  ```rust
  fn calculate_link_path(source: &GanttTask, target: &GanttTask, dep_type: DependencyType) -> String
  ```
- Implement cycle detection using DFS
- Auto-schedule using topological sort + forward pass

**Reference Implementation:** [Gantt chart dependencies guide](https://chisellabs.com/blog/gantt-chart-dependencies/)

### Phase 7: Advanced Features (Week 12-14)

**Goal:** Critical path, filtering, progress tracking

#### Deliverables
- [ ] Calculate critical path
- [ ] Highlight critical tasks
- [ ] Filter tasks by status/assignee/date
- [ ] Search tasks by name
- [ ] Progress percentage display
- [ ] Milestone rendering (diamonds)
- [ ] Resource avatars on task bars
- [ ] Custom task colors

#### Technical Approach
- Critical path algorithm:
  1. Calculate early start/finish (forward pass)
  2. Calculate late start/finish (backward pass)
  3. Find tasks with slack = 0
- Implement filter predicates:
  ```rust
  type TaskFilter = Box<dyn Fn(&GanttTask) -> bool>;
  ```
- Render milestones as rotated squares (45°)
- Use `Avatar` component for assignees

**Reference Implementation:** Critical path algorithms from project management texts

### Phase 8: Performance Optimization (Week 15-16)

**Goal:** Handle large datasets efficiently

#### Deliverables
- [ ] Implement virtual scrolling for task list
- [ ] Lazy render timeline (visible range only)
- [ ] Debounce scroll/resize events
- [ ] Memoize expensive calculations
- [ ] Canvas rendering for timeline (instead of SVG)
- [ ] Worker thread for dependency calculation
- [ ] Performance benchmarks (1k, 5k, 10k tasks)

#### Technical Approach
- Use `IntersectionObserver` for virtual scrolling
- Render only tasks in viewport + buffer:
  ```rust
  fn get_visible_tasks(scroll_top: f64, viewport_height: f64) -> Vec<GanttTask>
  ```
- Switch from SVG to Canvas for better performance with many tasks
- Use `web_sys::Worker` for background calculations
- Optimize with `Memo` signals for derived state

**Reference Implementation:** [SVAR Gantt performance features](https://svar.dev/react/gantt/)

### Phase 9: Accessibility & Testing (Week 17-18)

**Goal:** WCAG 2.2 compliance and comprehensive tests

#### Deliverables
- [ ] Full ARIA implementation
- [ ] Complete keyboard navigation
- [ ] Screen reader announcements
- [ ] Focus management
- [ ] High contrast mode support
- [ ] Reduced motion support
- [ ] Unit tests (80% coverage)
- [ ] Integration tests
- [ ] Accessibility audit (axe-core)

#### Technical Approach
- Add ARIA attributes to all interactive elements
- Implement keyboard event handlers
- Use `aria-live` regions for announcements
- Create test harness with sample data
- Use `web-sys` testing utilities
- Run automated accessibility tests

**Reference Implementation:** [DHTMLX Gantt accessibility](https://docs.dhtmlx.com/gantt/desktop__accessibility.html)

### Phase 10: Polish & Documentation (Week 19-20)

**Goal:** Production-ready component with docs

#### Deliverables
- [ ] Demo page with all features
- [ ] API documentation (rustdoc)
- [ ] Usage examples
- [ ] Migration guide (if needed)
- [ ] Performance tuning
- [ ] Error handling polish
- [ ] Loading states
- [ ] Empty states
- [ ] Export functionality (JSON, CSV, PNG)

#### Technical Approach
- Create comprehensive demo in showcase app
- Write detailed rustdoc comments
- Add code examples to documentation
- Implement export using `web_sys` APIs
- Add skeleton loaders for async operations

---

## Technical Stack

### Core Dependencies
```toml
[dependencies]
leptos = "0.8"
leptos_daisyui_rs = { path = ".." }
chrono = "0.4"  # Date/time handling
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
web-sys = { version = "0.3", features = [
    "HtmlCanvasElement",
    "CanvasRenderingContext2d",
    "SvgElement",
    "IntersectionObserver",
] }
wasm-bindgen = "0.2"
petgraph = "0.6"  # For dependency graph algorithms
```

### Development Dependencies
```toml
[dev-dependencies]
wasm-bindgen-test = "0.3"
```

---

## File Structure

```
src/components/gantt/
├── mod.rs                      # Public exports
├── component.rs                # Main GanttChart component
├── models.rs                   # Data structures
├── utils/
│   ├── mod.rs
│   ├── date_calc.rs           # Date/time calculations
│   ├── dependency_graph.rs    # Dependency validation & critical path
│   └── geometry.rs            # Position calculations
├── task_list/
│   ├── mod.rs
│   ├── component.rs           # Task list panel
│   ├── task_row.rs            # Individual task row
│   └── tree_node.rs           # Tree structure logic
├── timeline/
│   ├── mod.rs
│   ├── component.rs           # Timeline panel
│   ├── grid.rs                # Time grid rendering
│   ├── task_bar.rs            # Task bar rendering
│   └── dependency_link.rs     # Dependency link rendering
├── interactions/
│   ├── mod.rs
│   ├── drag_drop.rs           # Drag & drop logic
│   ├── selection.rs           # Selection management
│   └── zoom_pan.rs            # Zoom & pan controls
├── dialogs/
│   ├── mod.rs
│   ├── task_edit.rs           # Task edit dialog
│   └── context_menu.rs        # Context menu
└── style.rs                   # Enums for styling options
```

---

## Testing Strategy

### Unit Tests
- Data model validation
- Date calculation functions
- Dependency graph algorithms
- Critical path calculation
- Tree structure building

### Integration Tests
- Task CRUD operations
- Drag and drop workflows
- Dependency creation/deletion
- Zoom and pan behavior
- Filter and search functionality

### Accessibility Tests
- Keyboard navigation flows
- Screen reader announcements
- Focus management
- ARIA attribute validation

### Performance Tests
- Render 10,000 tasks
- Measure initial load time
- Measure interaction latency
- Memory profiling

### Visual Regression Tests
- Screenshot comparison for different states
- Theme switching
- Responsive breakpoints

---

## Open Questions & Future Enhancements

### Questions for Design Review
1. Should we support real-time collaboration (multiple users editing)?
2. What export formats are required (JSON, CSV, PNG, PDF)?
3. Do we need baseline comparison features?
4. Should resources have capacity/allocation tracking?
5. Print-friendly view requirement?

### Future Enhancement Ideas
- **Resource View:** Gantt by assignee instead of by task
- **Swimlanes:** Group tasks by category/team
- **Custom Fields:** User-defined task metadata
- **Templates:** Pre-configured project templates
- **Recurring Tasks:** Auto-generate periodic tasks
- **Notifications:** Deadline reminders, overdue alerts
- **Integration:** Import/export to MS Project, JIRA, etc.
- **Mobile App:** Native mobile experience
- **Offline Mode:** Service worker for offline editing
- **AI Features:** Smart scheduling suggestions

---

## References

### Best-in-Class Libraries
1. [Frappe Gantt](https://github.com/frappe/gantt) - Simple, beautiful JavaScript Gantt chart
2. [DHTMLX Gantt](https://github.com/DHTMLX/gantt) - Enterprise-grade GPL Gantt component
3. [SVAR React Gantt](https://svar.dev/react/gantt/) - Modern React Gantt with TypeScript
4. [gantt-task-react](https://github.com/MaTeMaTuK/gantt-task-react) - Lightweight React Gantt

### Design & UX References
5. [Gantt Chart Design Best Practices](https://pageflows.com/resources/gantt-chart-example/) - UI/UX patterns
6. [Asana Timeline Design](https://medium.com/asana-design/designing-timeline-lessons-learned-from-our-journey-beyond-gantt-charts-645e80177aaa) - Modern approach to project timelines
7. [User-Friendly Gantt Charts](https://blog.netronic.com/9-basic-design-tips-for-user-friendly-gantt-charts-part-1) - Design tips
8. [Reimagining Gantt Charts for UX](https://blog.logrocket.com/ux-design/reimagining-gantt-charts-ux-project-management/) - UX project management

### Technical Implementation
9. [DHTMLX Drag & Drop Docs](https://docs.dhtmlx.com/gantt/desktop__dnd.html) - Drag-and-drop implementation
10. [DHTMLX Accessibility Docs](https://docs.dhtmlx.com/gantt/desktop__accessibility.html) - Accessibility features
11. [Gantt Dependencies Guide](https://chisellabs.com/blog/gantt-chart-dependencies/) - Dependency types and management

### Accessibility Standards
12. [WCAG 2.2 Guidelines](https://www.w3.org/WAI/WCAG22/quickref/) - Web accessibility standards
13. [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/) - ARIA patterns
14. [Telerik Gantt Accessibility](https://www.telerik.com/design-system/docs/components/gantt/accessibility/) - Accessibility implementation example

### Additional Resources
15. [Top React Gantt Libraries 2026](https://svar.dev/blog/top-react-gantt-charts/) - Comparison of modern libraries
16. [Best JavaScript Gantt Libraries](https://www.anychart.com/blog/2025/11/05/best-javascript-gantt-chart-libraries/) - Comprehensive guide
17. [ClickUp Gantt Dependencies](https://clickup.com/blog/gantt-chart-dependencies/) - Dependency management patterns

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-30 | Initial specification document |

---

## Approval & Sign-off

**Document Status:** 📋 Draft - Awaiting Review

**Stakeholders:**
- [ ] Product Owner
- [ ] Tech Lead
- [ ] UX Designer
- [ ] Accessibility Expert

**Next Steps:**
1. Review and approve specification
2. Create GitHub issue with phased milestones
3. Assign development team
4. Begin Phase 1 implementation
