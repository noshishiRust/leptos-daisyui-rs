# Gantt Chart Component - Development Plan Summary

**Created:** 2026-01-30
**Epic ID:** `leptos-daisyui-rs-iyk`
**Total Issues:** 88 (1 epic + 10 features + 77 tasks)
**Estimated Duration:** 20 weeks

---

## Epic Overview

**Gantt Chart Component** - Production-ready Gantt chart component for leptos-daisyui-rs library with interactive task scheduling, dependencies, critical path, and resource management. Based on best-in-class implementations (Frappe, DHTMLX, SVAR).

---

## Feature Breakdown

### Phase 1: Foundation & Data Model (Week 1-2)
**ID:** `leptos-daisyui-rs-iyk.1` | **Priority:** P0 | **Tasks:** 9

**Deliverables:**
- Module structure (gantt/, utils/, task_list/, timeline/, interactions/, dialogs/)
- GanttTask and TaskDependency data models
- TaskType, ViewMode, GanttTaskHeight enums
- Date/time utility functions
- Basic GanttChart component structure
- Simple task list view (no tree)
- Unit tests for models and date calculations

**Tasks:**
1. Create gantt module structure
2. Implement GanttTask data model
3. Implement TaskDependency data model
4. Implement TaskType and ViewMode enums
5. Build date/time utility functions
6. Create GanttChart component structure
7. Implement simple task list view
8. Add unit tests for data models
9. Add unit tests for date calculations

---

### Phase 2: Static Timeline (Week 3-4)
**ID:** `leptos-daisyui-rs-iyk.2` | **Priority:** P0 | **Tasks:** 8
**Depends on:** Phase 1

**Deliverables:**
- Timeline grid SVG rendering
- Time scale calculation (hour/day/week/month/quarter/year)
- Task bar rendering on timeline
- Weekend/holiday shading
- Today marker line
- Synchronized scrolling
- Horizontal scrollbar
- Responsive split panel with resize

**Tasks:**
1. Implement timeline grid SVG rendering
2. Calculate time scale headers
3. Render task bars on timeline
4. Implement weekend and holiday shading
5. Add today marker line
6. Synchronize task list and timeline scrolling
7. Add horizontal scrollbar for timeline
8. Create responsive split panel with resize

---

### Phase 3: Tree Structure (Week 5)
**ID:** `leptos-daisyui-rs-iyk.3` | **Priority:** P1 | **Tasks:** 6
**Depends on:** Phase 2

**Deliverables:**
- Task tree data structure
- Expand/collapse functionality
- Auto-calculated parent task dates
- Visual indentation (20px per level)
- Optional tree connector lines
- Parent bars spanning children

**Tasks:**
1. Build task tree data structure
2. Implement expand/collapse functionality
3. Calculate parent task dates from children
4. Add visual indentation for nested tasks
5. Implement tree connector lines
6. Make parent task bars span children

---

### Phase 4: Basic Interactivity (Week 6-7)
**ID:** `leptos-daisyui-rs-iyk.4` | **Priority:** P1 | **Tasks:** 9
**Depends on:** Phase 3

**Deliverables:**
- Task selection (single and multi-select)
- Zoom in/out controls
- Zoom to fit
- Pan timeline with mouse drag
- Keyboard navigation (25+ shortcuts)
- Context menu (right-click)
- Inline task name editing
- Task edit dialog

**Tasks:**
1. Implement single task selection
2. Implement multi-selection
3. Add zoom in/out controls
4. Implement zoom to fit functionality
5. Add pan timeline with mouse drag
6. Implement keyboard navigation
7. Create context menu component
8. Implement inline task name editing
9. Create task edit dialog

---

### Phase 5: Drag and Drop (Week 8-9)
**ID:** `leptos-daisyui-rs-iyk.5` | **Priority:** P1 | **Tasks:** 9
**Depends on:** Phase 4

**Deliverables:**
- Drag state management
- Drag to move task (shift dates)
- Drag to resize start/end
- Drag progress indicator
- Visual drag preview
- Snap to grid (configurable)
- Drag constraints (min duration, working days)
- Undo/redo support (50 operations)

**Tasks:**
1. Implement drag state management
2. Drag to move task (change dates)
3. Drag to resize task start edge
4. Drag to resize task end edge
5. Drag progress indicator
6. Add visual drag preview
7. Implement snap to grid during drag
8. Enforce drag constraints
9. Add undo/redo for drag operations

---

### Phase 6: Dependencies (Week 10-11)
**ID:** `leptos-daisyui-rs-iyk.6` | **Priority:** P1 | **Tasks:** 9
**Depends on:** Phase 5

**Deliverables:**
- Dependency links as SVG bezier curves
- 4 dependency types (FS/SS/FF/SF)
- Lag time support (positive/negative)
- Drag to create dependency
- Circular dependency validation (DFS)
- Delete dependency
- Dependency hover/selection
- Auto-schedule dependent tasks
- Dependency graph utilities (petgraph)

**Tasks:**
1. Render dependency links as SVG paths
2. Calculate link paths for 4 dependency types
3. Support lag time in dependencies
4. Drag to create dependency
5. Validate dependencies (no circular)
6. Delete dependency via context menu
7. Implement dependency hover/selection
8. Auto-schedule dependent tasks
9. Add dependency graph utilities

---

### Phase 7: Advanced Features (Week 12-14)
**ID:** `leptos-daisyui-rs-iyk.7` | **Priority:** P2 | **Tasks:** 8
**Depends on:** Phase 6

**Deliverables:**
- Critical path calculation (forward/backward pass)
- Critical path highlighting
- Task filtering (status/assignee/date)
- Task search (by name)
- Progress percentage display
- Milestone rendering (diamonds)
- Resource avatars on bars
- Custom task colors

**Tasks:**
1. Implement critical path algorithm
2. Highlight critical path tasks
3. Implement task filtering
4. Add task search functionality
5. Display progress percentage
6. Render milestones as diamonds
7. Add resource avatars on task bars
8. Support custom task colors

---

### Phase 8: Performance Optimization (Week 15-16)
**ID:** `leptos-daisyui-rs-iyk.8` | **Priority:** P1 | **Tasks:** 8
**Depends on:** Phase 7

**Deliverables:**
- Virtual scrolling (IntersectionObserver)
- Lazy rendering (visible range only)
- Debounced scroll/resize (16ms/100ms)
- Memoized calculations (Memo signals)
- Canvas rendering (instead of SVG)
- Worker threads for dependency calc
- Performance benchmarks (1k/5k/10k tasks)
- Hot path optimization

**Tasks:**
1. Implement virtual scrolling for task list
2. Lazy render timeline visible range
3. Debounce scroll and resize events
4. Memoize expensive calculations
5. Convert timeline to canvas rendering
6. Use worker thread for dependency calc
7. Create performance benchmarks
8. Profile and optimize hot paths

**Performance Targets:**
- 10,000+ tasks without degradation
- Initial render < 500ms for 1,000 tasks
- 60 FPS during drag operations
- Memory < 100MB for 5,000 tasks

---

### Phase 9: Accessibility & Testing (Week 17-18)
**ID:** `leptos-daisyui-rs-iyk.9` | **Priority:** P0 | **Tasks:** 9
**Depends on:** Phase 8

**Deliverables:**
- Full ARIA implementation (role=treegrid, 15+ attributes)
- Complete keyboard navigation (25+ shortcuts)
- Screen reader announcements (aria-live)
- Focus management (2px outline, trap, restore)
- High contrast mode support
- Reduced motion support (prefers-reduced-motion)
- Unit tests (80% coverage)
- Integration tests (workflows)
- Accessibility audit (axe-core)

**Tasks:**
1. Implement full ARIA attributes
2. Complete keyboard navigation
3. Add screen reader announcements
4. Implement focus management
5. Support high contrast mode
6. Respect reduced motion preference
7. Write unit tests for core logic
8. Write integration tests
9. Run accessibility audit with axe-core

**Standards Compliance:**
- WCAG 2.2 Level AA
- Section 508
- WAI-ARIA best practices

---

### Phase 10: Demo & Documentation (Week 19-20)
**ID:** `leptos-daisyui-rs-iyk.10` | **Priority:** P1 | **Tasks:** 10
**Depends on:** Phase 9

**Deliverables:**
- Comprehensive demo page (all features)
- Demo route and menu integration
- API documentation (rustdoc)
- Usage examples (5+ scenarios)
- Migration guide (if needed)
- Performance tuning pass
- Error handling polish
- Loading/empty states
- Export functionality (JSON/CSV/PNG)
- Component_Library.md update

**Tasks:**
1. Create comprehensive demo page
2. Add route and menu item for demo
3. Write API documentation (rustdoc)
4. Create usage examples
5. Write migration guide if needed
6. Performance tuning pass
7. Polish error handling
8. Implement loading and empty states
9. Add export functionality
10. Update Component_Library.md

---

## Task Statistics

| Phase | Feature ID | Priority | Tasks | Status |
|-------|-----------|----------|-------|--------|
| Phase 1 | iyk.1 | P0 | 9 | Open |
| Phase 2 | iyk.2 | P0 | 8 | Blocked (→ Phase 1) |
| Phase 3 | iyk.3 | P1 | 6 | Blocked (→ Phase 2) |
| Phase 4 | iyk.4 | P1 | 9 | Blocked (→ Phase 3) |
| Phase 5 | iyk.5 | P1 | 9 | Blocked (→ Phase 4) |
| Phase 6 | iyk.6 | P1 | 9 | Blocked (→ Phase 5) |
| Phase 7 | iyk.7 | P2 | 8 | Blocked (→ Phase 6) |
| Phase 8 | iyk.8 | P1 | 8 | Blocked (→ Phase 7) |
| Phase 9 | iyk.9 | P0 | 9 | Blocked (→ Phase 8) |
| Phase 10 | iyk.10 | P1 | 10 | Blocked (→ Phase 9) |
| **Total** | **10 features** | | **85 tasks** | |

---

## Dependency Chain

The phases must be completed sequentially:

```
Phase 1 (Foundation)
  ↓
Phase 2 (Timeline)
  ↓
Phase 3 (Tree)
  ↓
Phase 4 (Interactivity)
  ↓
Phase 5 (Drag & Drop)
  ↓
Phase 6 (Dependencies)
  ↓
Phase 7 (Advanced)
  ↓
Phase 8 (Performance)
  ↓
Phase 9 (Accessibility)
  ↓
Phase 10 (Documentation)
```

---

## Getting Started

### View Epic Status
```bash
bd epic status leptos-daisyui-rs-iyk
```

### View Ready Tasks
```bash
bd ready
```

### Start Working on Phase 1
```bash
# View Phase 1 tasks
bd show leptos-daisyui-rs-iyk.1 --children

# Claim first task
bd update leptos-daisyui-rs-iyk.1.1 --status=in_progress --assignee=your-name

# When complete
bd close leptos-daisyui-rs-iyk.1.1
```

### View All Gantt Issues
```bash
bd list --limit=0 | grep iyk
```

---

## Implementation Resources

### Documentation
- **Specification:** `Gantt_Component.md` - Complete functional/non-functional requirements
- **This Plan:** `Gantt_Development_Plan.md` - Beads structure and task breakdown

### Reference Implementations
1. [Frappe Gantt](https://github.com/frappe/gantt) - Beautiful JavaScript library
2. [DHTMLX Gantt](https://github.com/DHTMLX/gantt) - Enterprise GPL solution
3. [SVAR React Gantt](https://svar.dev/react/gantt/) - Modern React component
4. [gantt-task-react](https://github.com/MaTeMaTuK/gantt-task-react) - TypeScript implementation

### Key Dependencies
```toml
chrono = "0.4"      # Date/time handling
petgraph = "0.6"    # Dependency graph algorithms
serde = "1.0"       # Serialization
web-sys = "0.3"     # Canvas, IntersectionObserver
```

---

## Success Criteria

- [ ] All 85 tasks completed
- [ ] All tests passing (unit, integration, accessibility)
- [ ] Performance benchmarks met (10k tasks, <500ms render, 60 FPS)
- [ ] WCAG 2.2 Level AA compliance
- [ ] Comprehensive demo page
- [ ] Full API documentation
- [ ] Component added to library catalog

---

## Notes

This is a comprehensive 20-week project requiring expertise in:
- Rust and Leptos framework
- SVG/Canvas rendering
- Complex state management
- Graph algorithms (DFS, topological sort, critical path)
- Accessibility standards (WCAG, ARIA)
- Performance optimization

The component will be the most complex in the leptos-daisyui-rs library and serve as a flagship feature for project management use cases.
