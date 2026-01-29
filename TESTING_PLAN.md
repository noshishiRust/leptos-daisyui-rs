# Comprehensive Demo App Testing Plan
## Using Chrome DevTools MCP & Claude Chrome Extension

**Target**: 100% component coverage validation
**Components**: 62 daisyUI components across 7 categories
**Test Environment**: http://127.0.0.1:3000

---

## 🎯 Testing Strategy Overview

This plan validates all 62 components through automated browser testing using Chrome DevTools MCP. The tests verify:

1. **Visual Rendering**: Components display correctly
2. **Interactive Behavior**: Clicks, hovers, inputs work as expected
3. **Responsive Layout**: Mobile/desktop drawer behavior
4. **Navigation**: All routes accessible
5. **Theme Support**: Light/dark theme switching
6. **Console Errors**: No JavaScript errors during interaction

---

## 📋 Pre-Test Setup

### 1. Start Demo Server
```bash
.\launcher.ps1 -Task dev
# Or: cargo make dev
# Or: cd demo && trunk serve
```

### 2. Initialize Chrome DevTools Session
```javascript
// Open browser and navigate to demo
list_pages()
select_page(0)
navigate_page({ type: "url", url: "http://127.0.0.1:3000" })
take_snapshot()
```

### 3. Verify Landing Page
```javascript
// Should see hero with title and badges
wait_for({ text: "leptos-daisyui-rs" })
take_screenshot()
```

---

## 🧪 Test Execution Plan

### Phase 1: Navigation & Layout Testing (15 min)

#### Test 1.1: Landing Page
- [ ] Verify hero renders
- [ ] Check "Browse Components" button
- [ ] Check "View on GitHub" link
- [ ] Verify badges display (Leptos, Tailwind CSS, daisyUI)

```javascript
// Navigate to landing
navigate_page({ type: "url", url: "http://127.0.0.1:3000" })
take_snapshot()

// Click Browse Components button
click({ uid: "browse-components-button-uid" })
wait_for({ text: "Button" })
```

#### Test 1.2: Desktop Layout
- [ ] Verify navbar renders
- [ ] Check sidebar menu is visible (desktop)
- [ ] Verify all 7 categories load
- [ ] Count menu items (should be 62)

```javascript
// Desktop view (1920x1080)
resize_page({ width: 1920, height: 1080 })
take_snapshot({ verbose: true })

// Verify menu categories
wait_for({ text: "Action" })
wait_for({ text: "Data Display" })
wait_for({ text: "Navigation" })
wait_for({ text: "Feedback" })
wait_for({ text: "Data Input" })
wait_for({ text: "Layout" })
wait_for({ text: "Mockup" })
```

#### Test 1.3: Mobile Layout
- [ ] Verify drawer is hidden
- [ ] Click hamburger menu
- [ ] Verify drawer opens
- [ ] Navigate to component
- [ ] Verify drawer closes

```javascript
// Mobile view (375x667 - iPhone SE)
resize_page({ width: 375, height: 667 })
take_snapshot()

// Open drawer
click({ uid: "drawer-toggle-uid" })
wait_for({ text: "Components" })
take_screenshot({ filePath: "mobile-drawer-open.png" })

// Click a component
click({ uid: "button-menu-item-uid" })
wait_for({ text: "Button" })
```

---

### Phase 2: Component-by-Component Validation

Each category gets systematic testing following this pattern:

1. Navigate to component page
2. Take snapshot of rendered state
3. Test interactive elements (clicks, hovers, inputs)
4. Check console for errors
5. Verify responsive behavior
6. Take screenshot for documentation

---

## 📂 Category 1: Action Components (6 components)

### 1. Button (`/components/button`)
**Test Cases**:
- [ ] All color variants render (neutral, primary, secondary, accent, ghost, link)
- [ ] All sizes render (xs, sm, md, lg)
- [ ] All shapes render (circle, square)
- [ ] All styles render (outline, ghost, glass)
- [ ] States work (disabled, loading, active)
- [ ] Click events trigger

```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/button" })
take_snapshot({ verbose: true })

// Test clicking a button
click({ uid: "primary-button-uid" })

// Verify no console errors
list_console_messages({ types: ["error"] })
```

### 2. Dropdown (`/components/dropdown`)
**Test Cases**:
- [ ] Dropdown opens on click
- [ ] Menu items are visible
- [ ] Click closes dropdown
- [ ] Outside click closes dropdown
- [ ] Keyboard navigation works

```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/dropdown" })
take_snapshot()

// Open dropdown
click({ uid: "dropdown-trigger-uid" })
wait_for({ text: "Item 1" })
take_screenshot()

// Select item
click({ uid: "dropdown-item-uid" })
```

### 3. FAB (`/components/fab`)
**Test Cases**:
- [ ] FAB button renders
- [ ] Speed dial opens (flower layout)
- [ ] Speed dial opens (vertical layout)
- [ ] Action buttons are clickable
- [ ] Animations work smoothly

```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/fab" })
take_snapshot()

// Test FAB interaction
click({ uid: "fab-button-uid" })
wait_for({ text: "Speed Dial" })
take_screenshot({ filePath: "fab-open.png" })
```

### 4. Modal (`/components/modal`)
**Test Cases**:
- [ ] Modal opens on button click
- [ ] Modal content displays
- [ ] Close button works
- [ ] Outside click closes modal
- [ ] ESC key closes modal
- [ ] Multiple modals work

```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/modal" })
take_snapshot()

// Open modal
click({ uid: "open-modal-button-uid" })
wait_for({ text: "Modal Title" })
take_screenshot()

// Close with button
click({ uid: "close-modal-button-uid" })

// Open again and close with ESC
click({ uid: "open-modal-button-uid" })
press_key({ key: "Escape" })
```

### 5. Swap (`/components/swap`)
**Test Cases**:
- [ ] Swap renders both states
- [ ] Click toggles swap
- [ ] Rotate effect works
- [ ] Flip effect works
- [ ] Multiple swaps work independently

```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/swap" })
take_snapshot()

// Test swap toggle
click({ uid: "swap-component-uid" })
take_screenshot({ filePath: "swap-toggled.png" })
```

### 6. Theme Controller (`/components/theme_controller`)
**Test Cases**:
- [ ] Theme selector renders
- [ ] Light theme applies
- [ ] Dark theme applies
- [ ] Custom themes work
- [ ] Theme persists across navigation

```javascript
navigate_page({ type: "url", url: "http://127.0.0.1:3000/components/theme_controller" })
take_snapshot()

// Switch to dark theme
click({ uid: "dark-theme-button-uid" })
take_screenshot({ filePath: "dark-theme.png" })

// Verify theme attribute changed
evaluate_script({
  function: "() => document.documentElement.getAttribute('data-theme')"
})
```

---

## 📂 Category 2: Data Display Components (18 components)

### 7. Accordion (`/components/accordion`)
- [ ] Accordion items render
- [ ] Click expands item
- [ ] Click collapses item
- [ ] Multiple items can be open
- [ ] Exclusive mode works

### 8. Avatar (`/components/avatar`)
- [ ] Avatar with image renders
- [ ] Avatar with placeholder renders
- [ ] Avatar sizes work (xs, sm, md, lg)
- [ ] Avatar shapes work (circle, square)
- [ ] Avatar group renders
- [ ] Online indicator works

### 9. Badge (`/components/badge`)
- [ ] All badge colors render
- [ ] All badge sizes render
- [ ] Badge outline variant works
- [ ] Badge in button works
- [ ] Badge with icon works

### 10. Card (`/components/card`)
- [ ] Card body renders
- [ ] Card title renders
- [ ] Card actions render
- [ ] Card with image works
- [ ] Compact card works
- [ ] Card colors work

### 11. Carousel (`/components/carousel`)
- [ ] Carousel items render
- [ ] Navigation buttons work
- [ ] Scroll snap works
- [ ] Indicators work
- [ ] Auto-play works (if implemented)

### 12. Chat (`/components/chat`)
- [ ] Chat bubbles render
- [ ] Chat alignment works (left/right)
- [ ] Chat with avatar works
- [ ] Chat with timestamp works
- [ ] Chat colors work

### 13. Collapse (`/components/collapse`)
- [ ] Collapse content hidden initially
- [ ] Click expands collapse
- [ ] Click collapses content
- [ ] Arrow indicator rotates
- [ ] Multiple collapses work

### 14. Countdown (`/components/countdown`)
- [ ] Countdown displays time
- [ ] Countdown updates (check reactivity)
- [ ] Multiple formats work (days, hours, min, sec)
- [ ] Countdown sizes work

### 15. Diff (`/components/diff`)
- [ ] Diff component renders
- [ ] Before/after comparison works
- [ ] Slider interaction works
- [ ] Responsive behavior works

### 16. Hover 3D (`/components/hover_3d`)
- [ ] Element renders flat
- [ ] Hover triggers 3D tilt
- [ ] Tilt follows mouse movement
- [ ] Reset on mouse leave
- [ ] Multiple elements work independently

### 17. Hover Gallery (`/components/hover_gallery`)
- [ ] Gallery items render horizontally
- [ ] Hover reveals next item
- [ ] Smooth transitions
- [ ] All items accessible
- [ ] Responsive behavior

### 18. Kbd (`/components/kbd`)
- [ ] Keyboard keys render
- [ ] Key sizes work
- [ ] Key combinations render
- [ ] Styled like keyboard buttons

### 19. List (`/components/list`)
- [ ] Ordered list renders
- [ ] Unordered list renders
- [ ] List styles work
- [ ] Nested lists work

### 20. Stats (`/components/stats`)
- [ ] Stat items render
- [ ] Stat titles render
- [ ] Stat values render
- [ ] Stat descriptions render
- [ ] Horizontal layout works
- [ ] Vertical layout works

### 21. Status (`/components/status`)
- [ ] Status indicator renders
- [ ] Status colors work
- [ ] Status with text works
- [ ] Status sizes work

### 22. Table (`/components/table`)
- [ ] Table headers render
- [ ] Table rows render
- [ ] Zebra striping works
- [ ] Hover effect works
- [ ] Compact table works
- [ ] Pinned headers work (if implemented)

### 23. Text Rotate (`/components/text_rotate`)
- [ ] Text items render
- [ ] Rotation animation plays
- [ ] Configurable speed works
- [ ] All items display in cycle

### 24. Timeline (`/components/timeline`)
- [ ] Timeline items render
- [ ] Timeline connectors render
- [ ] Horizontal layout works
- [ ] Vertical layout works
- [ ] Icons in timeline work

---

## 📂 Category 3: Navigation Components (6 components)

### 25. Breadcrumbs (`/components/breadcrumbs`)
- [ ] Breadcrumb items render
- [ ] Separators display
- [ ] Links are clickable
- [ ] Active item highlighted

### 26. Menu (`/components/menu`)
- [ ] Menu items render
- [ ] Submenu expands
- [ ] Selected item highlighted
- [ ] Menu hover effects work
- [ ] Compact menu works

### 27. Navbar (`/components/navbar`)
- [ ] Navbar start section renders
- [ ] Navbar center section renders
- [ ] Navbar end section renders
- [ ] Navbar colors work
- [ ] Navbar responsive behavior works

### 28. Pagination (`/components/pagination`)
- [ ] Page buttons render
- [ ] Active page highlighted
- [ ] Previous/next buttons work
- [ ] Page number clicks work
- [ ] Disabled states work

### 29. Steps (`/components/steps`)
- [ ] Step items render
- [ ] Current step highlighted
- [ ] Completed steps marked
- [ ] Horizontal layout works
- [ ] Vertical layout works

### 30. Tab (`/components/tab`)
- [ ] Tab buttons render
- [ ] Active tab highlighted
- [ ] Tab click changes content
- [ ] Tab variants work (bordered, lifted)
- [ ] Tab sizes work

---

## 📂 Category 4: Feedback Components (7 components)

### 31. Alert (`/components/alert`)
- [ ] Alert types render (info, success, warning, error)
- [ ] Alert with icon works
- [ ] Alert with close button works
- [ ] Alert dismissal works

### 32. Loading (`/components/loading`)
- [ ] Spinner renders
- [ ] Loading sizes work
- [ ] Loading colors work
- [ ] Multiple loading types work (spinner, dots, ring, ball, bars)

### 33. Progress (`/components/progress`)
- [ ] Progress bar renders
- [ ] Progress value updates
- [ ] Progress colors work
- [ ] Indeterminate progress works

### 34. Radial Progress (`/components/radial_progress`)
- [ ] Radial progress renders
- [ ] Progress percentage displays
- [ ] Progress value updates
- [ ] Radial progress sizes work
- [ ] Radial progress colors work

### 35. Skeleton (`/components/skeleton`)
- [ ] Skeleton elements render
- [ ] Skeleton animation works
- [ ] Different skeleton shapes work
- [ ] Skeleton in content works

### 36. Toast (`/components/toast`)
- [ ] Toast notification appears
- [ ] Toast auto-dismisses
- [ ] Toast positions work (top, bottom, left, right)
- [ ] Multiple toasts stack
- [ ] Toast with actions works

### 37. Tooltip (`/components/tooltip`)
- [ ] Tooltip appears on hover
- [ ] Tooltip positions work (top, bottom, left, right)
- [ ] Tooltip colors work
- [ ] Tooltip with custom content works

---

## 📂 Category 5: Data Input Components (14 components)

### 38. Calendar (`/components/calendar`)
- [ ] Calendar wrapper styles apply
- [ ] Integration examples render
- [ ] Date selection works (if demo includes)

### 39. Checkbox (`/components/checkbox`)
- [ ] Checkbox renders
- [ ] Checkbox toggles
- [ ] Checkbox sizes work
- [ ] Checkbox colors work
- [ ] Disabled state works
- [ ] Indeterminate state works

### 40. Fieldset (`/components/fieldset`)
- [ ] Fieldset renders
- [ ] Legend displays
- [ ] Form fields inside fieldset work
- [ ] Disabled fieldset works

### 41. File Input (`/components/file_input`)
- [ ] File input renders
- [ ] File input sizes work
- [ ] File input colors work
- [ ] File selection works
- [ ] Multiple file selection works

### 42. Filter (`/components/filter`)
- [ ] Filter buttons render
- [ ] Filter selection works
- [ ] Active filter highlighted
- [ ] Multiple filters work

### 43. Input (`/components/input`)
- [ ] Input renders
- [ ] Input types work (text, email, password, etc.)
- [ ] Input sizes work
- [ ] Input colors work
- [ ] Input with icon works
- [ ] Disabled state works

### 44. Label (`/components/label`)
- [ ] Label renders
- [ ] Label text displays
- [ ] Label alt text works
- [ ] Label with required indicator works

### 45. Radio (`/components/radio`)
- [ ] Radio buttons render
- [ ] Radio selection works
- [ ] Only one radio selected in group
- [ ] Radio sizes work
- [ ] Radio colors work
- [ ] Disabled state works

### 46. Range (`/components/range`)
- [ ] Range slider renders
- [ ] Slider value changes
- [ ] Range min/max work
- [ ] Range step works
- [ ] Range sizes work
- [ ] Range colors work

### 47. Rating (`/components/rating`)
- [ ] Rating stars render
- [ ] Rating click works
- [ ] Rating hover effects work
- [ ] Rating sizes work
- [ ] Half-star ratings work (if implemented)

### 48. Select (`/components/select`)
- [ ] Select renders
- [ ] Select opens dropdown
- [ ] Option selection works
- [ ] Select sizes work
- [ ] Select colors work
- [ ] Disabled state works

### 49. Textarea (`/components/textarea`)
- [ ] Textarea renders
- [ ] Textarea accepts input
- [ ] Textarea sizes work
- [ ] Textarea colors work
- [ ] Disabled state works
- [ ] Resize behavior works

### 50. Toggle (`/components/toggle`)
- [ ] Toggle renders
- [ ] Toggle switches on/off
- [ ] Toggle sizes work
- [ ] Toggle colors work
- [ ] Disabled state works

### 51. Validator (`/components/validator`)
- [ ] Validation message renders
- [ ] Error state displays
- [ ] Success state displays
- [ ] Validation rules work
- [ ] Real-time validation works

---

## 📂 Category 6: Layout Components (10 components)

### 52. Divider (`/components/divider`)
- [ ] Horizontal divider renders
- [ ] Vertical divider renders
- [ ] Divider with text works
- [ ] Divider colors work

### 53. Dock (`/components/dock`)
- [ ] Dock renders
- [ ] Dock items display
- [ ] Dock magnification effect works
- [ ] Dock icons are clickable

### 54. Drawer (`/components/drawer`)
- [ ] Drawer renders
- [ ] Drawer toggle works
- [ ] Drawer content displays
- [ ] Drawer side renders
- [ ] Overlay click closes drawer
- [ ] Drawer positions work (left, right)

### 55. Footer (`/components/footer`)
- [ ] Footer renders
- [ ] Footer sections display
- [ ] Footer links work
- [ ] Footer responsive layout works

### 56. Hero (`/components/hero`)
- [ ] Hero section renders
- [ ] Hero content displays
- [ ] Hero with image works
- [ ] Hero responsive behavior works

### 57. Indicator (`/components/indicator`)
- [ ] Indicator renders
- [ ] Indicator positions work (top-right, bottom-left, etc.)
- [ ] Indicator with count works
- [ ] Indicator colors work

### 58. Join (`/components/join`)
- [ ] Joined items render
- [ ] Horizontal join works
- [ ] Vertical join works
- [ ] Joined buttons work
- [ ] Joined inputs work

### 59. Link (`/components/link`)
- [ ] Link renders
- [ ] Link is clickable
- [ ] Link colors work
- [ ] Link hover effects work
- [ ] External link works

### 60. Mask (`/components/mask`)
- [ ] Mask shapes render
- [ ] Different mask types work (squircle, heart, star, hexagon, etc.)
- [ ] Mask sizes work
- [ ] Mask with image works

### 61. Stack (`/components/stack`)
- [ ] Stacked items render
- [ ] Stack z-index layering works
- [ ] Stack with images works
- [ ] Stack with cards works

---

## 📂 Category 7: Mockup Components (4 components)

### 62. Mockup Browser (`/components/mockup_browser`)
- [ ] Browser mockup renders
- [ ] Address bar displays
- [ ] Browser toolbar renders
- [ ] Content area displays

### 63. Mockup Code (`/components/mockup_code`)
- [ ] Code mockup renders
- [ ] Code syntax display works
- [ ] Line numbers display (if implemented)
- [ ] Code highlighting works

### 64. Mockup Phone (`/components/mockup_phone`)
- [ ] Phone mockup renders
- [ ] Phone screen displays content
- [ ] Phone frame renders correctly
- [ ] Notch displays (if styled)

### 65. Mockup Window (`/components/mockup_window`)
- [ ] Window mockup renders
- [ ] Window controls display
- [ ] Title bar renders
- [ ] Content area displays

---

## 🤖 Automated Test Script Template

### Full Automation Script (Chrome DevTools MCP)

```javascript
// ========================================
// AUTOMATED DEMO APP TESTING SCRIPT
// ========================================

// Configuration
const BASE_URL = "http://127.0.0.1:3000";
const SCREENSHOT_DIR = "./test-screenshots";
const DESKTOP_SIZE = { width: 1920, height: 1080 };
const MOBILE_SIZE = { width: 375, height: 667 };

// Component routes (all 62)
const COMPONENT_ROUTES = [
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

// ========================================
// Test Functions
// ========================================

async function setupTest() {
  console.log("🚀 Setting up test environment...");

  // List pages
  const pages = await list_pages();
  console.log(`Found ${pages.length} page(s)`);

  // Select first page
  await select_page(0);

  // Resize to desktop
  await resize_page(DESKTOP_SIZE);

  console.log("✅ Setup complete");
}

async function testLandingPage() {
  console.log("\n📍 Testing Landing Page...");

  await navigate_page({ type: "url", url: BASE_URL });
  await wait_for({ text: "leptos-daisyui-rs", timeout: 5000 });

  const snapshot = await take_snapshot();
  console.log("Snapshot taken");

  await take_screenshot({
    filePath: `${SCREENSHOT_DIR}/00-landing.png`
  });

  // Check for errors
  const errors = await list_console_messages({ types: ["error"] });
  if (errors.length > 0) {
    console.warn(`⚠️ Found ${errors.length} console errors`);
  }

  console.log("✅ Landing page test complete");
}

async function testComponent(componentName, index) {
  console.log(`\n📍 Testing Component ${index + 1}/62: ${componentName}...`);

  const url = `${BASE_URL}/components/${componentName}`;

  try {
    // Navigate to component
    await navigate_page({ type: "url", url, timeout: 10000 });

    // Wait a bit for render
    await new Promise(resolve => setTimeout(resolve, 1000));

    // Take snapshot
    const snapshot = await take_snapshot({ verbose: false });

    // Take screenshot
    const screenshotNum = String(index + 1).padStart(2, '0');
    await take_screenshot({
      filePath: `${SCREENSHOT_DIR}/${screenshotNum}-${componentName}.png`,
      fullPage: true
    });

    // Check for console errors
    const errors = await list_console_messages({ types: ["error"] });
    if (errors.length > 0) {
      console.warn(`⚠️ ${componentName}: ${errors.length} console error(s)`);
      // Get error details
      for (const error of errors.slice(0, 3)) {
        const msg = await get_console_message({ msgid: error.msgid });
        console.log(`   Error: ${msg.text}`);
      }
    } else {
      console.log(`✅ ${componentName}: No errors`);
    }

    return { component: componentName, success: true, errors: errors.length };

  } catch (error) {
    console.error(`❌ ${componentName}: Test failed - ${error.message}`);
    return { component: componentName, success: false, error: error.message };
  }
}

async function testMobileResponsiveness() {
  console.log("\n📱 Testing Mobile Responsiveness...");

  // Switch to mobile size
  await resize_page(MOBILE_SIZE);

  // Navigate to a component page
  await navigate_page({
    type: "url",
    url: `${BASE_URL}/components/button`
  });

  // Take snapshot
  const snapshot = await take_snapshot();

  // Look for drawer toggle (hamburger menu)
  // This would require finding the element UID from snapshot

  await take_screenshot({
    filePath: `${SCREENSHOT_DIR}/mobile-view.png`
  });

  console.log("✅ Mobile test complete");
}

async function testThemeSwitching() {
  console.log("\n🎨 Testing Theme Switching...");

  await resize_page(DESKTOP_SIZE);

  // Navigate to theme controller page
  await navigate_page({
    type: "url",
    url: `${BASE_URL}/components/theme_controller`
  });

  await wait_for({ text: "Theme Controller", timeout: 5000 });

  // Take screenshot in light mode
  await take_screenshot({
    filePath: `${SCREENSHOT_DIR}/theme-light.png`
  });

  // Check current theme
  const currentTheme = await evaluate_script({
    function: "() => document.documentElement.getAttribute('data-theme')"
  });
  console.log(`Current theme: ${currentTheme}`);

  // Note: Actual theme switching would require knowing button UIDs from snapshot

  console.log("✅ Theme test complete");
}

async function generateReport(results) {
  console.log("\n" + "=".repeat(50));
  console.log("📊 TEST REPORT");
  console.log("=".repeat(50));

  const successful = results.filter(r => r.success).length;
  const failed = results.filter(r => !r.success).length;
  const totalErrors = results.reduce((sum, r) => sum + (r.errors || 0), 0);

  console.log(`\n✅ Successful: ${successful}/62`);
  console.log(`❌ Failed: ${failed}/62`);
  console.log(`⚠️ Total console errors: ${totalErrors}`);

  if (failed > 0) {
    console.log("\n❌ Failed components:");
    results
      .filter(r => !r.success)
      .forEach(r => console.log(`   - ${r.component}: ${r.error}`));
  }

  if (totalErrors > 0) {
    console.log("\n⚠️ Components with errors:");
    results
      .filter(r => r.errors > 0)
      .forEach(r => console.log(`   - ${r.component}: ${r.errors} error(s)`));
  }

  console.log("\n" + "=".repeat(50));
}

// ========================================
// Main Test Runner
// ========================================

async function runAllTests() {
  console.log("🎯 Starting Comprehensive Demo App Testing");
  console.log(`📅 ${new Date().toISOString()}\n`);

  try {
    // Setup
    await setupTest();

    // Test landing page
    await testLandingPage();

    // Test all 62 components
    const results = [];
    for (let i = 0; i < COMPONENT_ROUTES.length; i++) {
      const result = await testComponent(COMPONENT_ROUTES[i], i);
      results.push(result);

      // Small delay between tests
      await new Promise(resolve => setTimeout(resolve, 500));
    }

    // Test mobile responsiveness
    await testMobileResponsiveness();

    // Test theme switching
    await testThemeSwitching();

    // Generate report
    await generateReport(results);

    console.log("\n🎉 Testing complete!");

  } catch (error) {
    console.error("💥 Test runner failed:", error);
  }
}

// Run the tests
runAllTests();
```

---

## 📝 Manual Testing Checklist

For components with complex interactions, manual testing supplements automation:

### Interactive Components Priority List

1. **Modal** - Test ESC key, outside click, nested modals
2. **Dropdown** - Test keyboard navigation (Tab, Arrow keys, Enter)
3. **Drawer** - Test swipe gestures on mobile (if supported)
4. **Carousel** - Test touch swipe on mobile
5. **Range** - Test keyboard arrow keys for slider
6. **Rating** - Test keyboard navigation
7. **Input** - Test autocomplete, validation
8. **Textarea** - Test resize handles
9. **Select** - Test keyboard search in options
10. **FAB** - Test animation timing and smoothness

---

## 🔍 Performance Testing

### Metrics to Collect

```javascript
// Performance measurement
await evaluate_script({
  function: `() => {
    const perfData = performance.getEntriesByType("navigation")[0];
    return {
      domComplete: perfData.domComplete,
      loadEventEnd: perfData.loadEventEnd,
      domInteractive: perfData.domInteractive
    };
  }`
});

// Check for layout shifts
await evaluate_script({
  function: `() => {
    let cls = 0;
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) {
        if (!entry.hadRecentInput) {
          cls += entry.value;
        }
      }
    }).observe({ entryTypes: ['layout-shift'] });
    return cls;
  }`
});
```

---

## 📸 Visual Regression Testing

Save screenshots from each test run and compare against baseline:

```bash
# Create baseline
mkdir test-screenshots/baseline
# Run tests, save to baseline/

# Future runs
mkdir test-screenshots/run-$(date +%Y%m%d)
# Compare with baseline using image diff tools
```

---

## ✅ Success Criteria

### Component Pass Criteria
- [ ] Component renders without console errors
- [ ] All visual variants display correctly
- [ ] Interactive elements respond to user actions
- [ ] No layout shifts or flickers
- [ ] Responsive behavior works on mobile
- [ ] Theme switching applies correctly

### Overall Pass Criteria
- [ ] 62/62 components load successfully
- [ ] Zero critical console errors
- [ ] All navigation routes work
- [ ] Mobile drawer functions correctly
- [ ] Theme switching works globally
- [ ] No accessibility violations (check with DevTools)

---

## 🚀 Execution Commands

### Using Chrome DevTools MCP (from Claude Code)

```javascript
// Option 1: Interactive testing
// Run each test phase manually through Chrome DevTools MCP tools

// Option 2: Automated script
// Copy the automation script above and execute through evaluate_script

// Option 3: Hybrid approach
// Use Task tool to spawn automation agent
```

### Using Claude Chrome Extension

1. Open Chrome with extension
2. Navigate to http://127.0.0.1:3000
3. Open Claude side panel
4. Paste testing commands
5. Ask Claude to verify each component

---

## 📊 Deliverables

1. **Test Screenshots** (62+ images)
   - One per component
   - Mobile/desktop views
   - Light/dark themes

2. **Console Error Log**
   - List of all errors found
   - Severity classification
   - Recommended fixes

3. **Component Status Report**
   - Pass/fail status for each component
   - Interactive feature coverage
   - Known issues

4. **Performance Metrics**
   - Load times per component
   - Layout shift measurements
   - Resource usage

---

## 🔧 Troubleshooting

### Common Issues

**Issue**: Component not loading
**Solution**: Check network tab, verify route in main.rs

**Issue**: Snapshot missing elements
**Solution**: Increase wait timeout, check for lazy loading

**Issue**: Click not working
**Solution**: Verify UID from latest snapshot, check for overlays

**Issue**: Console errors on all pages
**Solution**: Check for global JavaScript errors, verify Trunk build

---

## 📅 Estimated Time

- **Setup**: 10 minutes
- **Automated tests**: 30-45 minutes (all 62 components)
- **Manual interaction tests**: 30 minutes
- **Performance analysis**: 15 minutes
- **Report generation**: 15 minutes

**Total**: ~2 hours for complete validation

---

## 🎯 Next Steps

After testing completion:

1. Create GitHub issue for any failures
2. Document breaking changes
3. Update component documentation
4. Create regression test suite
5. Set up CI/CD integration

---

**Version**: 1.0
**Last Updated**: 2026-01-29
**Test Environment**: Trunk dev server on localhost:3000
