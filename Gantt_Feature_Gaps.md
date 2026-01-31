# Gantt Chart Feature Gap Analysis

**Document Version:** 1.0
**Created:** 2026-01-30
**Analysis Scope:** Comparison of Gantt_Component.md specification against industry leaders (Frappe Gantt, SVAR Gantt, DHTMLX Gantt)

---

## Executive Summary

This document identifies **27 significant features** present in industry-leading Gantt chart libraries but missing from our initial specification. These gaps represent opportunities to enhance our component's competitiveness and address real-world use cases.

**Key Findings:**
- **8 P0/P1 features** critical for production readiness
- **11 P2 features** that significantly improve user experience
- **8 P3 features** that provide enterprise-grade capabilities

The analysis focuses on practical features that add business value, with implementation complexity estimates and phased rollout recommendations.

---

## Table of Contents

1. [P0 Priority Gaps (Critical)](#p0-priority-gaps-critical)
2. [P1 Priority Gaps (High)](#p1-priority-gaps-high)
3. [P2 Priority Gaps (Medium)](#p2-priority-gaps-medium)
4. [P3 Priority Gaps (Low)](#p3-priority-gaps-low)
5. [Implementation Roadmap](#implementation-roadmap)
6. [Comparison Matrix](#comparison-matrix)

---

## P0 Priority Gaps (Critical)

### GAP-01: Configurable Read-Only Modes

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Granular control over which aspects of the Gantt chart are editable:
- Read-only mode (no edits allowed)
- Timeline-only editing (tasks editable in chart, not in grid)
- Grid-only editing (edit properties, not dates)
- Per-task read-only flags (some tasks locked)
- Per-column read-only settings

**Why It Matters:**
- Essential for role-based access control
- Prevents accidental changes to approved plans
- Required for stakeholder review/presentation modes
- Enables templates and locked baseline views

**Implementation Complexity:** Low

**Suggested Phase:** Phase 4 (Basic Interactivity)

**Specification Impact:** Add to API Design section
```rust
#[prop(optional, default = false)]
read_only: bool,

#[prop(optional)]
read_only_mode: Option<ReadOnlyMode>,

pub enum ReadOnlyMode {
    Full,           // No edits
    TimelineOnly,   // Can edit in chart, not grid
    GridOnly,       // Can edit properties, not dates
    Custom(Box<dyn Fn(&GanttTask, EditContext) -> bool>), // Custom logic
}
```

---

### GAP-02: Smart Rendering Optimizations

**Found In:** SVAR Gantt (10k+ tasks), DHTMLX Gantt

**Description:** Advanced performance optimizations beyond basic virtual scrolling:
- Render only visible + buffer zone (already planned)
- **NEW: Progressive loading** - Load data in chunks as user scrolls
- **NEW: Lazy dependency calculation** - Calculate links only for visible tasks
- **NEW: Level-of-detail rendering** - Simplified bars at high zoom-out levels
- **NEW: Debounced updates** - Batch rapid changes (e.g., during drag)

**Why It Matters:**
- Critical for enterprise projects (10,000+ tasks)
- Prevents browser freezing during scrolling/zooming
- Maintains 60 FPS even with massive datasets
- Reduces memory footprint

**Implementation Complexity:** High

**Suggested Phase:** Phase 8 (Performance Optimization) - expand scope

**Specification Impact:** Enhance NFR-1 performance requirements with specific techniques

---

### GAP-03: Localization (i18n) Support

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Internationalization for global teams:
- Translatable UI strings (buttons, labels, error messages)
- Date format localization (MM/DD/YYYY vs DD/MM/YYYY)
- First day of week configuration (Sunday vs Monday)
- Time format (12h vs 24h)
- Number format (decimal separator)
- RTL (right-to-left) language support

**Why It Matters:**
- Required for international teams
- Legal/regulatory requirements in some regions
- Improves accessibility for non-English speakers
- Standard feature in enterprise software

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 7 (Advanced Features)

**Specification Impact:** Add to API Design
```rust
#[prop(optional)]
locale: Option<GanttLocale>,

pub struct GanttLocale {
    pub language: String,           // "en", "es", "fr", etc.
    pub date_format: String,        // "MM/DD/YYYY"
    pub first_day_of_week: u8,     // 0 = Sunday, 1 = Monday
    pub translations: HashMap<String, String>,
}
```

---

### GAP-04: Comprehensive Keyboard Shortcuts

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Extended keyboard navigation beyond basic arrows:

**Currently Specified:**
- Arrow keys, Tab, Enter, Space, Delete, Undo/Redo, Zoom (+/-/0)

**Missing Shortcuts:**
| Shortcut | Action |
|----------|--------|
| **Ctrl/Cmd+D** | Duplicate selected task(s) |
| **Ctrl/Cmd+C/V** | Copy/paste tasks |
| **Ctrl/Cmd+X** | Cut tasks |
| **Ctrl/Cmd+N** | New task |
| **Ctrl/Cmd+E** | Edit selected task |
| **Ctrl/Cmd+Shift+Left/Right** | Move task start/end by 1 day |
| **Alt+Up/Down** | Move task up/down in list (reorder) |
| **Ctrl/Cmd+Shift+E** | Expand all |
| **Ctrl/Cmd+Shift+C** | Collapse all |
| **F2** | Rename task (inline edit) |
| **Shift+F10** | Open context menu |
| **T** | Jump to today |
| **G** | Go to specific date (open date picker) |

**Why It Matters:**
- Power users demand keyboard efficiency
- Accessibility requirement (users who can't use mouse)
- Standard in project management tools
- Dramatically speeds up task management

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 9 (Accessibility & Testing)

**Specification Impact:** Expand ACC-2 keyboard navigation table significantly

---

## P1 Priority Gaps (High)

### GAP-05: Custom Time Periods / Sprint Support

**Found In:** SVAR Gantt, DHTMLX Gantt

**Description:** Define custom time periods beyond standard calendars:
- **Sprints** - 2-week iterations with custom start dates
- **Quarters** - Fiscal quarters (Q1 = Apr-Jun, not Jan-Mar)
- **Custom periods** - User-defined milestones, phases, releases
- **Highlight bands** - Visual markers for period boundaries
- **Period headers** - Show sprint/phase names in timeline header

**Why It Matters:**
- Essential for Agile/Scrum teams (most common workflow)
- Financial planning uses fiscal calendars, not calendar years
- Project phases don't align with months
- Enables "Planning Poker" style sprint planning

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 7 (Advanced Features)

**Specification Impact:** Add to FR-2 Timeline Visualization
```rust
#[prop(optional)]
custom_periods: Option<Vec<CustomPeriod>>,

pub struct CustomPeriod {
    pub id: String,
    pub name: String,          // "Sprint 23", "Q2 FY2026"
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub color: Option<String>, // Highlight band color
}
```

---

### GAP-06: Split Tasks (Pause/Resume)

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Tasks that have gaps/interruptions:
- Split task into multiple segments
- Visual representation: multiple bars with gaps
- Common scenarios:
  - Task paused for resource availability
  - Part-time work (Mon-Wed only)
  - Interrupted by higher-priority work
  - Seasonal work (summer/winter split)

**Why It Matters:**
- Reflects reality of project work (interruptions are common)
- Accurate resource planning (not working full-time on task)
- Better schedule forecasting
- Differentiates from simple multi-phase tasks

**Implementation Complexity:** High

**Suggested Phase:** Phase 10+ (Future Enhancement)

**Specification Impact:** Add to data model
```rust
pub struct GanttTask {
    // ... existing fields
    pub segments: Vec<TaskSegment>, // NEW
}

pub struct TaskSegment {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
```

---

### GAP-07: Unscheduled Tasks (Backlog)

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Tasks without dates (backlog items):
- Tasks exist but not yet scheduled
- Displayed in separate "Unscheduled" section
- Drag from backlog to timeline to schedule
- Use cases:
  - Product backlog in Agile
  - Idea list / future tasks
  - Contingency tasks

**Why It Matters:**
- Critical for Agile workflows (backlog grooming)
- Prevents cluttering timeline with "someday" tasks
- Enables drag-to-schedule workflow
- Common in JIRA, Monday.com, Asana

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 7 (Advanced Features)

**Specification Impact:** Add to data model and UI
```rust
pub struct GanttTask {
    // ... existing fields
    pub start: Option<NaiveDateTime>, // Make optional
    pub end: Option<NaiveDateTime>,   // Make optional
}

// Add to component props
#[prop(optional, default = true)]
show_unscheduled_section: bool,
```

---

### GAP-08: Work Calendar Configuration

**Found In:** DHTMLX Gantt

**Description:** Sophisticated scheduling calendar:
- **Working hours** - 9am-5pm, or custom per day
- **Non-working days** - Holidays, company events (already planned)
- **Resource-specific calendars** - Different teams have different schedules
- **Shift work** - 24/7 operations with rotating shifts
- **Duration types:**
  - Calendar days (includes weekends)
  - Working days (excludes weekends/holidays)
  - Working hours (e.g., 16 hours = 2 working days)

**Why It Matters:**
- Accurate duration calculations
- Realistic deadline estimates
- Critical for manufacturing/shift work
- Prevents scheduling work on holidays

**Implementation Complexity:** High

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Major addition to data model
```rust
pub struct WorkCalendar {
    pub working_hours: HashMap<Weekday, Vec<TimeRange>>,
    pub holidays: Vec<NaiveDate>,
    pub exceptions: Vec<CalendarException>, // Special days
}

pub enum DurationType {
    CalendarDays,
    WorkingDays,
    WorkingHours,
}
```

---

### GAP-09: Backward Planning (Deadline-Driven)

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Schedule tasks backward from a fixed deadline:
- Set project end date (deadline)
- Calculate start dates working backward
- Respect dependencies in reverse (predecessor starts when successor ends)
- Use cases:
  - Event planning (wedding, conference)
  - Product launch with fixed date
  - Regulatory deadlines

**Why It Matters:**
- Real projects often have fixed deadlines
- More realistic than "start today, when do we finish?"
- Identifies latest possible start dates
- Critical path becomes "longest path from end"

**Implementation Complexity:** High

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Add to component props
```rust
#[prop(optional)]
planning_mode: Option<PlanningMode>,

pub enum PlanningMode {
    Forward,    // Schedule from start date (default)
    Backward,   // Schedule from end date
}
```

---

### GAP-10: Expected Progress Line

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Visual indicator of schedule health:
- Diagonal line across timeline showing expected progress today
- Compare actual progress to expected
- Color coding:
  - Green: Ahead of schedule
  - Yellow: On track
  - Red: Behind schedule
- Also called "Progress Line" or "Status Line"

**Why It Matters:**
- At-a-glance project health assessment
- Identifies at-risk tasks immediately
- Common in earned value management
- Answers "Are we on track?" instantly

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 7 (Advanced Features)

**Specification Impact:** Add to VIS-5 Visual Indicators
```rust
#[prop(optional, default = false)]
show_expected_progress: bool,
```

---

### GAP-11: Vertical Timeline Markers

**Found In:** SVAR Gantt, DHTMLX Gantt

**Description:** Custom vertical lines on timeline:
- Today marker (already specified)
- **NEW: Release dates** - Major milestones
- **NEW: Sprint boundaries** - Start/end of iterations
- **NEW: Custom events** - Holidays, team events, deadlines
- Configurable color, style, label

**Why It Matters:**
- Contextualizes timeline with important dates
- Helps align tasks to external events
- Visual reference points for planning
- Common in all major PM tools

**Implementation Complexity:** Low

**Suggested Phase:** Phase 7 (Advanced Features)

**Specification Impact:** Add to API
```rust
#[prop(optional)]
timeline_markers: Option<Vec<TimelineMarker>>,

pub struct TimelineMarker {
    pub date: NaiveDate,
    pub label: String,
    pub color: String,
    pub style: MarkerStyle, // Solid, Dashed, Dotted
}
```

---

### GAP-12: Today Navigation Button

**Found In:** All libraries (Frappe, SVAR, DHTMLX)

**Description:** Quick navigation to current date:
- "Today" button in toolbar
- Scrolls timeline to show today marker
- Highlights today's column briefly
- Keyboard shortcut: T key

**Why It Matters:**
- Users get lost in large timelines
- Quick orientation after zooming/panning
- Standard UX pattern (calendars, maps)
- Trivial to implement, high user value

**Implementation Complexity:** Low

**Suggested Phase:** Phase 4 (Basic Interactivity)

**Specification Impact:** Add to INT-5 Zoom and Pan controls

---

## P2 Priority Gaps (Medium)

### GAP-13: Infinite Scroll Padding

**Found In:** SVAR Gantt

**Description:** Automatically extend timeline as user scrolls:
- Timeline initially shows project date range
- Scrolling past end adds blank space for new tasks
- Prevents "trapped" feeling (can't scroll past last task)
- Configurable padding amount (e.g., +3 months)

**Why It Matters:**
- Enables adding tasks beyond current range
- Smoother UX than manual "extend timeline" button
- Encourages forward planning
- Modern infinite-scroll pattern

**Implementation Complexity:** Low

**Suggested Phase:** Phase 5 (Drag and Drop)

**Specification Impact:** Add to component props
```rust
#[prop(optional, default = true)]
infinite_scroll: bool,

#[prop(optional, default = Duration::weeks(4))]
scroll_padding: Duration,
```

---

### GAP-14: Custom HTML in Grid Cells

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Render arbitrary HTML in task list columns:
- Custom formatters for columns
- Rich content: badges, icons, progress bars
- Inline buttons (edit, delete)
- Use cases:
  - Status badges with icons
  - Avatar images for assignees
  - Priority flags
  - Action buttons

**Why It Matters:**
- Professional, polished look
- Information density (show more without cluttering)
- Custom branding
- Standard in modern data grids

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 7 (Advanced Features)

**Specification Impact:** Add to GanttColumn
```rust
pub struct GanttColumn {
    // ... existing fields
    pub renderer: Option<Box<dyn Fn(&GanttTask) -> View>>, // Custom cell renderer
}
```

---

### GAP-15: Taskbar Tooltips

**Found In:** All libraries

**Description:** Hover tooltips on task bars:
- Show full task details without opening dialog
- Customizable content
- Default info: name, dates, progress, assignees
- Appears on hover with delay

**Why It Matters:**
- Essential for long task names (truncated in bar)
- Shows metadata without cluttering chart
- Standard UX pattern
- Aids accessibility (supplemental info)

**Implementation Complexity:** Low

**Suggested Phase:** Phase 6 (Dependencies)

**Specification Impact:** Add to component props
```rust
#[prop(optional)]
tooltip_template: Option<Box<dyn Fn(&GanttTask) -> String>>,
```

---

### GAP-16: Task Reordering in Grid via Drag

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Drag tasks within grid to reorder:
- Drag task rows up/down
- Change task sequence
- Re-parent tasks (drag to new parent)
- Visual drop indicators

**Currently Specified:** INT-1 mentions "Task List Reordering" but limited detail

**Enhancement Needed:**
- Specify drag handle in grid (not just task bar)
- Allow drag between different parent tasks
- Preserve or update dependencies on reorder

**Why It Matters:**
- Intuitive task organization
- Faster than "Move Up/Down" buttons
- Standard in modern UIs (Trello, Asana)

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 5 (Drag and Drop) - expand scope

**Specification Impact:** Clarify in INT-1 that grid reordering is distinct from timeline dragging

---

### GAP-17: Minute-Level Time Granularity

**Found In:** DHTMLX Gantt

**Description:** Support minute/hour precision for short-duration tasks:
- Timeline scale: Minutes, Hours (already has Hour)
- Task duration in minutes (not just days)
- Use cases:
  - Manufacturing processes (30-minute task)
  - Meeting schedules
  - Maintenance windows
  - Surgery schedules

**Currently Specified:** ViewMode has Hour, but unclear if tasks support minute-level precision

**Why It Matters:**
- Some industries need sub-day precision
- Enables Gantt for non-project use cases
- Calendar/scheduling hybrid

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Clarify in FR-2 that Hour mode supports minute precision
```rust
pub enum ViewMode {
    Minute,  // NEW: 5-min, 15-min, 30-min subdivisions
    Hour,    // Existing
    // ... rest
}
```

---

### GAP-18: Custom Dependency Link Styling

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Style dependency links beyond color:
- Line thickness
- Line style (solid, dashed, dotted)
- Arrow size and style
- Per-dependency styling (e.g., critical path uses thick red line)
- Lag time indicator (show +5d on link)

**Currently Specified:** VIS-2 has basic styling, but limited customization

**Why It Matters:**
- Visual hierarchy (important vs. minor dependencies)
- Show lag/lead time visually
- Critical path emphasis
- Custom color coding by dependency type

**Implementation Complexity:** Low

**Suggested Phase:** Phase 6 (Dependencies)

**Specification Impact:** Add to API
```rust
pub struct TaskDependency {
    // ... existing fields
    pub style: Option<DependencyStyle>,
}

pub struct DependencyStyle {
    pub color: String,
    pub thickness: u8,
    pub line_style: LineStyle,
}
```

---

### GAP-19: Multiple Theme Skins

**Found In:** DHTMLX Gantt (7+ themes), SVAR Gantt

**Description:** Pre-built visual themes beyond light/dark:
- Material Design theme
- Terrace theme (office-style)
- Contrast theme (accessibility)
- Minimalist theme
- Corporate themes (match brand colors)

**Currently Specified:** VIS-1 mentions daisyUI theming, but assumes single custom theme

**Why It Matters:**
- Professional appearance options
- Match corporate branding
- User preference (some prefer dense, some prefer spacious)
- Accessibility (high contrast themes)

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 10 (Polish & Documentation)

**Specification Impact:** Add theme preset system
```rust
#[prop(optional)]
theme_preset: Option<GanttThemePreset>,

pub enum GanttThemePreset {
    Material,
    Minimalist,
    Contrast,
    Custom(GanttTheme),
}
```

---

### GAP-20: iCal / MS Project / Excel Export

**Found In:** DHTMLX Gantt

**Description:** Export to industry-standard formats:

**Currently Specified:** JSON, CSV, PNG exports

**Missing Formats:**
- **iCal (.ics)** - Calendar import (Apple Calendar, Google Calendar)
- **MS Project (.mpp or .xml)** - Microsoft Project import
- **Excel (.xlsx)** - Rich formatting, formulas
- **PDF** - Print-ready reports

**Why It Matters:**
- Interoperability with existing tools
- Stakeholder reporting (PDF)
- Integration with calendars
- Data portability

**Implementation Complexity:** High (requires format libraries)

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Add to public methods
```rust
impl GanttChart {
    pub fn export_to_ical(&self) -> String;
    pub fn export_to_ms_project(&self) -> Vec<u8>;
    pub fn export_to_excel(&self) -> Vec<u8>;
    pub fn export_to_pdf(&self) -> Vec<u8>;
}
```

---

### GAP-21: Drag to Create Dependency (Enhanced)

**Found In:** SVAR Gantt, DHTMLX Gantt

**Description:** Visual dependency creation workflow:

**Currently Specified:** "Drag from task connector to create dependency link"

**Missing Details:**
- **Connection points** - Show 4 drag handles on task (start/end, top/bottom)
- **Dependency type selection** - Choose FS/SS/FF/SF during drag
- **Visual preview** - Live link preview while dragging
- **Smart suggestions** - Suggest dependency type based on drag direction
- **Lag time input** - Set lag immediately after creating link

**Why It Matters:**
- More intuitive than right-click menu
- Visual feedback reduces errors
- Fast workflow for power users

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 6 (Dependencies) - expand scope

**Specification Impact:** Add detailed UX flow to INT-1

---

### GAP-22: Column Header Sorting Indicators

**Found In:** All libraries

**Description:** Visual sort state in grid columns:
- Up/down arrow in column header
- Indicate sort direction (ascending/descending)
- Multi-column sort indicators (1, 2, 3)
- Click header to toggle sort

**Currently Specified:** INT-7 mentions sorting, but no UI details

**Why It Matters:**
- Users need to know current sort state
- Standard data grid UX
- Accessibility (announce sort state)

**Implementation Complexity:** Low

**Suggested Phase:** Phase 4 (Basic Interactivity)

**Specification Impact:** Add to VIS-4 Task List Panel column layout

---

### GAP-23: Resource Utilization View

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Separate view showing resource workload:
- Timeline by person (not by task)
- Show all tasks for each resource
- Detect overallocation (100%+ capacity)
- Color code by utilization:
  - Green: 0-80% (healthy)
  - Yellow: 80-100% (full)
  - Red: 100%+ (overallocated)

**Currently Specified:** FR-7 mentions "Resource utilization view (separate feature)" but not detailed

**Why It Matters:**
- Critical for resource planning
- Prevents burnout (identify overwork)
- Balances team workload
- Standard in enterprise PM tools

**Implementation Complexity:** High

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Expand FR-7 or create separate ResourceUtilization component

---

## P3 Priority Gaps (Low)

### GAP-24: Auto-Scheduling Modes

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Advanced auto-scheduling algorithms:

**Currently Specified:** `auto_schedule` boolean prop (simple)

**Missing Advanced Modes:**
- **ASAP (As Soon As Possible)** - Start tasks earliest possible date
- **ALAP (As Late As Possible)** - Start tasks latest possible date
- **Manual override** - Lock specific tasks, schedule around them
- **Resource leveling** - Delay tasks to avoid overallocation
- **Constraint types:**
  - Must Start On (MSO)
  - Must Finish On (MFO)
  - Start No Earlier Than (SNET)
  - Finish No Later Than (FNLT)

**Why It Matters:**
- Sophisticated project scheduling
- Matches Microsoft Project functionality
- Critical for complex dependencies
- Enterprise feature

**Implementation Complexity:** Very High

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Major addition to scheduling engine

---

### GAP-25: Undo/Redo with History Panel

**Found In:** DHTMLX Gantt

**Description:** Visual undo history:

**Currently Specified:** Ctrl+Z/Ctrl+Y keyboard shortcuts

**Missing Visual Feature:**
- History panel showing all actions
- Click to jump to any point in history
- Action descriptions ("Moved Task A", "Added dependency")
- Branching history (undo to point, then make new changes)

**Why It Matters:**
- Advanced users want to see what they're undoing
- Selective undo (revert specific change)
- Debugging aid (what changed?)

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Add optional history panel UI component

---

### GAP-26: Task Templates

**Found In:** DHTMLX Gantt

**Description:** Pre-configured task templates:
- Save tasks as reusable templates
- Template library (Marketing campaign, Software sprint, etc.)
- Drag template to timeline to instantiate
- Update all tasks from template

**Why It Matters:**
- Speeds up project creation
- Ensures consistency
- Best practice sharing
- Common in enterprise tools

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Add template management system

---

### GAP-27: Baseline Comparison (Enhanced)

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Advanced baseline features:

**Currently Specified:** FR-8 mentions baselines (P3 priority)

**Missing Advanced Features:**
- **Multiple baselines** - Track 3+ snapshots (e.g., original, approved, current)
- **Variance indicators** - Show +/- days from baseline
- **Variance reports** - Summary of all changes
- **Baseline overlay** - Show baseline bars as ghost outlines
- **Automatic snapshots** - Save baseline on milestone completion

**Why It Matters:**
- Change management and accountability
- Executive reporting
- Earned value management
- Contract compliance

**Implementation Complexity:** High

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Expand FR-8 with detailed baseline system

---

### GAP-28: Drag to Select (Rubber Band)

**Found In:** SVAR Gantt, DHTMLX Gantt

**Description:** Click and drag to select multiple tasks:

**Currently Specified:** Ctrl+Click, Shift+Click mentioned in INT-4

**Missing:** "Drag to select (rubber band)" is mentioned but not detailed

**Enhancement Needed:**
- Specify rubber band selection area styling
- Select tasks in both grid and timeline
- Partial overlap behavior (touch vs. fully inside)

**Why It Matters:**
- Standard in all modern UIs
- Faster than individual clicks
- Essential for bulk operations

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 4 (Basic Interactivity)

**Specification Impact:** Add detailed UX spec to INT-4

---

### GAP-29: Task Notes / Comments

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Attach notes to tasks:
- Text notes on tasks
- Comment threads (multi-user)
- Show comment indicator icon on task bar
- Comment history / audit trail

**Why It Matters:**
- Collaboration (team discussions)
- Context preservation (why was this delayed?)
- Audit trail for compliance
- Standard in PM tools (Asana, Jira)

**Implementation Complexity:** Medium

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Add to data model
```rust
pub struct GanttTask {
    // ... existing fields
    pub notes: Vec<TaskNote>,
}

pub struct TaskNote {
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub content: String,
}
```

---

### GAP-30: Print-Friendly View

**Found In:** DHTMLX Gantt

**Description:** Optimized layout for printing:
- Fit timeline to page width
- Page breaks between task groups
- Remove interactive controls
- High-contrast colors
- Print CSS stylesheet

**Why It Matters:**
- Stakeholder presentations
- Hard-copy reports
- Regulatory documentation
- Meeting handouts

**Implementation Complexity:** Low

**Suggested Phase:** Phase 10 (Polish & Documentation)

**Specification Impact:** Add to export functionality

---

### GAP-31: Swimlanes / Task Grouping

**Found In:** DHTMLX Gantt, SVAR Gantt

**Description:** Horizontal lanes for task grouping:
- Group by assignee, department, priority, etc.
- Visual separation between groups
- Collapse/expand groups
- Different from parent-child (flat grouping)

**Why It Matters:**
- Organize large projects
- Team-based views
- Kanban-style visualization
- Reduce scrolling

**Implementation Complexity:** High

**Suggested Phase:** Phase 11+ (Future Enhancement)

**Specification Impact:** Major UI paradigm addition

---

## Implementation Roadmap

### Recommended Integration into Existing Phases

#### Phase 4 (Basic Interactivity) - Add 3 gaps
- **GAP-01:** Configurable Read-Only Modes (Low complexity)
- **GAP-12:** Today Navigation Button (Low complexity)
- **GAP-22:** Column Header Sorting Indicators (Low complexity)

**Rationale:** All are low-complexity, high-value features that naturally fit with selection/navigation work.

---

#### Phase 5 (Drag and Drop) - Add 2 gaps
- **GAP-13:** Infinite Scroll Padding (Low complexity)
- **GAP-16:** Task Reordering in Grid via Drag (expand scope)

**Rationale:** Both are drag-related features, implement while drag infrastructure is fresh.

---

#### Phase 6 (Dependencies) - Add 2 gaps
- **GAP-15:** Taskbar Tooltips (Low complexity)
- **GAP-18:** Custom Dependency Link Styling (Low complexity)
- **GAP-21:** Drag to Create Dependency - Enhanced (expand scope)

**Rationale:** Polish dependency visualization with tooltips and styling.

---

#### Phase 7 (Advanced Features) - Add 7 gaps
- **GAP-03:** Localization (i18n) Support (Medium complexity)
- **GAP-05:** Custom Time Periods / Sprint Support (Medium complexity)
- **GAP-07:** Unscheduled Tasks (Backlog) (Medium complexity)
- **GAP-10:** Expected Progress Line (Medium complexity)
- **GAP-11:** Vertical Timeline Markers (Low complexity)
- **GAP-14:** Custom HTML in Grid Cells (Medium complexity)
- **GAP-19:** Multiple Theme Skins (Medium complexity)

**Rationale:** This phase already focuses on advanced features; these gaps fit naturally.

---

#### Phase 8 (Performance Optimization) - Add 1 gap
- **GAP-02:** Smart Rendering Optimizations (expand scope)

**Rationale:** Enhance planned virtual scrolling with progressive loading and LOD rendering.

---

#### Phase 9 (Accessibility & Testing) - Add 1 gap
- **GAP-04:** Comprehensive Keyboard Shortcuts (expand scope)

**Rationale:** Accessibility phase should include complete keyboard navigation.

---

#### Phase 10 (Polish & Documentation) - Add 1 gap
- **GAP-30:** Print-Friendly View (Low complexity)

**Rationale:** Final polish includes export/print features.

---

#### Phase 11+ (Future Enhancements) - Remaining 13 gaps
- **GAP-06:** Split Tasks (High complexity)
- **GAP-08:** Work Calendar Configuration (High complexity)
- **GAP-09:** Backward Planning (High complexity)
- **GAP-17:** Minute-Level Time Granularity (Medium complexity)
- **GAP-20:** iCal / MS Project / Excel Export (High complexity)
- **GAP-23:** Resource Utilization View (High complexity)
- **GAP-24:** Auto-Scheduling Modes (Very High complexity)
- **GAP-25:** Undo/Redo with History Panel (Medium complexity)
- **GAP-26:** Task Templates (Medium complexity)
- **GAP-27:** Baseline Comparison - Enhanced (High complexity)
- **GAP-28:** Drag to Select (Rubber Band) - Enhanced (Medium complexity)
- **GAP-29:** Task Notes / Comments (Medium complexity)
- **GAP-31:** Swimlanes / Task Grouping (High complexity)

**Rationale:** These are complex features or niche use cases best suited for post-MVP releases.

---

## Comparison Matrix

### Feature Presence by Library

| Feature | Frappe | SVAR | DHTMLX | Priority |
|---------|--------|------|--------|----------|
| Configurable Read-Only Modes | ❌ | ✅ | ✅ | P0 |
| Smart Rendering (10k+ tasks) | ❌ | ✅ | ✅ | P0 |
| Localization (i18n) | ❌ | ✅ | ✅ | P0 |
| Comprehensive Keyboard Shortcuts | ❌ | ✅ | ✅ | P0 |
| Custom Time Periods (Sprints) | ❌ | ✅ | ✅ | P1 |
| Split Tasks | ❌ | ✅ | ✅ | P1 |
| Unscheduled Tasks (Backlog) | ❌ | ✅ | ✅ | P1 |
| Work Calendar Configuration | ❌ | ❌ | ✅ | P1 |
| Backward Planning | ❌ | ✅ | ✅ | P1 |
| Expected Progress Line | ❌ | ✅ | ✅ | P1 |
| Vertical Timeline Markers | ✅ | ✅ | ✅ | P1 |
| Today Navigation Button | ✅ | ✅ | ✅ | P1 |
| Infinite Scroll Padding | ❌ | ✅ | ❌ | P2 |
| Custom HTML in Grid Cells | ❌ | ✅ | ✅ | P2 |
| Taskbar Tooltips | ✅ | ✅ | ✅ | P2 |
| Task Reordering in Grid | ❌ | ✅ | ✅ | P2 |
| Minute-Level Granularity | ❌ | ❌ | ✅ | P2 |
| Custom Dependency Styling | ❌ | ✅ | ✅ | P2 |
| Multiple Theme Skins | ❌ | ✅ | ✅ | P2 |
| iCal/MS Project/Excel Export | ❌ | ❌ | ✅ | P2 |
| Enhanced Dependency Creation | ❌ | ✅ | ✅ | P2 |
| Column Header Sort Indicators | ✅ | ✅ | ✅ | P2 |
| Resource Utilization View | ❌ | ✅ | ✅ | P2 |
| Auto-Scheduling Modes (ASAP/ALAP) | ❌ | ❌ | ✅ | P3 |
| Undo History Panel | ❌ | ❌ | ✅ | P3 |
| Task Templates | ❌ | ❌ | ✅ | P3 |
| Enhanced Baseline Comparison | ❌ | ✅ | ✅ | P3 |
| Rubber Band Selection | ✅ | ✅ | ✅ | P3 |
| Task Notes/Comments | ❌ | ✅ | ✅ | P3 |
| Print-Friendly View | ❌ | ❌ | ✅ | P3 |
| Swimlanes/Grouping | ❌ | ✅ | ✅ | P3 |

### Summary Statistics

| Library | P0 Features | P1 Features | P2 Features | P3 Features | Total |
|---------|-------------|-------------|-------------|-------------|-------|
| **Frappe Gantt** | 0/4 (0%) | 2/8 (25%) | 3/11 (27%) | 1/8 (13%) | 6/31 (19%) |
| **SVAR Gantt** | 3/4 (75%) | 7/8 (88%) | 8/11 (73%) | 5/8 (63%) | 23/31 (74%) |
| **DHTMLX Gantt** | 3/4 (75%) | 7/8 (88%) | 9/11 (82%) | 7/8 (88%) | 26/31 (84%) |

**Key Insight:** DHTMLX Gantt leads with 84% of identified gap features, followed closely by SVAR Gantt (74%). Frappe Gantt is significantly simpler (19%), focusing on core functionality.

---

## Priority Distribution

### By Implementation Complexity

| Complexity | P0 | P1 | P2 | P3 | Total |
|------------|----|----|----|----|-------|
| **Low** | 1 | 2 | 4 | 1 | 8 (26%) |
| **Medium** | 2 | 3 | 5 | 4 | 14 (45%) |
| **High** | 1 | 3 | 2 | 3 | 9 (29%) |

**Key Insight:** 45% of gaps are medium complexity, achievable with focused effort. Only 29% are high-complexity, suitable for later phases.

---

### By Business Value vs. Effort

#### Quick Wins (High Value, Low Effort)
1. **GAP-12:** Today Navigation Button (P1, Low)
2. **GAP-11:** Vertical Timeline Markers (P1, Low)
3. **GAP-15:** Taskbar Tooltips (P2, Low)
4. **GAP-22:** Column Header Sorting (P2, Low)
5. **GAP-13:** Infinite Scroll Padding (P2, Low)

#### Strategic Investments (High Value, High Effort)
1. **GAP-08:** Work Calendar Configuration (P1, High)
2. **GAP-09:** Backward Planning (P1, High)
3. **GAP-02:** Smart Rendering (P0, High)

#### Nice-to-Haves (Lower Priority, Easy to Add)
1. **GAP-01:** Read-Only Modes (P0, Low) - Actually critical for security!
2. **GAP-30:** Print View (P3, Low)

---

## Recommendations

### MVP (Phases 1-10)
**Add 17 gaps** to existing phases:
- 4 P0 gaps (all critical)
- 8 P1 gaps (high value)
- 5 P2 gaps (polish features)

**Estimated Timeline:** Adds ~3-4 weeks to 20-week plan (15% increase)

**Value Proposition:** Brings component to parity with SVAR Gantt (74% feature coverage) and improves competitiveness significantly.

---

### Post-MVP (Phase 11+)
**Defer 13 gaps** to future releases:
- Enterprise features (auto-scheduling modes, work calendars)
- Niche use cases (split tasks, backward planning)
- Advanced integrations (MS Project export)

**Rationale:** These features have high complexity and lower user demand. Better suited for v2.0 release.

---

### Critical Must-Haves for Phase 4-7
1. **GAP-01:** Read-Only Modes (security requirement)
2. **GAP-03:** Localization (international users)
3. **GAP-04:** Comprehensive Keyboard Shortcuts (accessibility)
4. **GAP-05:** Sprint Support (Agile teams - huge market)
5. **GAP-07:** Unscheduled Tasks (Agile backlog)

**Rationale:** These gaps address fundamental use cases (Agile workflows, international teams, accessibility) that are table stakes for modern PM tools.

---

## Next Steps

1. **Review & Prioritize:** Validate gap priorities with stakeholders
2. **Update Specification:** Incorporate approved gaps into `Gantt_Component.md`
3. **Revise Timeline:** Adjust development plan for added scope
4. **Create Issues:** Track each gap as separate GitHub issue
5. **Research Implementations:** Study DHTMLX/SVAR code for complex gaps

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-30 | Initial gap analysis with 31 identified features |

---

## Appendix: Gap Sources

### Research Sources
- **Frappe Gantt:** https://github.com/frappe/gantt (simple, minimalist approach)
- **SVAR Gantt:** https://svar.dev/react/gantt/ (modern, TypeScript, performance-focused)
- **DHTMLX Gantt:** https://docs.dhtmlx.com/gantt/ (enterprise, feature-rich, GPL)

### Feature Discovery Method
1. Analyzed documentation for all three libraries
2. Compared against existing `Gantt_Component.md` specification
3. Categorized gaps by:
   - Business value (P0-P3)
   - Implementation complexity (Low/Medium/High)
   - Library presence (which library has it)
   - Development phase fit (where it should go)

### Validation
- Cross-referenced with industry standards (Microsoft Project, Asana, Monday.com)
- Consulted WCAG 2.2 accessibility guidelines
- Reviewed user feedback from library GitHub issues
