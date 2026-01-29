# Theming System - Beads Issue Structure

This document outlines the complete beads issue structure for implementing the Advanced Runtime Theming System. All issues have been created with proper dependencies and priorities.

## Epic Hierarchy

```
leptos-daisyui-rs-25p [P1] - Advanced Runtime Theming System (MAIN EPIC)
├─ leptos-daisyui-rs-dvl [P1] - Phase 1: Theming Foundation
│  ├─ leptos-daisyui-rs-dvl.1 [P2] - Define theme data structures
│  ├─ leptos-daisyui-rs-dvl.2 [P2] - Implement ThemeProvider context
│  ├─ leptos-daisyui-rs-dvl.3 [P2] - Build CSS variable injection system
│  ├─ leptos-daisyui-rs-dvl.4 [P2] - Add localStorage persistence
│  ├─ leptos-daisyui-rs-dvl.5 [P2] - Create utility functions
│  └─ leptos-daisyui-rs-dvl.6 [P2] - Write foundation tests
│
├─ leptos-daisyui-rs-3t5 [P1] - Phase 2: Base Theme Selection
│  ├─ leptos-daisyui-rs-3t5.1 [P2] - Enhance ThemeController component
│  ├─ leptos-daisyui-rs-3t5.2 [P2] - Create BaseThemeSelector component
│  ├─ leptos-daisyui-rs-3t5.3 [P2] - Implement theme switching logic
│  ├─ leptos-daisyui-rs-3t5.4 [P2] - Add transition animations
│  └─ leptos-daisyui-rs-3t5.5 [P2] - Test theme switching
│
├─ leptos-daisyui-rs-nqv [P1] - Phase 3: Typography Customization
│  ├─ leptos-daisyui-rs-nqv.1 [P2] - Create TypographyCustomizer component
│  ├─ leptos-daisyui-rs-nqv.2 [P2] - Add font family selection
│  ├─ leptos-daisyui-rs-nqv.3 [P2] - Implement font scale system
│  ├─ leptos-daisyui-rs-nqv.4 [P2] - Add Google Fonts integration
│  ├─ leptos-daisyui-rs-nqv.5 [P2] - Create typography preview
│  └─ leptos-daisyui-rs-nqv.6 [P2] - Test typography changes
│
├─ leptos-daisyui-rs-0m0 [P1] - Phase 4: Color Customization
│  ├─ leptos-daisyui-rs-0m0.1 [P2] - Create ColorCustomizer component
│  ├─ leptos-daisyui-rs-0m0.2 [P2] - Implement color picker integration
│  ├─ leptos-daisyui-rs-0m0.3 [P2] - Add hex to Oklahoma LCH conversion
│  ├─ leptos-daisyui-rs-0m0.4 [P2] - Create color palette preview
│  ├─ leptos-daisyui-rs-0m0.5 [P2] - Add color accessibility checker
│  └─ leptos-daisyui-rs-0m0.6 [P2] - Test color customization
│
├─ leptos-daisyui-rs-33x [P1] - Phase 5: Border & Spacing Customization
│  ├─ leptos-daisyui-rs-33x.1 [P2] - Create BorderCustomizer component
│  ├─ leptos-daisyui-rs-33x.2 [P2] - Add border width controls
│  ├─ leptos-daisyui-rs-33x.3 [P2] - Add border radius controls
│  ├─ leptos-daisyui-rs-33x.4 [P2] - Implement spacing scale
│  ├─ leptos-daisyui-rs-33x.5 [P2] - Add preview cards
│  └─ leptos-daisyui-rs-33x.6 [P2] - Test border and spacing changes
│
├─ leptos-daisyui-rs-y75 [P1] - Phase 6: Component-Specific Overrides
│  ├─ leptos-daisyui-rs-y75.1 [P2] - Create ComponentCustomizer component
│  ├─ leptos-daisyui-rs-y75.2 [P2] - Add Card component overrides
│  ├─ leptos-daisyui-rs-y75.3 [P2] - Add Button component overrides
│  ├─ leptos-daisyui-rs-y75.4 [P2] - Add Input component overrides
│  ├─ leptos-daisyui-rs-y75.5 [P2] - Update library components to use CSS variables
│  ├─ leptos-daisyui-rs-y75.6 [P2] - Create component showcase
│  └─ leptos-daisyui-rs-y75.7 [P2] - Verify component overrides
│
└─ leptos-daisyui-rs-bt5 [P1] - Phase 7: Import/Export & Polish
   ├─ leptos-daisyui-rs-bt5.1 [P2] - Implement JSON export
   ├─ leptos-daisyui-rs-bt5.2 [P2] - Add JSON import with validation
   ├─ leptos-daisyui-rs-bt5.3 [P2] - Create preset themes gallery
   ├─ leptos-daisyui-rs-bt5.4 [P2] - Add theme sharing functionality
   ├─ leptos-daisyui-rs-bt5.5 [P2] - Write comprehensive documentation
   ├─ leptos-daisyui-rs-bt5.6 [P2] - Create example implementations
   ├─ leptos-daisyui-rs-bt5.7 [P2] - Polish UI and UX
   └─ leptos-daisyui-rs-bt5.8 [P2] - Accessibility audit
```

## Issue Statistics

- **Main Epic**: 1
- **Phase Epics**: 7
- **Total Tasks**: 43
- **Total Issues**: 51

### Priority Breakdown
- **P1 (High Priority)**: 8 issues (1 main epic + 7 phase epics)
- **P2 (Medium Priority)**: 43 issues (all tasks)

## Dependency Structure

### Phase Dependencies

```
Phase 1 (Foundation)
    ├─ Blocks: Phase 2, 3, 4, 5, 6
    └─ Required for: All subsequent work

Phase 2 (Base Theme Selection)
    └─ Depends on: Phase 1

Phase 3 (Typography)
    └─ Depends on: Phase 1

Phase 4 (Colors)
    └─ Depends on: Phase 1

Phase 5 (Borders & Spacing)
    └─ Depends on: Phase 1

Phase 6 (Component Overrides)
    └─ Depends on: Phase 1

Phase 7 (Import/Export & Polish)
    └─ Depends on: Phases 1-6 (all previous phases)
```

### Critical Path

The critical path for implementing the theming system follows this sequence:

1. **Phase 1: Foundation** (Week 1-2)
   - Must complete all 6 tasks before any other phase can begin
   - Creates core data structures and infrastructure

2. **Phases 2-6 (Parallel Work)** (Week 2-7)
   - After Phase 1 completes, Phases 2-6 can proceed in parallel
   - Each phase focuses on a specific aspect of customization
   - Recommended order: Phase 2 → 3 → 4 → 5 → 6 (but flexible)

3. **Phase 7: Final Polish** (Week 7-8)
   - Requires all previous phases to be complete
   - Adds import/export, documentation, and final polish

## Implementation Strategy

### Recommended Workflow

1. **Start with Phase 1**
   ```bash
   bd ready  # Check for Phase 1 tasks
   bd update leptos-daisyui-rs-dvl.1 --status=in_progress
   # Work on task...
   bd close leptos-daisyui-rs-dvl.1
   ```

2. **Complete Phase 1 tasks in order**
   - Task 1 must complete first (defines data structures)
   - Tasks 2-6 depend on Task 1 but can proceed as soon as Task 1 is done

3. **Choose next phase based on priorities**
   - Phase 2 (Theme Selection) is quick and provides immediate value
   - Phase 3 (Typography) is highly visible to users
   - Phase 4 (Colors) is most requested feature
   - Phase 5 (Borders) and Phase 6 (Components) are refinements

4. **Complete Phase 7 last**
   - Requires all features to be implemented
   - Focuses on polish, documentation, and user experience

### Tracking Progress

```bash
# View all theming work
bd list --status=open | grep "Phase"

# Check ready tasks (no blockers)
bd ready

# View specific phase
bd show leptos-daisyui-rs-dvl  # Phase 1
bd show leptos-daisyui-rs-3t5  # Phase 2
# ... etc

# Check blocked issues
bd blocked

# View statistics
bd stats
```

## Task Estimates

### Phase 1: Foundation (Week 1-2)
- Task 1: 2-3 days (data structures)
- Task 2: 2-3 days (ThemeProvider)
- Task 3: 2-3 days (CSS injection)
- Task 4: 1-2 days (persistence)
- Task 5: 1-2 days (utilities)
- Task 6: 2-3 days (tests)
- **Total**: 10-16 days (2 weeks)

### Phase 2: Base Theme Selection (Week 2-3)
- Task 1: 1-2 days (enhance controller)
- Task 2: 2-3 days (selector UI)
- Task 3: 2-3 days (switching logic)
- Task 4: 1 day (animations)
- Task 5: 1 day (testing)
- **Total**: 7-10 days (1.5 weeks)

### Phase 3: Typography (Week 3-4)
- Task 1: 2-3 days (component UI)
- Task 2: 1 day (font selection)
- Task 3: 2-3 days (scale system)
- Task 4: 1-2 days (Google Fonts)
- Task 5: 1 day (preview)
- Task 6: 1 day (testing)
- **Total**: 8-11 days (1.5 weeks)

### Phase 4: Colors (Week 4-5)
- Task 1: 2-3 days (component UI)
- Task 2: 1-2 days (picker integration)
- Task 3: 2-3 days (color conversion)
- Task 4: 1-2 days (preview)
- Task 5: 2-3 days (accessibility)
- Task 6: 1 day (testing)
- **Total**: 9-14 days (2 weeks)

### Phase 5: Borders & Spacing (Week 5-6)
- Task 1: 2 days (component UI)
- Task 2: 1 day (width controls)
- Task 3: 1 day (radius controls)
- Task 4: 1-2 days (spacing scale)
- Task 5: 1 day (preview)
- Task 6: 1 day (testing)
- **Total**: 7-8 days (1.5 weeks)

### Phase 6: Component Overrides (Week 6-7)
- Task 1: 2 days (component UI)
- Task 2-4: 1 day each (3 days total)
- Task 5: 3-5 days (update components)
- Task 6: 2-3 days (showcase)
- Task 7: 1 day (verification)
- **Total**: 9-12 days (2 weeks)

### Phase 7: Polish (Week 7-8)
- Task 1-2: 1 day each (2 days total)
- Task 3: 2-3 days (preset gallery)
- Task 4: 1-2 days (sharing)
- Task 5: 3-5 days (documentation)
- Task 6: 2-3 days (examples)
- Task 7: 2-3 days (polish)
- Task 8: 2-3 days (accessibility)
- **Total**: 13-19 days (3 weeks)

### Total Estimate
- **Minimum**: 63 days (~13 weeks / 3 months)
- **Maximum**: 90 days (~18 weeks / 4.5 months)
- **Realistic**: 75 days (~15 weeks / 3.5-4 months)

## Key Milestones

1. **M1: Foundation Complete** (Week 2)
   - All Phase 1 tasks closed
   - ThemeProvider working with basic CSS variable injection
   - Can save/load themes from localStorage

2. **M2: Core Customization Complete** (Week 5)
   - Phases 2, 3, 4 complete
   - Users can switch themes, customize typography and colors
   - Live preview working

3. **M3: Full Customization Complete** (Week 7)
   - Phases 5, 6 complete
   - Users can customize borders, spacing, and per-component styles
   - All library components support CSS variables

4. **M4: Production Ready** (Week 8)
   - Phase 7 complete
   - Documentation written
   - Examples published
   - Import/export working
   - Accessibility validated

## Success Criteria

### Phase 1 Success
- [ ] All data structures compile with Serialize/Deserialize
- [ ] ThemeProvider context accessible in components
- [ ] CSS variables inject correctly to document root
- [ ] Themes persist across page reloads
- [ ] Unit tests pass with >90% coverage

### Phase 2 Success
- [ ] All 32 daisyUI themes selectable
- [ ] Theme switches preserve custom CSS variables
- [ ] Smooth transitions between themes
- [ ] Selected theme persists

### Phase 3 Success
- [ ] Font family changes apply instantly
- [ ] Font scale calculations correct
- [ ] Preview shows all text sizes accurately
- [ ] Typography persists

### Phase 4 Success
- [ ] All semantic color tokens customizable
- [ ] Hex to Oklahoma LCH conversion accurate
- [ ] Color preview shows real-time updates
- [ ] Accessibility checker identifies issues
- [ ] Colors persist

### Phase 5 Success
- [ ] Border width/radius controls work
- [ ] Spacing scale applies correctly
- [ ] Preview cards show changes
- [ ] Settings persist

### Phase 6 Success
- [ ] Component-specific overrides work
- [ ] All library components use CSS variables
- [ ] Showcase demonstrates all components
- [ ] Component overrides persist

### Phase 7 Success
- [ ] Export downloads valid JSON
- [ ] Import validates and loads themes
- [ ] Preset gallery shows curated themes
- [ ] Documentation complete and accurate
- [ ] Examples demonstrate key features
- [ ] Accessibility audit passes
- [ ] No breaking changes

## Related Documents

- **Strategy Document**: `THEMING_STRATEGY.md` - Detailed technical design
- **Main Epic**: View with `bd show leptos-daisyui-rs-25p`
- **Beads Workflow**: `CLAUDE.md` - Commands and workflow guide

## Commands Quick Reference

```bash
# View main epic
bd show leptos-daisyui-rs-25p

# View specific phase
bd show leptos-daisyui-rs-dvl    # Phase 1
bd show leptos-daisyui-rs-3t5    # Phase 2
bd show leptos-daisyui-rs-nqv    # Phase 3
bd show leptos-daisyui-rs-0m0    # Phase 4
bd show leptos-daisyui-rs-33x    # Phase 5
bd show leptos-daisyui-rs-y75    # Phase 6
bd show leptos-daisyui-rs-bt5    # Phase 7

# Find ready tasks
bd ready

# Start work on a task
bd update <task-id> --status=in_progress

# Complete a task
bd close <task-id>

# View all open theming issues
bd list --status=open

# View project statistics
bd stats

# Check for blocked issues
bd blocked
```

## Notes

- All issues are created in "open" status
- Priorities: P1 for epics, P2 for tasks
- Dependencies properly configured to enforce workflow
- Phase 1 is the critical path - everything else depends on it
- Phases 2-6 can proceed in parallel after Phase 1
- Phase 7 requires all previous phases complete
- All tasks have detailed descriptions in beads system
- Use `bd show <issue-id>` to view full details
