# Thaw UI Component Implementation Plan

## 🎯 Project Overview

**Goal:** Add 30 Thaw UI-inspired components to leptos-daisyui-rs
**From:** 62 components → **To:** 92 components
**Design System:** daisyUI (maintaining theme integration)
**Reference:** Thaw UI (for functionality and interactivity)
**Status:** Planning Complete - Ready for Implementation

---

## 📊 Beads Issue Structure

### Main Epic
- **leptos-daisyui-rs-loq** - Add 30 Thaw-UI-inspired components to leptos-daisyui-rs

### Sub-Epics (7 categories)

1. **leptos-daisyui-rs-loq.1** - Form & Data Input Components (10 components)
2. **leptos-daisyui-rs-loq.2** - Layout & Structure Components (6 components)
3. **leptos-daisyui-rs-loq.3** - Data Display & Typography Components (5 components)
4. **leptos-daisyui-rs-loq.4** - Navigation Components (2 components)
5. **leptos-daisyui-rs-loq.5** - Feedback & Loading Components (3 components)
6. **leptos-daisyui-rs-loq.6** - Specialized Components (3 components)
7. **leptos-daisyui-rs-loq.7** - System Components (1 component)

### Total Tasks: 30 component implementation tasks

---

## 🔴 Priority 1 (Critical - Implement First)

These are the most important missing components:

| ID | Component | Category | Why Critical |
|----|-----------|----------|--------------|
| **leptos-daisyui-rs-loq.1.4** | DatePicker | Form | Essential for date selection in forms |
| **leptos-daisyui-rs-loq.1.9** | TimePicker | Form | Complements DatePicker for datetime |
| **leptos-daisyui-rs-loq.3.4** | Tree | Data Display | Required for hierarchical data (files, org charts) |
| **leptos-daisyui-rs-loq.4.2** | Popover | Navigation | Essential for context menus, tooltips |
| **leptos-daisyui-rs-loq.6.3** | Icon + Lucide | Specialized | Icon system with Lucide integration |

**Recommendation:** Start with these 5 components to provide the most value quickly.

---

## 🟠 Priority 2 (High Value - Implement Next)

**Form & Data Input (8 components):**
- leptos-daisyui-rs-loq.1.1 - AutoComplete
- leptos-daisyui-rs-loq.1.2 - ColorPicker
- leptos-daisyui-rs-loq.1.3 - Combobox
- leptos-daisyui-rs-loq.1.5 - Field
- leptos-daisyui-rs-loq.1.6 - Slider
- leptos-daisyui-rs-loq.1.7 - SpinButton
- leptos-daisyui-rs-loq.1.8 - TagPicker
- leptos-daisyui-rs-loq.1.10 - Upload

**Layout & Structure (5 components):**
- leptos-daisyui-rs-loq.2.1 - Flex
- leptos-daisyui-rs-loq.2.2 - Grid
- leptos-daisyui-rs-loq.2.3 - Layout
- leptos-daisyui-rs-loq.2.4 - Space
- leptos-daisyui-rs-loq.2.6 - BackTop

**Data Display (3 components):**
- leptos-daisyui-rs-loq.3.1 - Code
- leptos-daisyui-rs-loq.3.2 - Image
- leptos-daisyui-rs-loq.3.3 - Text

**Navigation (1 component):**
- leptos-daisyui-rs-loq.4.1 - Nav

**Feedback & Loading (2 components):**
- leptos-daisyui-rs-loq.5.1 - LoadingBar
- leptos-daisyui-rs-loq.5.2 - MessageBar

**Specialized (1 component):**
- leptos-daisyui-rs-loq.6.2 - Tag

**System (1 component):**
- leptos-daisyui-rs-loq.7.1 - ConfigProvider

---

## 🟢 Priority 3 (Nice to Have - Implement Last)

- leptos-daisyui-rs-loq.2.5 - Scrollbar
- leptos-daisyui-rs-loq.3.5 - InfoLabel
- leptos-daisyui-rs-loq.5.3 - Spinner (similar to existing Loading)
- leptos-daisyui-rs-loq.6.1 - Persona

---

## 📋 Component Details

### 1. Form & Data Input Components (10)

#### leptos-daisyui-rs-loq.1.1 - AutoComplete
**Purpose:** Input with auto-completion suggestions
**Thaw Reference:** https://thawui.vercel.app/components/auto-complete
**Key Features:**
- Dropdown suggestions as user types
- Keyboard navigation (arrow keys, enter, escape)
- Async data loading support
- Customizable suggestion rendering
- Clear button

**Implementation Notes:**
- Use Signal<String> for input value
- Signal<Vec<T>> for suggestions
- Callback for onSelect
- Filter logic for matching
- daisyUI dropdown + input styling

---

#### leptos-daisyui-rs-loq.1.2 - ColorPicker
**Purpose:** Color selection interface
**Thaw Reference:** https://thawui.vercel.app/components/color-picker
**Key Features:**
- Color palette grid
- Hex/RGB/HSL/HSV input modes
- Eye dropper support (browser API)
- Recent colors history
- Alpha/opacity slider
- Preset color swatches

**Implementation Notes:**
- ColorMode enum (Hex, RGB, HSL, HSV)
- Signal<Color> with conversion methods
- Canvas for color picker
- Use daisyUI theme colors as presets
- Copy to clipboard functionality

---

#### leptos-daisyui-rs-loq.1.3 - Combobox
**Purpose:** Searchable dropdown
**Thaw Reference:** https://thawui.vercel.app/components/combobox
**Key Features:**
- Dropdown with search input
- Filter options as you type
- Custom option rendering
- Multi-select support
- Keyboard navigation
- Option groups

**Implementation Notes:**
- ComboboxOption sub-component
- Signal<Vec<T>> for options
- Filter function for search
- Multiple selection mode
- Similar to existing Dropdown but searchable

---

#### leptos-daisyui-rs-loq.1.4 - DatePicker ⭐
**Purpose:** Calendar-based date selection
**Thaw Reference:** https://thawui.vercel.app/components/date-picker
**Key Features:**
- Calendar grid display
- Month/year navigation
- Date range selection
- Min/max date constraints
- Disabled dates support
- Multiple date formats
- Keyboard shortcuts
- Today button
- Week numbers (optional)

**Implementation Notes:**
- Use existing Calendar component as base
- Add date selection logic
- Signal<Option<NaiveDate>> for single
- Signal<(NaiveDate, NaiveDate)> for range
- Date validation and constraints
- Format string support
- Integration with chrono crate

---

#### leptos-daisyui-rs-loq.1.5 - Field
**Purpose:** Form field wrapper with label and validation
**Thaw Reference:** https://thawui.vercel.app/components/field
**Key Features:**
- Label with optional required indicator
- Help text / hint text
- Error message display
- Validation state styling
- Layout variants (vertical/horizontal)
- Wraps any form input component

**Implementation Notes:**
- FieldLabel, FieldHint, FieldError sub-components
- ValidationState enum (None, Valid, Invalid, Warning)
- Children prop for input
- required prop for * indicator
- Use daisyUI form-control classes

---

#### leptos-daisyui-rs-loq.1.6 - Slider
**Purpose:** Enhanced range slider
**Thaw Reference:** https://thawui.vercel.app/components/slider
**Key Features:**
- Single or dual handles (range)
- Min/max/step configuration
- Value labels/tooltips
- Vertical/horizontal orientation
- Marks/ticks display
- Disabled state
- Keyboard support (arrow keys)

**Implementation Notes:**
- SliderOrientation enum
- Signal<f64> for single value
- Signal<(f64, f64)> for range
- Track, fill, and handle elements
- Mouse drag handlers
- More advanced than existing Range component

---

#### leptos-daisyui-rs-loq.1.7 - SpinButton
**Purpose:** Number input with increment/decrement buttons
**Thaw Reference:** https://thawui.vercel.app/components/spin-button
**Key Features:**
- Numeric input field
- +/- buttons
- Min/max constraints
- Step amount
- Precision/decimal places
- Button position variants (inline/stacked)
- Keyboard shortcuts (arrow up/down)
- Hold to repeat increment

**Implementation Notes:**
- ButtonPosition enum (Inline, Stacked)
- Signal<f64> for value
- Validation on input
- Debounced onChange
- daisyUI button + input styling

---

#### leptos-daisyui-rs-loq.1.8 - TagPicker
**Purpose:** Multi-select with tag display
**Thaw Reference:** https://thawui.vercel.app/components/tag-picker
**Key Features:**
- Multi-select with tag chips
- Searchable/filterable options
- Add custom tags
- Remove tags with X button
- Maximum selections limit
- Tag color variants
- Autocomplete suggestions
- Keyboard navigation

**Implementation Notes:**
- Signal<Vec<String>> for selected tags
- Dropdown for options
- Tag display with remove button
- Integration with existing Badge component
- allow_custom prop for user-created tags

---

#### leptos-daisyui-rs-loq.1.9 - TimePicker ⭐
**Purpose:** Time selection interface
**Thaw Reference:** https://thawui.vercel.app/components/time-picker
**Key Features:**
- Hour/minute/second selectors
- 12-hour or 24-hour format
- AM/PM toggle
- Scrollable time columns
- Manual input support
- Now button
- Keyboard navigation
- Step intervals (e.g., 15-minute increments)

**Implementation Notes:**
- TimeFormat enum (Hour12, Hour24)
- Spinner columns for hour/minute/second
- Signal<NaiveTime> for value
- Scrollable dropdowns
- Integration with chrono crate
- Validation for time constraints

---

#### leptos-daisyui-rs-loq.1.10 - Upload
**Purpose:** Advanced file upload
**Thaw Reference:** https://thawui.vercel.app/components/upload
**Key Features:**
- Drag and drop zone
- Click to browse
- Multiple file selection
- File type restrictions (accept)
- File size limits
- Upload progress indicator
- File preview (images)
- Remove files before upload
- Custom upload handler

**Implementation Notes:**
- UploadFile struct (name, size, type, data)
- Drag/drop event handlers
- Signal<Vec<UploadFile>> for file list
- onUpload callback (async)
- Progress tracking per file
- More advanced than existing FileInput
- Image preview with FileReader API

---

### 2. Layout & Structure Components (6)

#### leptos-daisyui-rs-loq.2.1 - Flex
**Purpose:** Flexbox layout wrapper
**Thaw Reference:** https://thawui.vercel.app/components/flex
**Key Features:**
- Flex direction (row, column)
- Gap/spacing control
- Justify content
- Align items
- Wrap control

**Implementation Notes:**
- FlexDirection enum
- JustifyContent enum
- AlignItems enum
- Gap size props
- Renders as `<div>` with flex CSS

---

#### leptos-daisyui-rs-loq.2.2 - Grid
**Purpose:** CSS Grid layout wrapper
**Thaw Reference:** https://thawui.vercel.app/components/grid
**Key Features:**
- Column/row configuration
- Gap control
- Responsive columns
- Auto-fit/auto-fill

**Implementation Notes:**
- columns prop (number or "auto-fit")
- rows prop
- gap size
- Renders as `<div>` with grid CSS

---

#### leptos-daisyui-rs-loq.2.3 - Layout
**Purpose:** App shell structure
**Thaw Reference:** https://thawui.vercel.app/components/layout
**Key Features:**
- Header/Sidebar/Content/Footer areas
- Collapsible sidebar
- Fixed/sticky header
- Responsive behavior

**Implementation Notes:**
- LayoutHeader, LayoutSidebar, LayoutContent, LayoutFooter sub-components
- Sidebar collapse state
- CSS Grid for layout structure
- Responsive breakpoints

---

#### leptos-daisyui-rs-loq.2.4 - Space
**Purpose:** Spacing utility
**Thaw Reference:** https://thawui.vercel.app/components/space
**Key Features:**
- Consistent spacing between children
- Vertical/horizontal direction
- Size variants (small, medium, large)
- Custom gap size

**Implementation Notes:**
- SpaceDirection enum
- SpaceSize enum
- CSS gap property
- Wraps children with spacing

---

#### leptos-daisyui-rs-loq.2.5 - Scrollbar
**Purpose:** Custom scrollbar styling
**Thaw Reference:** https://thawui.vercel.app/components/scrollbar
**Key Features:**
- Custom scrollbar width
- Track and thumb styling
- Theme-aware colors
- Cross-browser support

**Implementation Notes:**
- CSS ::-webkit-scrollbar
- Firefox scrollbar-width
- daisyUI theme colors
- Wrapper component

---

#### leptos-daisyui-rs-loq.2.6 - BackTop
**Purpose:** Back-to-top button
**Thaw Reference:** https://thawui.vercel.app/components/back-top
**Key Features:**
- Fixed position floating button
- Appears after scroll threshold
- Smooth scroll to top
- Customizable icon
- Bottom-right positioning

**Implementation Notes:**
- Scroll event listener
- visibility_threshold prop
- Smooth scroll behavior
- Fixed positioning
- Use existing Button styling

---

### 3. Data Display & Typography Components (5)

#### leptos-daisyui-rs-loq.3.1 - Code
**Purpose:** Code syntax display
**Thaw Reference:** https://thawui.vercel.app/components/code
**Key Features:**
- Syntax highlighting (optional)
- Line numbers
- Copy to clipboard button
- Language label
- Inline vs block mode

**Implementation Notes:**
- Use existing MockupCode as base
- Add copy button
- Optional syntax highlighting with highlight.js
- CodeLanguage enum
- Inline variant

---

#### leptos-daisyui-rs-loq.3.2 - Image
**Purpose:** Enhanced image component
**Thaw Reference:** https://thawui.vercel.app/components/image
**Key Features:**
- Lazy loading
- Placeholder while loading
- Error fallback image
- Zoom on click (lightbox)
- Aspect ratio control

**Implementation Notes:**
- loading="lazy" attribute
- placeholder slot
- onError fallback
- Click to expand modal
- Aspect ratio CSS

---

#### leptos-daisyui-rs-loq.3.3 - Text
**Purpose:** Typography component
**Thaw Reference:** https://thawui.vercel.app/components/text
**Key Features:**
- Size variants (xs, sm, base, lg, xl, etc.)
- Font weight control
- Color variants
- Line clamp/truncation
- Italic, underline, strikethrough

**Implementation Notes:**
- TextSize enum
- TextWeight enum
- lines prop for clamp
- CSS text utilities
- daisyUI text colors

---

#### leptos-daisyui-rs-loq.3.4 - Tree ⭐
**Purpose:** Hierarchical tree view
**Thaw Reference:** https://thawui.vercel.app/components/tree
**Key Features:**
- Expandable/collapsible nodes
- Nested structure
- Checkboxes (optional)
- Icons for files/folders
- Selection support
- Search/filter
- Drag and drop (advanced)

**Implementation Notes:**
- TreeNode component (recursive)
- TreeNodeData struct
- expanded state Signal
- selected state Signal
- Icons for expand/collapse
- Indentation for hierarchy
- Complex but very useful

---

#### leptos-daisyui-rs-loq.3.5 - InfoLabel
**Purpose:** Information label with icon
**Thaw Reference:** https://thawui.vercel.app/components/info-label
**Key Features:**
- Label with info icon
- Tooltip on hover
- Color variants
- Size variants

**Implementation Notes:**
- Use existing Tooltip component
- Info icon (requires Icon component)
- Simple wrapper
- daisyUI colors

---

### 4. Navigation Components (2)

#### leptos-daisyui-rs-loq.4.1 - Nav
**Purpose:** Navigation component
**Thaw Reference:** https://thawui.vercel.app/components/nav
**Key Features:**
- Vertical navigation list
- Active state indication
- Nested navigation items
- Icons for items
- Collapsible groups

**Implementation Notes:**
- NavItem, NavGroup sub-components
- active prop for current page
- Different from Navbar (vertical vs horizontal)
- Sidebar navigation use case

---

#### leptos-daisyui-rs-loq.4.2 - Popover ⭐
**Purpose:** Floating content overlay
**Thaw Reference:** https://thawui.vercel.app/components/popover
**Key Features:**
- Floating content
- Positioning (top, bottom, left, right, auto)
- Trigger modes (hover, click, focus)
- Arrow pointer
- Close on outside click
- Portal rendering

**Implementation Notes:**
- PopoverTrigger, PopoverContent sub-components
- Positioning logic (floating-ui or similar)
- Trigger event handlers
- Different from Dropdown (more flexible)
- Z-index management

---

### 5. Feedback & Loading Components (3)

#### leptos-daisyui-rs-loq.5.1 - LoadingBar
**Purpose:** Top-of-page loading indicator
**Thaw Reference:** https://thawui.vercel.app/components/loading-bar
**Key Features:**
- Fixed top positioning
- Progress percentage
- Auto-increment animation
- Start/finish methods
- Color customization

**Implementation Notes:**
- Fixed positioning (top: 0)
- Width percentage based on progress
- Signal<f64> for progress (0-100)
- Auto-increment with setTimeout
- Similar to YouTube/GitHub progress bar

---

#### leptos-daisyui-rs-loq.5.2 - MessageBar
**Purpose:** Inline message notifications
**Thaw Reference:** https://thawui.vercel.app/components/message-bar
**Key Features:**
- Color variants (info, success, warning, error)
- Dismissable
- Icon display
- Multi-line support
- Action buttons

**Implementation Notes:**
- MessageBarType enum
- close_button prop
- Icons for each type
- Similar to existing Alert but inline
- daisyUI alert styling

---

#### leptos-daisyui-rs-loq.5.3 - Spinner
**Purpose:** Loading spinner
**Thaw Reference:** https://thawui.vercel.app/components/spinner
**Key Features:**
- Size variants
- Color variants
- Inline or block display

**Implementation Notes:**
- SpinnerSize enum
- CSS animation
- Similar to existing Loading component
- Thaw-style design

---

### 6. Specialized Components (3)

#### leptos-daisyui-rs-loq.6.1 - Persona
**Purpose:** User profile card
**Thaw Reference:** https://thawui.vercel.app/components/persona
**Key Features:**
- Avatar display
- Name and role/title
- Online status indicator
- Size variants
- Presence badge

**Implementation Notes:**
- Use existing Avatar component
- PersonaSize enum
- PresenceStatus enum (online, offline, away, busy)
- Microsoft Fluent Design style
- Adapt to daisyUI theme

---

#### leptos-daisyui-rs-loq.6.2 - Tag
**Purpose:** Tag/chip component
**Thaw Reference:** https://thawui.vercel.app/components/tag
**Key Features:**
- Color variants
- Size variants
- Dismissable (close button)
- Icon support
- Rounded/square variants

**Implementation Notes:**
- Different from existing Badge
- TagSize enum
- TagColor enum
- closable prop with onClose callback
- More interactive than Badge

---

#### leptos-daisyui-rs-loq.6.3 - Icon + Lucide ⭐
**Purpose:** Icon component system
**Thaw Reference:** https://thawui.vercel.app/components/icon
**Lucide Icons:** https://lucide.dev/icons/
**Key Features:**
- Lucide icon set integration (400+ icons)
- Size variants
- Color props
- Rotation
- Stroke width control
- Icon name as prop

**Implementation Notes:**
- Use leptos-icons or create wrapper
- Lucide icons as SVG components
- IconSize enum
- rotation prop (0, 90, 180, 270)
- color from daisyUI theme
- CRITICAL: Must support all Lucide icons

**Lucide Integration:**
```rust
// Example usage goal:
<Icon name="home" size=IconSize::Medium />
<Icon name="settings" color="primary" />
<Icon name="arrow-right" rotation=90 />
```

---

### 7. System Components (1)

#### leptos-daisyui-rs-loq.7.1 - ConfigProvider
**Purpose:** Global configuration provider
**Thaw Reference:** https://thawui.vercel.app/components/config-provider
**Key Features:**
- Theme configuration
- i18n/locale settings
- Global component defaults
- Context-based

**Implementation Notes:**
- Leptos context provider
- ConfigContext struct
- Integration with existing ThemeController
- Locale support for future i18n
- Component default props

---

## 🎨 Design Integration Guidelines

### daisyUI Theme Integration

All components must:
1. Use daisyUI CSS custom properties for colors
2. Support theme switching via ThemeController
3. Follow daisyUI naming conventions for variants
4. Use daisyUI utility classes where possible

### Example Color Integration:
```rust
// Use daisyUI theme colors
pub enum ComponentColor {
    Primary,    // bg-primary, text-primary
    Secondary,  // bg-secondary, text-secondary
    Accent,     // bg-accent, text-accent
    Neutral,    // bg-neutral, text-neutral
    // ... etc
}
```

### Component Structure Pattern:
```rust
use leptos::prelude::*;
use crate::merge_classes;

#[component]
pub fn NewComponent(
    #[prop(optional, into)] variant: Signal<ComponentVariant>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "new-component",
                    variant.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </div>
    }
}
```

---

## 📚 Reference Resources

### Thaw UI
- **Docs:** https://thawui.vercel.app
- **GitHub:** https://github.com/thaw-ui/thaw
- **Source:** /tmp/thaw/thaw/src/

### Lucide Icons
- **Website:** https://lucide.dev/icons/
- **Icons:** 400+ open source icons
- **License:** ISC License (permissive)

### daisyUI
- **Docs:** https://daisyui.com/components/
- **Theme System:** https://daisyui.com/docs/themes/
- **Colors:** https://daisyui.com/docs/colors/

---

## 🚀 Implementation Workflow

For each component:

1. **Research Phase**
   - Study Thaw UI implementation
   - Check daisyUI equivalent (if any)
   - Review reference docs

2. **Design Phase**
   - Define style enums
   - Plan Signal props
   - Design component API
   - Map to daisyUI theme

3. **Implementation Phase**
   - Create component directory
   - Write component.rs
   - Write style.rs (if needed)
   - Write mod.rs
   - Add to src/components/mod.rs

4. **Documentation Phase**
   - Add doc comments
   - CSS @source directives
   - Usage examples
   - Update README.md

5. **Testing Phase**
   - Compile check
   - Visual testing in demo
   - Theme switching test
   - Responsiveness check

---

## 📈 Progress Tracking

Use beads to track progress:

```bash
# List all open tasks
bd list --status=open

# Show ready tasks (no blockers)
bd ready

# Start working on a task
bd update leptos-daisyui-rs-loq.1.4 --status=in_progress

# Complete a task
bd close leptos-daisyui-rs-loq.1.4

# Check project stats
bd stats
```

---

## ✅ Success Criteria

- ✅ All 30 components implemented
- ✅ All components integrate with daisyUI themes
- ✅ All components follow existing library patterns
- ✅ All components documented with examples
- ✅ README.md updated with new component count (92)
- ✅ Icon component supports Lucide icon set
- ✅ All components compile without errors
- ✅ Demo app showcases new components

---

## 🎯 Suggested Implementation Order

### Phase 1: Critical Components (Week 1)
1. Icon + Lucide (needed by many others)
2. DatePicker
3. TimePicker
4. Tree
5. Popover

### Phase 2: High-Value Forms (Week 2)
6. Field (wrapper for others)
7. AutoComplete
8. Combobox
9. ColorPicker
10. Upload
11. Slider
12. TagPicker
13. SpinButton

### Phase 3: Layout & Display (Week 3)
14. Flex
15. Grid
16. Layout
17. Space
18. Code
19. Image
20. Text
21. BackTop

### Phase 4: Navigation & Feedback (Week 4)
22. Nav
23. LoadingBar
24. MessageBar
25. Tag

### Phase 5: System & Polish (Week 5)
26. ConfigProvider
27. Persona
28. InfoLabel
29. Scrollbar
30. Spinner

---

## 📞 Questions or Issues?

Track in beads:
```bash
bd create --title="Question: [your question]" --type=task --parent=leptos-daisyui-rs-loq
```

---

**Ready to start!** 🚀

Begin with: `bd ready` to see available tasks.
