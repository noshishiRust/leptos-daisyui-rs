# Demo App Test Execution Guide
## Quick Start Testing with Chrome DevTools MCP

This is the **executable companion** to `TESTING_PLAN.md`. Use this for hands-on testing.

---

## 🚀 Quick Start (5 Minutes)

### 1. Start the Demo Server
```powershell
.\launcher.ps1 -Task dev
# Wait for: "Your application is available at http://127.0.0.1:3000"
```

### 2. Basic Smoke Test
Run these commands through Claude Code with Chrome DevTools MCP:

```javascript
// Step 1: List and select page
list_pages()
select_page({ pageIdx: 0 })

// Step 2: Navigate to demo
navigate_page({ type: "url", url: "http://127.0.0.1:3000" })

// Step 3: Verify landing page loads
wait_for({ text: "leptos-daisyui-rs", timeout: 5000 })
take_snapshot()

// Step 4: Take a screenshot
take_screenshot({ filePath: "landing-page.png" })

// Step 5: Check for errors
list_console_messages({ types: ["error"] })
```

**Success**: You should see the landing page with no console errors.

---

## 🧪 Test Phases

### Phase 1: Navigation Test (2 minutes)

Test that all 62 component routes load:

```javascript
// Navigate to components section
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/button" })
take_snapshot()

// Check sidebar menu is visible
wait_for({ text: "Action" })
wait_for({ text: "Data Display" })
wait_for({ text: "Navigation" })
wait_for({ text: "Feedback" })
wait_for({ text: "Data Input" })
wait_for({ text: "Layout" })
wait_for({ text: "Mockup" })

// Take screenshot of layout
take_screenshot({ filePath: "layout-with-menu.png" })
```

### Phase 2: Sample Component Tests (10 minutes)

Test one component from each category:

#### 2.1 Action: Button
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/button" })
wait_for({ text: "Button", timeout: 3000 })
take_snapshot()

// Look for button with specific UID from snapshot, then click it
// click({ uid: "button-uid-from-snapshot" })

take_screenshot({ filePath: "button-component.png", fullPage: true })
list_console_messages({ types: ["error"] })
```

#### 2.2 Data Display: Card
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/card" })
wait_for({ text: "Card", timeout: 3000 })
take_snapshot()
take_screenshot({ filePath: "card-component.png", fullPage: true })
list_console_messages({ types: ["error"] })
```

#### 2.3 Navigation: Menu
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/menu" })
wait_for({ text: "Menu", timeout: 3000 })
take_snapshot()
take_screenshot({ filePath: "menu-component.png", fullPage: true })
list_console_messages({ types: ["error"] })
```

#### 2.4 Feedback: Alert
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/alert" })
wait_for({ text: "Alert", timeout: 3000 })
take_snapshot()
take_screenshot({ filePath: "alert-component.png", fullPage: true })
list_console_messages({ types: ["error"] })
```

#### 2.5 Data Input: Input
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/input" })
wait_for({ text: "Input", timeout: 3000 })
take_snapshot()
take_screenshot({ filePath: "input-component.png", fullPage: true })
list_console_messages({ types: ["error"] })
```

#### 2.6 Layout: Drawer
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/drawer" })
wait_for({ text: "Drawer", timeout: 3000 })
take_snapshot()
take_screenshot({ filePath: "drawer-component.png", fullPage: true })
list_console_messages({ types: ["error"] })
```

#### 2.7 Mockup: Mockup Browser
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/mockup_browser" })
wait_for({ text: "Mockup", timeout: 3000 })
take_snapshot()
take_screenshot({ filePath: "mockup-browser-component.png", fullPage: true })
list_console_messages({ types: ["error"] })
```

### Phase 3: Interactive Component Tests (15 minutes)

#### 3.1 Modal Test
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/modal" })
wait_for({ text: "Modal", timeout: 3000 })
take_snapshot({ verbose: true })

// Find button UID from snapshot to open modal
// Example: click({ uid: "xyz123" })

// After modal opens:
take_screenshot({ filePath: "modal-open.png" })

// Close with ESC
press_key({ key: "Escape" })
take_screenshot({ filePath: "modal-closed.png" })

list_console_messages({ types: ["error"] })
```

#### 3.2 Dropdown Test
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/dropdown" })
wait_for({ text: "Dropdown", timeout: 3000 })
take_snapshot({ verbose: true })

// Click dropdown to open (find UID from snapshot)
// click({ uid: "dropdown-uid" })

take_screenshot({ filePath: "dropdown-open.png" })

list_console_messages({ types: ["error"] })
```

#### 3.3 Theme Controller Test
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/theme_controller" })
wait_for({ text: "Theme", timeout: 3000 })
take_snapshot({ verbose: true })

// Check current theme
evaluate_script({
  function: "() => document.documentElement.getAttribute('data-theme')"
})

take_screenshot({ filePath: "theme-controller.png" })

// After clicking theme switch button:
// take_screenshot({ filePath: "theme-dark.png" })

list_console_messages({ types: ["error"] })
```

#### 3.4 FAB Test (New Component)
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/fab" })
wait_for({ text: "FAB", timeout: 3000 })
take_snapshot({ verbose: true })

// Click FAB to open speed dial
// click({ uid: "fab-uid" })

take_screenshot({ filePath: "fab-open.png" })

list_console_messages({ types: ["error"] })
```

#### 3.5 Hover 3D Test (New Component)
```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/hover_3d" })
wait_for({ text: "Hover", timeout: 3000 })
take_snapshot({ verbose: true })

// Hover over 3D element
// hover({ uid: "hover3d-uid" })

take_screenshot({ filePath: "hover-3d.png" })

list_console_messages({ types: ["error"] })
```

### Phase 4: Mobile Responsiveness Test (5 minutes)

```javascript
// Switch to mobile viewport (iPhone SE)
resize_page({ width: 375, height: 667 })

// Navigate to components
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/button" })
take_snapshot()

// Hamburger menu should be visible
take_screenshot({ filePath: "mobile-menu-closed.png" })

// Click hamburger to open drawer (find UID from snapshot)
// click({ uid: "drawer-toggle-uid" })

take_screenshot({ filePath: "mobile-menu-open.png" })

// Navigate to another component from mobile menu
// click({ uid: "card-menu-item-uid" })

take_screenshot({ filePath: "mobile-component-view.png" })

// Switch back to desktop
resize_page({ width: 1920, height: 1080 })
```

### Phase 5: Performance Check (5 minutes)

```javascript
// Check Core Web Vitals on landing page
navigate_page({ type: "url", url: "http://127.0.0.1:3000" })

// Get performance metrics
evaluate_script({
  function: `() => {
    const perfData = performance.getEntriesByType("navigation")[0];
    return {
      domContentLoaded: perfData.domContentLoadedEventEnd - perfData.domContentLoadedEventStart,
      loadComplete: perfData.loadEventEnd - perfData.fetchStart,
      domInteractive: perfData.domInteractive - perfData.fetchStart,
      resourcesLoaded: performance.getEntriesByType("resource").length
    };
  }`
})

// Check for network errors
list_network_requests({ resourceTypes: ["document", "script", "stylesheet"] })

// Performance trace (optional - takes time)
performance_start_trace({ reload: true, autoStop: true })
// Wait for trace to complete...
performance_stop_trace()
```

---

## 🔄 Bulk Component Validation

### Test All 62 Components (Sequential)

For automated validation of all components at once:

```javascript
// Component list (all 62)
const components = [
  "accordion", "alert", "avatar", "badge", "breadcrumbs", "button",
  "calendar", "card", "carousel", "chat", "checkbox", "collapse",
  "countdown", "diff", "divider", "dock", "drawer", "dropdown",
  "fab", "fieldset", "file_input", "filter", "footer", "hero",
  "hover_3d", "hover_gallery", "indicator", "input", "join", "kbd",
  "label", "link", "list", "loading", "mask", "menu", "mockup_browser",
  "mockup_code", "mockup_phone", "mockup_window", "modal", "navbar",
  "pagination", "progress", "radial_progress", "radio", "range",
  "rating", "select", "skeleton", "stack", "stats", "status", "steps",
  "swap", "tab", "table", "text_rotate", "textarea", "theme_controller",
  "timeline", "toast", "toggle", "tooltip", "validator"
];

// For each component, run:
for (const component of components) {
  // Navigate
  navigate_page({
    type: "url",
    url: `http://127.0.0.1:3000/components/${component}`,
    timeout: 10000
  });

  // Wait for load
  // (Note: Claude will need to execute these one at a time)

  // Take screenshot
  take_screenshot({
    filePath: `screenshots/${component}.png`,
    fullPage: true
  });

  // Check errors
  const errors = list_console_messages({ types: ["error"] });
  // Log results
  console.log(`${component}: ${errors.length} errors`);
}
```

**Note**: Since Chrome DevTools MCP doesn't support JavaScript loops directly, you'll need to either:
1. Ask Claude to iterate through components using Task tool
2. Run commands manually for each component
3. Use the automation agent approach

---

## 🤖 Automated Testing with Task Agent

For fully automated testing, ask Claude to:

```
"Use the Task tool with the general-purpose agent to systematically test
all 62 components in the demo app at http://127.0.0.1:3000, taking
screenshots and checking for console errors on each page."
```

The agent will:
1. Navigate through all component routes
2. Capture screenshots
3. Log console errors
4. Generate a summary report

---

## ✅ Validation Checklist

After running tests, verify:

### Critical Tests
- [ ] Landing page loads without errors
- [ ] All 62 component routes are accessible
- [ ] Navigation menu displays all categories
- [ ] Mobile drawer opens/closes correctly
- [ ] Theme switching works
- [ ] No critical console errors

### Interactive Tests
- [ ] Modal opens and closes (ESC key works)
- [ ] Dropdown opens on click
- [ ] Theme controller changes theme attribute
- [ ] FAB speed dial expands
- [ ] Drawer toggle works on mobile
- [ ] Form inputs accept text

### Visual Tests
- [ ] Components render as expected
- [ ] No layout shifts
- [ ] Responsive breakpoints work
- [ ] All colors/variants display
- [ ] Icons load correctly

---

## 📊 Results Template

After testing, record results:

```markdown
## Test Results - [Date]

### Summary
- **Total Components**: 62
- **Tested**: 62
- **Passed**: X
- **Failed**: Y
- **Errors Found**: Z

### Failed Components
1. Component Name - Issue description
2. ...

### Console Errors
1. Page: /components/X - Error: "..."
2. ...

### Performance
- Landing page load: X ms
- Average component load: Y ms
- Network requests: Z

### Screenshots
- Total captured: X
- Location: ./screenshots/

### Recommendations
1. Fix issue X in component Y
2. Optimize Z for better performance
3. ...
```

---

## 🔧 Troubleshooting

### Issue: "Page not found"
**Solution**: Ensure demo server is running on port 3000

### Issue: "Timeout waiting for element"
**Solution**: Increase timeout, check if component rendered

### Issue: "Cannot find element UID"
**Solution**: Take fresh snapshot to get current UIDs

### Issue: "Console errors on all pages"
**Solution**: Check Trunk build, verify Tailwind CSS compiled

### Issue: "Screenshots not saving"
**Solution**: Create screenshots directory first

---

## 📝 Quick Reference

### Essential Commands

```javascript
// List pages
list_pages()

// Select page
select_page({ pageIdx: 0 })

// Navigate
navigate_page({ type: "url", url: "http://..." })

// Wait for text
wait_for({ text: "Button", timeout: 5000 })

// Take snapshot
take_snapshot({ verbose: true })

// Take screenshot
take_screenshot({ filePath: "output.png", fullPage: true })

// Click element
click({ uid: "element-uid-from-snapshot" })

// Fill input
fill({ uid: "input-uid", value: "test text" })

// Press key
press_key({ key: "Escape" })
press_key({ key: "Control+A" })

// Hover
hover({ uid: "element-uid" })

// Resize window
resize_page({ width: 1920, height: 1080 })

// Check console
list_console_messages({ types: ["error", "warn"] })

// Get console message details
get_console_message({ msgid: 123 })

// Check network
list_network_requests({ resourceTypes: ["xhr", "fetch"] })

// Execute JavaScript
evaluate_script({ function: "() => document.title" })
```

---

## 🎯 Next Steps

1. Run the Quick Start smoke test
2. Execute Phase 2 sample tests (one per category)
3. Test interactive components in Phase 3
4. Validate mobile responsiveness
5. Run bulk validation for all 62 components
6. Generate test report

**Estimated Time**:
- Quick smoke test: 5 min
- Sample tests: 10 min
- Interactive tests: 15 min
- Mobile test: 5 min
- Full validation: 30 min

**Total**: ~1 hour for comprehensive validation

---

**Version**: 1.0
**Compatible with**: Chrome DevTools MCP
**Demo URL**: http://127.0.0.1:3000
