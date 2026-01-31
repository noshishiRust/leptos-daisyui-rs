# Kanban Board Component - Requirements & Development Plan

## Overview

A production-ready Kanban board component for leptos-daisyui-rs that provides drag-and-drop task management with advanced filtering, search, and customization capabilities.

## Functional Requirements

### FR-1: Core Kanban Structure

#### FR-1.1: Board
- **Description**: Container for all kanban columns and cards
- **Properties**:
  - `board_id`: Unique identifier for the board
  - `title`: Board title (optional)
  - `columns`: Vector of column definitions
  - `on_card_move`: Callback when card is moved between columns or reordered
  - `on_card_click`: Callback when card is clicked
  - `on_card_create`: Callback when new card is created
  - `on_card_delete`: Callback when card is deleted
  - `max_height`: Optional maximum height for the board container

#### FR-1.2: Columns
- **Description**: Vertical swim lanes that contain cards
- **Properties**:
  - `column_id`: Unique identifier
  - `title`: Column header text
  - `color`: Optional theme color for column header
  - `card_limit`: Optional WIP (Work In Progress) limit
  - `cards`: Vector of cards in this column
  - `collapsed`: Boolean state for column collapse
  - `collapsible`: Whether column can be collapsed
  - `scrollable`: Whether column has independent scroll
  - `on_collapse`: Callback when column is collapsed/expanded
- **Features**:
  - Display card count in header
  - Display WIP limit warning when exceeded
  - Add new card button in header or footer
  - Drag-and-drop zone for receiving cards

#### FR-1.3: Cards
- **Description**: Individual task/item cards within columns
- **Properties**:
  - `card_id`: Unique identifier
  - `title`: Card title (required)
  - `description`: Card description (optional)
  - `labels`: Vector of label tags
  - `assignees`: Vector of assignee information
  - `priority`: Priority level (Low, Medium, High, Critical)
  - `due_date`: Optional due date
  - `metadata`: HashMap for custom fields
- **Features**:
  - Draggable within same column (reorder)
  - Draggable between columns (move)
  - Click to view details
  - Visual indicators for priority, overdue, etc.

### FR-2: Drag and Drop

#### FR-2.1: Card Dragging
- Cards can be dragged within the same column to reorder
- Cards can be dragged to different columns to change status
- Visual feedback during drag:
  - Dragged card becomes semi-transparent
  - Drop zones highlight when card hovers over them
  - Invalid drop zones show disabled state
- Smooth animations for card movement

#### FR-2.2: Drop Validation
- Respect WIP limits (optional warning or blocking)
- Custom validation callback to allow/deny drops
- Undo support for last move operation

### FR-3: Column Collapse

#### FR-3.1: Collapse Behavior
- Click column header or collapse icon to toggle
- Collapsed column shows:
  - Vertical title text
  - Card count badge
  - Minimal width (e.g., 48px)
- Expanded column shows full content
- Persist collapse state in local storage or via callback

#### FR-3.2: Collapse Animation
- Smooth width transition (300ms ease-in-out)
- Cards fade out when collapsing
- Cards fade in when expanding

### FR-4: Independent Column Scrolling

#### FR-4.1: Scroll Container
- Each column has independent scroll container
- Column header remains fixed at top
- Scrollbar appears only when content overflows
- Configurable max height per column

#### FR-4.2: Scroll Behavior
- Smooth scrolling
- Auto-scroll when dragging card near top/bottom of column
- Scroll position maintained when cards added/removed

### FR-5: Quick Filters

#### FR-5.1: Filter Bar
- Positioned above the board
- Contains multiple filter dropdowns:
  - **Assignee Filter**: Multi-select dropdown of all assignees
  - **Label Filter**: Multi-select dropdown of all labels
  - **Priority Filter**: Multi-select (Low, Medium, High, Critical)
  - **Due Date Filter**: Options (Overdue, Due Today, Due This Week, No Due Date)
- "Clear All Filters" button
- Active filter count badge

#### FR-5.2: Filter Logic
- Filters are AND-ed together (all must match)
- Within a filter category, selections are OR-ed
- Filtered cards:
  - Matching cards remain visible with normal opacity
  - Non-matching cards are hidden or shown with low opacity
- Real-time filtering (no apply button needed)

#### FR-5.3: Filter State
- Persist filter selections in URL query parameters
- Share-able filtered board URLs
- Save filter presets (optional advanced feature)

### FR-6: Search Functionality

#### FR-6.1: Search Bar
- Text input positioned in filter bar
- Placeholder: "Search cards..."
- Search icon prefix
- Clear button (X) when text entered

#### FR-6.2: Search Behavior
- Real-time search (debounced by 300ms)
- Case-insensitive matching
- Search across:
  - Card titles
  - Card descriptions
  - Label names
  - Assignee names
- Highlight matching text in cards (optional)

#### FR-6.3: Search + Filter Combination
- Search works in conjunction with filters
- Both must match for card to be visible
- Clear button clears only search, not filters

### FR-7: Additional Features

#### FR-7.1: Card Creation
- "Add Card" button in each column header or footer
- Inline creation:
  - Click button → Input field appears
  - Type title and press Enter to create
  - Press Escape to cancel
- Or modal creation for more details

#### FR-7.2: Card Actions
- Click card to open detail modal/panel
- Quick actions menu (three-dot icon):
  - Edit
  - Delete
  - Move to column (submenu)
  - Duplicate
- Keyboard shortcuts for actions

#### FR-7.3: Empty States
- Empty column message: "Drop cards here" or custom text
- Empty search/filter results: "No cards match your criteria"

#### FR-7.4: Loading States
- Skeleton cards while loading
- Loading spinner for async operations

---

## Non-Functional Requirements

### NFR-1: Performance

#### NFR-1.1: Rendering Performance
- Support up to 500 cards across all columns without lag
- Virtualize card lists if column has >50 cards
- Drag operations maintain 60fps
- Debounce search input (300ms)
- Throttle scroll events (16ms)

#### NFR-1.2: Memory Management
- Efficient signal usage in Leptos
- Cleanup event listeners on component unmount
- Lazy load card details/images

### NFR-2: Accessibility

#### NFR-2.1: Keyboard Navigation
- Tab through cards and columns
- Enter to select/activate
- Space to drag (keyboard drag-and-drop)
- Arrow keys to navigate between cards
- Escape to cancel drag or close modals

#### NFR-2.2: Screen Reader Support
- Proper ARIA labels for all interactive elements
- ARIA live regions announce card moves
- Column collapse state announced
- Filter changes announced

#### NFR-2.3: Focus Management
- Visible focus indicators (daisyUI focus ring)
- Focus trap in modals
- Return focus after card move

### NFR-3: Responsiveness

#### NFR-3.1: Desktop (≥1024px)
- Full horizontal layout with all columns visible
- Drag-and-drop fully supported
- Min column width: 300px

#### NFR-3.2: Tablet (768px - 1023px)
- Horizontal scroll for columns
- Reduced column width: 280px
- Drag-and-drop supported

#### NFR-3.3: Mobile (< 768px)
- Single column view with column selector dropdown
- Touch-friendly drag-and-drop
- Swipe gestures for navigation
- Simplified filters (bottom sheet)

### NFR-4: Browser Compatibility
- Modern browsers with ES2020+ support
- Chrome 90+, Firefox 88+, Safari 14+, Edge 90+
- WebAssembly support required

### NFR-5: Theming
- Fully compatible with daisyUI themes
- Respect user's theme selection
- Support all 32 daisyUI built-in themes
- Custom color overrides via CSS variables

---

## Interactivity Requirements

### INT-1: Drag and Drop Interactions

#### INT-1.1: Mouse Interactions
- **Click and hold** card (200ms) to initiate drag
- **Drag** card with cursor following
- **Hover** over drop zone highlights it
- **Release** to drop card
- **Cancel**: Press Escape or drag outside board

#### INT-1.2: Touch Interactions
- **Long press** card (500ms) to initiate drag
- **Drag** with finger, card follows touch point
- **Haptic feedback** on drag start (mobile)
- **Auto-scroll** when dragging near edge
- **Release** to drop

#### INT-1.3: Keyboard Interactions
- **Tab** to focus card
- **Space** to pick up card
- **Arrow keys** to move card to different position
- **Enter** to drop card
- **Escape** to cancel

### INT-2: Column Interactions

#### INT-2.1: Collapse/Expand
- **Click** column header or collapse icon
- **Animation**: 300ms width transition
- **Keyboard**: Enter/Space when header focused
- **Persist**: Save state to localStorage or callback

#### INT-2.2: Scrolling
- **Mouse wheel** scrolls active column
- **Drag near edge** auto-scrolls during card drag
- **Scroll bar** visible on hover (desktop)
- **Touch scroll** (mobile/tablet)

### INT-3: Filter Interactions

#### INT-3.1: Dropdown Filters
- **Click** to open dropdown
- **Multi-select** checkboxes for selections
- **Click outside** to close
- **Apply immediately** on selection change
- **Badge** shows active filter count

#### INT-3.2: Search
- **Type** to search (debounced)
- **Clear button** (X icon) clears search
- **Escape** key clears search
- **Enter** focuses first result card

### INT-4: Card Interactions

#### INT-4.1: Selection
- **Click** card to open details
- **Hover** shows quick action menu
- **Right-click** shows context menu

#### INT-4.2: Quick Actions
- **Three-dot menu** on hover
- **Edit**: Opens edit modal
- **Delete**: Shows confirmation
- **Move**: Submenu with column options
- **Duplicate**: Creates copy in same column

---

## Visual UI Requirements

### VIS-1: Board Layout

#### VIS-1.1: Overall Structure
```
┌─────────────────────────────────────────────────────────────┐
│  Board Title                          [Search] [Filters...] │
├─────────────────────────────────────────────────────────────┤
│ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐       │
│ │ Column 1 │ │ Column 2 │ │ Column 3 │ │ Column 4 │       │
│ │  (5)     │ │  (3)     │ │  (8)     │ │  (2)     │       │
│ ├──────────┤ ├──────────┤ ├──────────┤ ├──────────┤       │
│ │ ┌──────┐ │ │ ┌──────┐ │ │ ┌──────┐ │ │ ┌──────┐ │       │
│ │ │Card 1│ │ │ │Card 1│ │ │ │Card 1│ │ │ │Card 1│ │       │
│ │ └──────┘ │ │ └──────┘ │ │ └──────┘ │ │ └──────┘ │       │
│ │ ┌──────┐ │ │ ┌──────┐ │ │ ┌──────┐ │ │ ┌──────┐ │       │
│ │ │Card 2│ │ │ │Card 2│ │ │ │Card 2│ │ │ │Card 2│ │       │
│ │ └──────┘ │ │ └──────┘ │ │ └──────┘ │ │ └──────┘ │       │
│ │    ...   │ │    ...   │ │    ...   │ │    ...   │       │
│ └──────────┘ └──────────┘ └──────────┘ └──────────┘       │
└─────────────────────────────────────────────────────────────┘
```

#### VIS-1.2: Spacing & Sizing
- **Board padding**: 16px
- **Column gap**: 16px
- **Column width**: 300px (min), 350px (default), flexible
- **Column min-height**: 400px
- **Card gap**: 8px
- **Card padding**: 12px

### VIS-2: Column Styling

#### VIS-2.1: Column Header
- **Background**: `bg-base-200` (daisyUI)
- **Border**: `border-b-2 border-base-300`
- **Padding**: 12px 16px
- **Elements**:
  - Title (text-lg font-semibold)
  - Card count badge (badge badge-sm)
  - WIP limit (text-xs opacity-60)
  - Collapse icon (chevron)
  - Add card button (btn btn-ghost btn-sm)

#### VIS-2.2: Column Body
- **Background**: `bg-base-100`
- **Border**: `border border-base-300 rounded-lg`
- **Scroll**: `overflow-y-auto max-h-[600px]`
- **Padding**: 8px

#### VIS-2.3: Collapsed Column
- **Width**: 48px
- **Title**: Rotated -90deg, vertical text
- **Icon**: Expand chevron
- **Badge**: Card count

### VIS-3: Card Styling

#### VIS-3.1: Default Card
```css
.kanban-card {
  background: hsl(var(--b1));
  border: 1px solid hsl(var(--b3));
  border-radius: 8px;
  padding: 12px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  cursor: pointer;
  transition: box-shadow 0.2s, transform 0.2s;
}

.kanban-card:hover {
  box-shadow: 0 4px 6px rgba(0,0,0,0.1);
  transform: translateY(-2px);
}
```

#### VIS-3.2: Card Elements
- **Title**: text-base font-semibold mb-2
- **Description**: text-sm opacity-70 line-clamp-2
- **Labels**: Flex row gap-1, badge badge-sm
- **Footer**:
  - Assignee avatars (avatar-group)
  - Priority indicator (colored dot)
  - Due date (text-xs with calendar icon)

#### VIS-3.3: Card States
- **Dragging**: opacity-50, cursor-grabbing
- **Drag Over**: border-primary border-2
- **Selected**: ring-2 ring-primary
- **Filtered Out**: opacity-30 (or hidden)

### VIS-4: Filter Bar Styling

#### VIS-4.1: Layout
```
┌────────────────────────────────────────────────────────────┐
│ [🔍 Search cards...     ] [Assignee ▼] [Labels ▼] [⋯ More] │
│                           [Clear Filters]  Active: (3)     │
└────────────────────────────────────────────────────────────┘
```

#### VIS-4.2: Filter Dropdown
- **Base**: dropdown dropdown-hover
- **Content**: bg-base-100 border border-base-300 rounded-box
- **Items**:
  - Checkbox + label
  - Hover: bg-base-200
  - Selected: bg-primary/10

#### VIS-4.3: Search Input
- **Input**: input input-bordered w-64
- **Icon**: Prefix magnifying glass
- **Clear**: Suffix X button (visible when text entered)

### VIS-5: Animations

#### VIS-5.1: Card Drag Animation
```css
@keyframes card-pickup {
  from { transform: scale(1) rotate(0deg); }
  to { transform: scale(1.05) rotate(2deg); }
}

@keyframes card-drop {
  from { transform: scale(1.05); }
  to { transform: scale(1); }
}
```

#### VIS-5.2: Column Collapse
```css
.column-collapse {
  transition: width 300ms ease-in-out;
}

.column-content {
  transition: opacity 200ms ease-in-out;
}
```

#### VIS-5.3: Filter Highlight
- **Filter applied**: Fade in filtered cards (300ms)
- **No results**: Shake animation on empty state

### VIS-6: Responsive Adjustments

#### Desktop (≥1024px)
- Full horizontal layout
- All columns visible
- Hover effects enabled

#### Tablet (768px - 1023px)
- Horizontal scroll
- Column width: 280px
- Touch-optimized spacing

#### Mobile (<768px)
- Single column view
- Column selector dropdown
- Full-width cards
- Bottom sheet filters
- Touch drag handles

---

## Development Plan

### Phase 1: Core Infrastructure (Week 1)

#### Tasks:
1. **Setup Component Structure**
   - Create `src/components/kanban/` directory
   - Define module structure:
     ```
     kanban/
     ├── mod.rs
     ├── board.rs           # Main board component
     ├── column.rs          # Column component
     ├── card.rs            # Card component
     ├── types.rs           # Data structures
     ├── drag_drop.rs       # Drag-and-drop logic
     └── utils.rs           # Helper functions
     ```

2. **Define Data Structures** (types.rs)
   ```rust
   #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
   pub struct KanbanBoard {
       pub id: String,
       pub title: Option<String>,
       pub columns: Vec<KanbanColumn>,
   }

   #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
   pub struct KanbanColumn {
       pub id: String,
       pub title: String,
       pub color: Option<String>,
       pub card_limit: Option<usize>,
       pub collapsed: bool,
       pub cards: Vec<KanbanCard>,
   }

   #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
   pub struct KanbanCard {
       pub id: String,
       pub title: String,
       pub description: Option<String>,
       pub labels: Vec<Label>,
       pub assignees: Vec<Assignee>,
       pub priority: Priority,
       pub due_date: Option<String>,
       pub metadata: HashMap<String, String>,
   }

   #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
   pub enum Priority {
       Low,
       Medium,
       High,
       Critical,
   }

   #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
   pub struct Label {
       pub id: String,
       pub name: String,
       pub color: String,
   }

   #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
   pub struct Assignee {
       pub id: String,
       pub name: String,
       pub avatar_url: Option<String>,
   }
   ```

3. **Create Basic Components**
   - `KanbanBoard` component (container)
   - `KanbanColumn` component (with header and card list)
   - `KanbanCard` component (display only)

4. **Deliverable**: Static kanban board rendering (no interactions)

### Phase 2: Drag and Drop (Week 2)

#### Tasks:
1. **Integrate leptos-use Drag Hooks**
   - Add `leptos-use` dependency with drag features
   - Study `use_draggable` and `use_drop_zone` APIs

2. **Implement Card Dragging**
   - Make cards draggable with `use_draggable`
   - Add drag handle styling
   - Implement drag preview (semi-transparent clone)

3. **Implement Drop Zones**
   - Column drop zones for receiving cards
   - Card drop zones for reordering within column
   - Highlight drop zones on hover

4. **Implement State Updates**
   - `on_card_move` callback with from/to positions
   - Update column card lists
   - Re-render affected columns

5. **Add Visual Feedback**
   - Dragging state styles
   - Drop zone highlights
   - Smooth animations

6. **Deliverable**: Full drag-and-drop functionality

### Phase 3: Column Features (Week 3)

#### Tasks:
1. **Implement Column Collapse**
   - Collapse toggle button in header
   - Collapsed state styling (vertical title, 48px width)
   - Expand animation (300ms width transition)
   - Persist state to localStorage

2. **Implement Independent Scrolling**
   - Column scroll container with fixed header
   - Max height configuration
   - Auto-scroll during drag near edges
   - Custom scrollbar styling

3. **Add Card Count & WIP Limits**
   - Display card count badge in header
   - WIP limit indicator
   - Warning styling when limit exceeded
   - Optional blocking of drops when at limit

4. **Add "Add Card" Button**
   - Button in column header or footer
   - Inline creation (input field + Enter)
   - `on_card_create` callback

5. **Deliverable**: Fully functional columns with all features

### Phase 4: Filtering & Search (Week 4)

#### Tasks:
1. **Create Filter Bar Component**
   - Position above board
   - Layout: Search + Dropdowns + Clear button

2. **Implement Filter Dropdowns**
   - **Assignee Filter**: Multi-select with avatars
   - **Label Filter**: Multi-select with color chips
   - **Priority Filter**: Multi-select
   - **Due Date Filter**: Predefined options
   - Active filter count badge

3. **Implement Filter Logic**
   - Memo/Signal for filtered cards
   - AND logic between filter types
   - OR logic within filter type
   - Real-time filtering

4. **Implement Search**
   - Search input with debounce (300ms)
   - Search across title, description, labels, assignees
   - Combine with filters
   - Highlight matching text (optional)

5. **Implement Visual Filtering**
   - Hide/show cards based on filters
   - Opacity reduction for non-matching cards
   - Empty state message

6. **Persist Filter State**
   - URL query parameters
   - Share-able filtered URLs

7. **Deliverable**: Complete filtering and search functionality

### Phase 5: Polish & Accessibility (Week 5)

#### Tasks:
1. **Keyboard Navigation**
   - Tab through cards and columns
   - Arrow key navigation
   - Enter/Space for actions
   - Keyboard drag-and-drop

2. **ARIA Labels & Roles**
   - Proper roles for board/columns/cards
   - ARIA labels for all interactive elements
   - Live regions for announcements

3. **Focus Management**
   - Visible focus indicators
   - Focus trap in modals
   - Return focus after actions

4. **Loading & Empty States**
   - Skeleton cards while loading
   - Empty column messages
   - Empty search results message

5. **Error Handling**
   - Validation error messages
   - Drop validation feedback
   - Network error handling

6. **Deliverable**: Accessible, polished component

### Phase 6: Responsive & Mobile (Week 6)

#### Tasks:
1. **Desktop Optimizations**
   - Hover effects
   - Tooltips
   - Context menus

2. **Tablet Adjustments**
   - Horizontal scroll for columns
   - Reduced column width
   - Touch-friendly targets

3. **Mobile Implementation**
   - Single column view
   - Column selector dropdown
   - Bottom sheet filters
   - Touch drag handles
   - Swipe gestures

4. **Touch Optimizations**
   - Long press to drag (500ms)
   - Haptic feedback
   - Larger touch targets (min 44px)

5. **Deliverable**: Fully responsive component

### Phase 7: Advanced Features (Week 7)

#### Tasks:
1. **Card Actions**
   - Quick action menu (three-dot)
   - Edit card modal
   - Delete confirmation
   - Move to column submenu
   - Duplicate card

2. **Card Details Modal**
   - Full card view
   - Edit form
   - Comments section (optional)
   - Activity log (optional)

3. **Performance Optimizations**
   - Virtual scrolling for large columns (>50 cards)
   - Lazy load card images
   - Optimize re-renders

4. **Customization Options**
   - Custom card renderer
   - Custom column header renderer
   - Custom filter predicates
   - Theme overrides

5. **Deliverable**: Feature-complete component

### Phase 8: Testing & Documentation (Week 8)

#### Tasks:
1. **Unit Tests**
   - Test drag-and-drop logic
   - Test filter logic
   - Test search logic
   - Test column collapse
   - Target: 80%+ coverage

2. **Integration Tests**
   - Test complete user workflows
   - Test keyboard navigation
   - Test mobile interactions

3. **Component Documentation**
   - Usage examples
   - API reference
   - Props documentation
   - Callback documentation

4. **Demo Implementation**
   - Create comprehensive demo in `demo/src/demos/kanban.rs`
   - Show all features
   - Interactive examples
   - Code snippets

5. **Performance Benchmarks**
   - Test with 100, 500, 1000 cards
   - Measure drag performance
   - Measure filter performance

6. **Deliverable**: Production-ready, documented component

---

## Technical Architecture

### Component Hierarchy
```
KanbanBoard
├── FilterBar
│   ├── SearchInput
│   └── FilterDropdown (multiple)
├── KanbanColumn (multiple)
│   ├── ColumnHeader
│   │   ├── CollapseButton
│   │   ├── CardCountBadge
│   │   └── AddCardButton
│   └── ColumnBody (scroll container)
│       └── KanbanCard (multiple)
│           ├── CardHeader
│           ├── CardBody
│           └── CardFooter
└── CardDetailModal (conditional)
```

### State Management
```rust
// Board-level state
let (columns, set_columns) = signal(initial_columns);
let (drag_state, set_drag_state) = signal(None);
let (filters, set_filters) = signal(FilterState::default());
let (search_query, set_search_query) = signal(String::new());

// Computed/Memo signals
let filtered_cards = Memo::new(move |_| {
    apply_filters(columns.get(), filters.get(), search_query.get())
});

// Context for child components
provide_context(KanbanContext {
    columns,
    set_columns,
    drag_state,
    set_drag_state,
    on_card_move,
    on_card_click,
});
```

### Drag-and-Drop Flow
```
1. User starts drag
   → Set drag_state with card_id, source_column_id, source_index

2. User drags over drop zone
   → Highlight drop zone
   → Show insertion preview

3. User drops card
   → Validate drop (WIP limits, custom validation)
   → Update columns state (remove from source, insert at target)
   → Trigger on_card_move callback
   → Clear drag_state

4. Animate card movement
   → Transition to new position
```

### Filter Logic
```rust
fn apply_filters(
    columns: Vec<KanbanColumn>,
    filters: FilterState,
    search: String,
) -> Vec<KanbanColumn> {
    columns
        .into_iter()
        .map(|mut col| {
            col.cards = col.cards
                .into_iter()
                .filter(|card| {
                    // Search filter
                    let matches_search = search.is_empty() ||
                        card.title.to_lowercase().contains(&search.to_lowercase()) ||
                        card.description.as_ref()
                            .map(|d| d.to_lowercase().contains(&search.to_lowercase()))
                            .unwrap_or(false);

                    // Assignee filter (OR within category)
                    let matches_assignee = filters.assignees.is_empty() ||
                        card.assignees.iter()
                            .any(|a| filters.assignees.contains(&a.id));

                    // Label filter (OR within category)
                    let matches_label = filters.labels.is_empty() ||
                        card.labels.iter()
                            .any(|l| filters.labels.contains(&l.id));

                    // Priority filter
                    let matches_priority = filters.priorities.is_empty() ||
                        filters.priorities.contains(&card.priority);

                    // AND between categories
                    matches_search && matches_assignee && matches_label && matches_priority
                })
                .collect();
            col
        })
        .collect()
}
```

---

## CSS Classes (daisyUI Integration)

### Board
```css
.kanban-board {
  @apply bg-base-100 rounded-box p-4;
}

.kanban-columns {
  @apply flex gap-4 overflow-x-auto;
}
```

### Column
```css
.kanban-column {
  @apply flex-shrink-0 w-80 bg-base-200 rounded-box border border-base-300;
}

.kanban-column-header {
  @apply flex items-center justify-between p-3 border-b-2 border-base-300;
}

.kanban-column-title {
  @apply text-lg font-semibold flex items-center gap-2;
}

.kanban-column-badge {
  @apply badge badge-sm badge-neutral;
}

.kanban-column-body {
  @apply p-2 overflow-y-auto max-h-[600px] space-y-2;
}

.kanban-column-collapsed {
  @apply w-12 p-0;
}

.kanban-column-collapsed .kanban-column-title {
  @apply transform -rotate-90 origin-center whitespace-nowrap;
}
```

### Card
```css
.kanban-card {
  @apply card bg-base-100 shadow-sm hover:shadow-md transition-all cursor-pointer;
  @apply border border-base-300 hover:-translate-y-0.5;
}

.kanban-card-dragging {
  @apply opacity-50 cursor-grabbing;
}

.kanban-card-header {
  @apply card-body p-3;
}

.kanban-card-title {
  @apply card-title text-base mb-2;
}

.kanban-card-description {
  @apply text-sm opacity-70 line-clamp-2 mb-2;
}

.kanban-card-labels {
  @apply flex flex-wrap gap-1 mb-2;
}

.kanban-card-label {
  @apply badge badge-sm;
}

.kanban-card-footer {
  @apply flex items-center justify-between text-xs;
}

.kanban-card-priority-high {
  @apply border-l-4 border-error;
}

.kanban-card-priority-medium {
  @apply border-l-4 border-warning;
}

.kanban-card-priority-low {
  @apply border-l-4 border-info;
}
```

### Filter Bar
```css
.kanban-filter-bar {
  @apply bg-base-200 rounded-box p-4 mb-4 flex flex-wrap gap-3 items-center;
}

.kanban-search {
  @apply input input-bordered w-64;
}

.kanban-filter-dropdown {
  @apply dropdown dropdown-hover;
}

.kanban-filter-menu {
  @apply dropdown-content menu bg-base-100 rounded-box shadow-lg border border-base-300 w-64 p-2;
}

.kanban-filter-item {
  @apply flex items-center gap-2 p-2 hover:bg-base-200 rounded-btn cursor-pointer;
}

.kanban-filter-badge {
  @apply badge badge-primary badge-sm;
}
```

---

## Best Practices & Patterns

### 1. Signal Optimization
- Use `Memo` for expensive computations (filtering, sorting)
- Minimize signal reads in loops
- Use `RwSignal` for frequently updated values

### 2. Event Handling
- Debounce search input (300ms)
- Throttle scroll events (16ms for 60fps)
- Use event delegation for card clicks

### 3. Accessibility
- Provide keyboard alternatives for all mouse interactions
- Use semantic HTML elements
- Maintain logical tab order
- Announce dynamic changes with ARIA live regions

### 4. Performance
- Virtual scrolling for columns with >50 cards
- Lazy load card details and images
- Use CSS transforms for animations (GPU accelerated)
- Avoid layout thrashing

### 5. Error Handling
- Validate drops before committing
- Provide user feedback for errors
- Graceful degradation for missing data
- Undo support for destructive actions

---

## Example Usage

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn MyKanbanApp() -> impl IntoView {
    let (columns, set_columns) = signal(vec![
        KanbanColumn {
            id: "todo".to_string(),
            title: "To Do".to_string(),
            color: None,
            card_limit: Some(5),
            collapsed: false,
            cards: vec![
                KanbanCard {
                    id: "card-1".to_string(),
                    title: "Implement login page".to_string(),
                    description: Some("Create user authentication UI".to_string()),
                    labels: vec![
                        Label {
                            id: "frontend".to_string(),
                            name: "Frontend".to_string(),
                            color: "#3B82F6".to_string(),
                        },
                    ],
                    assignees: vec![],
                    priority: Priority::High,
                    due_date: Some("2024-02-15".to_string()),
                    metadata: HashMap::new(),
                },
            ],
        },
        KanbanColumn {
            id: "in-progress".to_string(),
            title: "In Progress".to_string(),
            color: Some("#F59E0B".to_string()),
            card_limit: Some(3),
            collapsed: false,
            cards: vec![],
        },
        KanbanColumn {
            id: "done".to_string(),
            title: "Done".to_string(),
            color: Some("#10B981".to_string()),
            card_limit: None,
            collapsed: false,
            cards: vec![],
        },
    ]);

    let on_card_move = Callback::new(move |event: CardMoveEvent| {
        set_columns.update(|cols| {
            // Remove card from source column
            let card = cols
                .iter_mut()
                .find(|c| c.id == event.from_column_id)
                .and_then(|col| {
                    if event.from_index < col.cards.len() {
                        Some(col.cards.remove(event.from_index))
                    } else {
                        None
                    }
                });

            // Insert card into target column
            if let Some(card) = card {
                if let Some(target_col) = cols.iter_mut().find(|c| c.id == event.to_column_id) {
                    let insert_index = event.to_index.min(target_col.cards.len());
                    target_col.cards.insert(insert_index, card);
                }
            }
        });
    });

    view! {
        <KanbanBoard
            columns=columns
            on_card_move=on_card_move
            on_card_click=Callback::new(|card_id: String| {
                leptos::logging::log!("Card clicked: {}", card_id);
            })
            enable_search=true
            enable_filters=true
            max_height="calc(100vh - 200px)"
        />
    }
}
```

---

## Success Metrics

### User Experience
- Drag operation completes in <100ms
- Search results appear within 300ms of typing
- Animations run at 60fps
- Component loads in <1s with 100 cards

### Code Quality
- 80%+ test coverage
- Zero accessibility violations (axe-core)
- Zero TypeScript/Rust compilation errors
- <10 minor linting warnings

### Documentation
- 100% of public APIs documented
- 5+ usage examples
- Interactive demo with all features
- Troubleshooting guide

---

## Future Enhancements (Post-MVP)

1. **Card Templates**: Predefined card layouts for different use cases
2. **Swimlanes**: Horizontal grouping of columns
3. **Card Dependencies**: Link cards with dependency arrows
4. **Bulk Operations**: Multi-select and move cards
5. **Card Attachments**: File uploads and previews
6. **Real-time Collaboration**: Multi-user editing with presence indicators
7. **Automation Rules**: Auto-move cards based on conditions
8. **Custom Fields**: User-defined card properties
9. **Export/Import**: JSON/CSV board data export
10. **Archive**: Archive completed cards
11. **Activity Timeline**: Track all board changes
12. **Notifications**: Card assignment and due date reminders

---

## References

### Inspiration from Best-in-Class Examples
- **Trello**: Card design, drag-and-drop UX, quick filters
- **Jira**: Advanced filtering, swimlanes, card details
- **GitHub Projects**: Column limits, keyboard navigation, responsive design
- **Notion**: Inline editing, database views, property customization
- **ClickUp**: Custom fields, automation, real-time collaboration

### Technical References
- [leptos-use drag hooks](https://leptos-use.rs/elements/use_draggable.html)
- [daisyUI components](https://daisyui.com/components/)
- [Web Accessibility Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [MDN Drag and Drop API](https://developer.mozilla.org/en-US/docs/Web/API/HTML_Drag_and_Drop_API)
