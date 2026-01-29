# Automated Test Suite for Demo App
## Execute with: Task tool or direct Chrome DevTools MCP commands

This file contains **copy-paste ready commands** for automated testing.

---

## 🚀 Prerequisites

1. Start demo server:
   ```powershell
   .\launcher.ps1 -Task dev
   ```

2. Wait for server to be ready at: http://127.0.0.1:3000

---

## 📋 Test Suite Execution

### Setup Phase

Copy and execute these commands in sequence:

```javascript
// Command 1: List pages
list_pages()
```

Wait for result, then:

```javascript
// Command 2: Select page (use pageIdx from list_pages result)
select_page({ pageIdx: 0 })
```

```javascript
// Command 3: Set desktop resolution
resize_page({ width: 1920, height: 1080 })
```

```javascript
// Command 4: Navigate to demo
navigate_page({ type: "url", url: "http://127.0.0.1:3000" })
```

```javascript
// Command 5: Verify landing page loaded
wait_for({ text: "leptos-daisyui-rs", timeout: 5000 })
```

```javascript
// Command 6: Take baseline snapshot
take_snapshot()
```

```javascript
// Command 7: Check for console errors
list_console_messages({ types: ["error"] })
```

✅ **Expected**: No errors, landing page visible

---

## 🧪 Component Testing (62 Components)

### Component Test Template

For each component, run these 4 commands:

```javascript
// 1. Navigate to component
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/COMPONENT_NAME" })

// 2. Wait for content to load
wait_for({ text: "COMPONENT_DISPLAY_NAME", timeout: 3000 })

// 3. Take snapshot
take_snapshot()

// 4. Check for errors
list_console_messages({ types: ["error"] })
```

---

### Action Category (6 components)

#### Button
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/button" })
wait_for({ text: "Button", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Dropdown
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/dropdown" })
wait_for({ text: "Dropdown", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### FAB
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/fab" })
wait_for({ text: "FAB", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Modal
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/modal" })
wait_for({ text: "Modal", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Swap
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/swap" })
wait_for({ text: "Swap", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Theme Controller
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/theme_controller" })
wait_for({ text: "Theme", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

---

### Data Display Category (18 components)

#### Accordion
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/accordion" })
wait_for({ text: "Accordion", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Avatar
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/avatar" })
wait_for({ text: "Avatar", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Badge
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/badge" })
wait_for({ text: "Badge", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Card
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/card" })
wait_for({ text: "Card", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Carousel
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/carousel" })
wait_for({ text: "Carousel", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Chat
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/chat" })
wait_for({ text: "Chat", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Collapse
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/collapse" })
wait_for({ text: "Collapse", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Countdown
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/countdown" })
wait_for({ text: "Countdown", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Diff
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/diff" })
wait_for({ text: "Diff", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Hover 3D
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/hover_3d" })
wait_for({ text: "Hover", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Hover Gallery
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/hover_gallery" })
wait_for({ text: "Gallery", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Kbd
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/kbd" })
wait_for({ text: "Kbd", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### List
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/list" })
wait_for({ text: "List", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Stats
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/stats" })
wait_for({ text: "Stats", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Status
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/status" })
wait_for({ text: "Status", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Table
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/table" })
wait_for({ text: "Table", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Text Rotate
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/text_rotate" })
wait_for({ text: "Rotate", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Timeline
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/timeline" })
wait_for({ text: "Timeline", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

---

### Navigation Category (6 components)

#### Breadcrumbs
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/breadcrumbs" })
wait_for({ text: "Breadcrumbs", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Menu
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/menu" })
wait_for({ text: "Menu", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Navbar
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/navbar" })
wait_for({ text: "Navbar", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Pagination
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/pagination" })
wait_for({ text: "Pagination", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Steps
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/steps" })
wait_for({ text: "Steps", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Tab
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/tab" })
wait_for({ text: "Tab", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

---

### Feedback Category (7 components)

#### Alert
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/alert" })
wait_for({ text: "Alert", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Loading
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/loading" })
wait_for({ text: "Loading", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Progress
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/progress" })
wait_for({ text: "Progress", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Radial Progress
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/radial_progress" })
wait_for({ text: "Radial", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Skeleton
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/skeleton" })
wait_for({ text: "Skeleton", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Toast
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/toast" })
wait_for({ text: "Toast", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Tooltip
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/tooltip" })
wait_for({ text: "Tooltip", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

---

### Data Input Category (14 components)

#### Calendar
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/calendar" })
wait_for({ text: "Calendar", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Checkbox
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/checkbox" })
wait_for({ text: "Checkbox", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Fieldset
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/fieldset" })
wait_for({ text: "Fieldset", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### File Input
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/file_input" })
wait_for({ text: "File", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Filter
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/filter" })
wait_for({ text: "Filter", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Input
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/input" })
wait_for({ text: "Input", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Label
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/label" })
wait_for({ text: "Label", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Radio
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/radio" })
wait_for({ text: "Radio", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Range
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/range" })
wait_for({ text: "Range", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Rating
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/rating" })
wait_for({ text: "Rating", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Select
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/select" })
wait_for({ text: "Select", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Textarea
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/textarea" })
wait_for({ text: "Textarea", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Toggle
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/toggle" })
wait_for({ text: "Toggle", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Validator
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/validator" })
wait_for({ text: "Validator", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

---

### Layout Category (10 components)

#### Divider
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/divider" })
wait_for({ text: "Divider", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Dock
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/dock" })
wait_for({ text: "Dock", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Drawer
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/drawer" })
wait_for({ text: "Drawer", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Footer
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/footer" })
wait_for({ text: "Footer", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Hero
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/hero" })
wait_for({ text: "Hero", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Indicator
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/indicator" })
wait_for({ text: "Indicator", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Join
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/join" })
wait_for({ text: "Join", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Link
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/link" })
wait_for({ text: "Link", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Mask
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/mask" })
wait_for({ text: "Mask", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Stack
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/stack" })
wait_for({ text: "Stack", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

---

### Mockup Category (4 components)

#### Mockup Browser
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/mockup_browser" })
wait_for({ text: "Mockup", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Mockup Code
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/mockup_code" })
wait_for({ text: "Mockup", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Mockup Phone
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/mockup_phone" })
wait_for({ text: "Mockup", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

#### Mockup Window
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/mockup_window" })
wait_for({ text: "Mockup", timeout: 3000 })
take_snapshot()
list_console_messages({ types: ["error"] })
```

---

## 📱 Mobile Responsiveness Test

```javascript
// Switch to mobile viewport
resize_page({ width: 375, height: 667 })
```

```javascript
// Navigate to components
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/button" })
```

```javascript
// Take mobile snapshot
take_snapshot({ verbose: true })
```

```javascript
// Check if hamburger menu is visible
// Look for drawer-toggle in snapshot, then click it
```

```javascript
// Switch back to desktop
resize_page({ width: 1920, height: 1080 })
```

---

## 🎨 Theme Testing

```javascript
// Navigate to theme controller
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/theme_controller" })
```

```javascript
// Check current theme
evaluate_script({
  function: "() => document.documentElement.getAttribute('data-theme')"
})
```

```javascript
// Take snapshot to find theme switcher UIDs
take_snapshot({ verbose: true })
```

```javascript
// After clicking theme button, verify theme changed
evaluate_script({
  function: "() => document.documentElement.getAttribute('data-theme')"
})
```

---

## ✅ Verification Commands

After running all tests, verify results:

```javascript
// Check total console errors across session
list_console_messages({ types: ["error"], includePreservedMessages: true })
```

```javascript
// Check network errors
list_network_requests({ resourceTypes: ["document", "script", "stylesheet"] })
```

```javascript
// Get performance data
evaluate_script({
  function: `() => {
    const perfData = performance.getEntriesByType("navigation")[0];
    return {
      domComplete: Math.round(perfData.domComplete),
      loadTime: Math.round(perfData.loadEventEnd - perfData.fetchStart),
      resources: performance.getEntriesByType("resource").length
    };
  }`
})
```

---

## 📊 Expected Results

### Success Criteria
- ✅ All 62 components load without errors
- ✅ Console shows 0 critical errors
- ✅ All snapshots contain expected elements
- ✅ Navigation menu shows 7 categories
- ✅ Theme switching changes data-theme attribute
- ✅ Mobile viewport displays hamburger menu

### Common Issues to Watch For
- ⚠️ 404 errors on component routes
- ⚠️ Missing CSS classes (Tailwind not compiled)
- ⚠️ JavaScript errors in console
- ⚠️ Timeout waiting for elements
- ⚠️ Blank pages or missing content

---

## 🤖 Automated Execution

To run all tests automatically, ask Claude:

```
"Please execute all component tests from run-automated-tests.md
sequentially, starting with the setup phase, then test all 62
components across all 7 categories. Report any console errors
or failures."
```

Or use Task tool:

```
"Use the general-purpose agent to systematically test all 62
daisyUI components in the demo app using Chrome DevTools MCP,
following the commands in run-automated-tests.md"
```

---

**Total Commands**: ~200 (4 per component × 62 components + setup/verification)
**Estimated Time**: 30-45 minutes for full automated run
**Output**: Component validation status + console error report

