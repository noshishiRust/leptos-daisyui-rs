# Demo Application QA Issues

This document tracks all issues discovered during visual testing of the leptos-daisyui-rs demo application.

**Testing Date**: 2026-01-29
**Total Issues Found**: 33
**Issues Created in Beads**: 33
**Issues Resolved**: 3 (75e, egz, 5sc)

## Recent Fixes (2026-01-29)

### ✅ Accordion Issues Resolved
- **leptos-daisyui-rs-75e**: Accordion toggle functionality - Added checkbox input type support for proper toggle behavior
- **leptos-daisyui-rs-egz**: Plus indicator symbol - Fixed CSS class typo ("collapse-opem" → "collapse-open")

See `.github-issues/ACCORDION_FIXES_2026-01-29.md` for detailed fix documentation.

### ✅ Dock Examples Enhanced
- **leptos-daisyui-rs-5sc**: Dock examples and labels - Added 3 new practical examples (badges/notifications, tooltips, various icon sets), improved section titles and descriptions

See `.github-issues/DOCK_EXAMPLES_COMPLETE_2026-01-29.md` for detailed enhancement documentation.

## Issue Summary by Priority

### P0 - Critical (13 issues)
Issues that prevent components from being usable or visible.

| Issue ID | Component | Description |
|----------|-----------|-------------|
| leptos-daisyui-rs-ep8 | List | List page shows no content |
| leptos-daisyui-rs-w5x | Status | Status page shows no content |
| leptos-daisyui-rs-zrl | Fieldset | Fieldset page shows no content |
| leptos-daisyui-rs-u1q | Filter | Filter page shows no content |
| leptos-daisyui-rs-nqz | Label | Label page shows no content |
| leptos-daisyui-rs-460 | Validator | Validator page shows no content |
| leptos-daisyui-rs-dn5 | Link | Link page shows no content |
| leptos-daisyui-rs-dve | Mask | Mask page shows no content |
| leptos-daisyui-rs-77x | Stack | Stack page shows no content |
| leptos-daisyui-rs-1lk | Pagination | Components do not update selection on click |
| leptos-daisyui-rs-pt4 | Pagination | Table pagination displays incorrect values |
| leptos-daisyui-rs-dzz | Navbar | Components only show letter 's' |
| leptos-daisyui-rs-em5 | Indicator | All badges in top-right, not showing various positions |

### P1 - High (9 issues remaining, 2 resolved)
Functional bugs and unimplemented features that impact UX.

| Issue ID | Component | Description |
|----------|-----------|-------------|
| leptos-daisyui-rs-8yb | Range | Implement Range component demo |
| leptos-daisyui-rs-3sx | Divider | Implement Divider component demo |
| leptos-daisyui-rs-n7w | Button | Link buttons cause unwanted page scroll |
| leptos-daisyui-rs-0rc | Carousel | Indicator buttons cause page scroll |
| leptos-daisyui-rs-1zh | Menu | Component selections cause unwanted page scroll |
| leptos-daisyui-rs-rlk | Footer | Social icons cause page scroll on hover/click |
| leptos-daisyui-rs-dzw | Dropdown | Top Dropdown opens below instead of above |
| leptos-daisyui-rs-ugi | FAB | Flower Layout shows buttons off-screen |
| ~~leptos-daisyui-rs-75e~~ | Accordion | ~~Does not close when clicking up-carat~~ ✅ FIXED |
| ~~leptos-daisyui-rs-egz~~ | Accordion | ~~Plus indicator shows wrong symbol~~ ✅ FIXED |
| leptos-daisyui-rs-8jv | Drawer | Close animation shows at wrong position |

### P2 - Medium (3 issues remaining, 1 resolved)
Enhancements that improve UX and visual feedback.

| Issue ID | Component | Description |
|----------|-----------|-------------|
| leptos-daisyui-rs-ow9 | Button | Enhance button push-depth for more pronounced effect |
| leptos-daisyui-rs-96g | Carousel | Add auto-play timer to Direction Control |
| leptos-daisyui-rs-7gd | File Input | Add image preview to Profile Picture Upload |
| ~~leptos-daisyui-rs-5sc~~ | Dock | ~~Add more examples and improve labels~~ ✅ FIXED |

### P3 - Low (1 issue)
Nice-to-have features.

| Issue ID | Component | Description |
|----------|-----------|-------------|
| leptos-daisyui-rs-f5j | Demo App | Add footer status bar with debug info |

## Issues by Category

### Empty/Invisible Pages (9 issues - P0)
Components exist but pages show no content:
- List (ep8)
- Status (w5x)
- Fieldset (zrl)
- Filter (u1q)
- Label (nqz)
- Validator (460)
- Link (dn5)
- Mask (dve)
- Stack (77x)

**Root Cause**: Likely rendering issues, missing imports, or incorrect component usage.

**Action**: Read each demo file and component implementation to diagnose why nothing renders.

### Focus/Scroll Issues (5 issues - P1)
Components trigger unwanted page scrolling:
- Button link variants (n7w)
- Carousel indicators (0rc)
- Menu selections (1zh)
- Footer social icons (rlk)

**Root Cause**: Likely default anchor behavior or scrollIntoView() without proper options.

**Action**: Prevent default on anchor clicks, add `block: "nearest"` to scrollIntoView calls.

### Positioning Issues (3 issues - P1)
Components appear in wrong location:
- Dropdown Top direction (dzw)
- FAB Flower Layout off-screen (ugi)
- Indicator badges all top-right (em5)

**Root Cause**: Incorrect positioning CSS or logic.

**Action**: Review CSS positioning classes and JavaScript positioning logic.

### Functional Issues (2 issues - P0, 2 resolved)
Components don't work as expected:
- Pagination not updating (1lk, pt4)
- ~~Accordion toggle broken (75e)~~ ✅ FIXED
- Navbar rendering only 's' (dzz)

**Root Cause**: State management issues or event handlers not wired correctly.

**Action**: Debug event handlers and reactive signals.

**Fixed**: Accordion now supports both Radio (traditional) and Checkbox (toggle) input types.

### Rendering Issues (1 issue - P0, 1 resolved)
Visual display problems:
- ~~Accordion symbol encoding (egz)~~ ✅ FIXED
- Navbar showing only letter 's' (dzz)

**Root Cause**: Character encoding issues or CSS problems.

**Action**: Check Unicode characters and font rendering.

**Fixed**: Typo in CSS class name corrected ("collapse-opem" → "collapse-open").

### Animation Issues (1 issue - P1)
- Drawer close animation at wrong position (8jv)

**Root Cause**: Animation origin not set correctly.

**Action**: Review CSS transform-origin and animation positioning.

### Unimplemented Features (2 issues - P1)
Components showing "coming soon" placeholders:
- Range component (8yb)
- Divider component (3sx)

**Action**: Implement demo pages for these components.

### Enhancement Requests (4 issues remaining, 1 resolved - P2/P3)
- Button push depth (ow9)
- Carousel auto-play (96g)
- File Input preview (7gd)
- ~~Dock examples (5sc)~~ ✅ FIXED
- Demo footer status bar (f5j)

## Quick Start Commands

### View All QA Issues
```bash
bd list --status=open | grep -E "(ep8|w5x|zrl|u1q|nqz|460|dn5|dve|77x|1lk|pt4|dzz|em5|8yb|3sx|n7w|0rc|1zh|rlk|dzw|ugi|75e|egz|8jv|ow9|96g|7gd|5sc|f5j)"
```

### View By Priority
```bash
# Critical issues
bd list --status=open --priority=0

# High priority
bd list --status=open --priority=1

# Medium priority
bd list --status=open --priority=2

# Low priority
bd list --status=open --priority=3
```

### View Specific Issue
```bash
bd show leptos-daisyui-rs-<id>
```

### Start Working on Issue
```bash
bd update leptos-daisyui-rs-<id> --status=in_progress
```

### Close Fixed Issue
```bash
bd close leptos-daisyui-rs-<id>
```

## Recommended Fix Order

### Phase 1: Critical Empty Pages (P0)
Fix the 9 empty/invisible pages first as they represent complete failures:
1. List, Status, Fieldset (Data Display category)
2. Filter, Label, Validator (Data Input category)
3. Link, Mask, Stack (Layout category)

**Estimated Time**: 1-2 days (likely simple rendering issues once root cause found)

### Phase 2: Functional Bugs (P0)
Fix non-functional components:
1. Pagination selection not updating (1lk, pt4)
2. Navbar rendering issue (dzz)
3. Indicator positioning (em5)

**Estimated Time**: 1-2 days

### Phase 3: Unimplemented Components (P1)
Complete the placeholder components:
1. Range component demo (8yb)
2. Divider component demo (3sx)

**Estimated Time**: 1 day

### Phase 4: Focus/Scroll Issues (P1)
Fix unwanted scrolling (affects UX but not functionality):
1. Button links (n7w)
2. Carousel indicators (0rc)
3. Menu selections (1zh)
4. Footer icons (rlk)

**Estimated Time**: 1 day (likely same root cause for all)

### Phase 5: Positioning & Functional Bugs (P1)
Fix remaining P1 issues:
1. Dropdown direction (dzw)
2. FAB positioning (ugi)
3. Accordion toggle (75e)
4. Accordion symbol (egz)
5. Drawer animation (8jv)

**Estimated Time**: 1-2 days

### Phase 6: Enhancements (P2/P3)
Implement nice-to-have features:
1. Button depth (ow9)
2. Carousel auto-play (96g)
3. File Input preview (7gd)
4. Dock examples (5sc)
5. Demo footer (f5j)

**Estimated Time**: 2-3 days

## Total Estimated Time

- **Critical Path (P0 + P1)**: 5-7 days
- **With Enhancements (P2/P3)**: 7-10 days

## Testing Checklist

After fixes, verify each component:

### Empty Pages
- [ ] List page shows list examples
- [ ] Status page shows status indicators
- [ ] Fieldset page shows fieldset examples
- [ ] Filter page shows filter controls
- [ ] Label page shows label examples
- [ ] Validator page shows validation examples
- [ ] Link page shows link variants
- [ ] Mask page shows mask shapes
- [ ] Stack page shows stacked elements

### Functional Components
- [ ] Pagination updates active state on click
- [ ] Table pagination shows correct values
- [ ] Navbar renders completely
- [ ] Indicator badges show in all positions
- [ ] Range slider works (after implementation)
- [ ] Divider renders correctly (after implementation)

### Focus/Scroll Issues
- [ ] Button links don't scroll page
- [ ] Carousel indicators don't scroll page
- [ ] Menu selections don't scroll page
- [ ] Footer icons don't scroll page

### Positioning
- [ ] Top Dropdown opens above button
- [ ] FAB Flower Layout stays on screen
- [x] Accordion toggle works correctly ✅
- [x] Accordion symbols display correctly ✅
- [ ] Drawer animation closes at correct position

### Enhancements
- [ ] Button push effect is more pronounced
- [ ] Carousel auto-plays (if implemented)
- [ ] File Input shows image preview (if implemented)
- [x] Dock has sufficient examples ✅
- [ ] Demo footer shows debug info (if implemented)

## Notes

- All issues have been created in beads system
- Issues are tagged by priority (P0-P3)
- Many issues may share root causes (especially scroll issues)
- Empty page issues likely have a common pattern to fix
- Focus/scroll issues all likely need same fix (scrollIntoView options)

## Related Files

- Demo files: `demo/src/demos/*.rs`
- Component files: `src/components/*/component.rs`
- Core layout: `demo/src/core/layout.rs`
- Main app: `demo/src/main.rs`
