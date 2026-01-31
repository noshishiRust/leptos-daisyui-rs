# Chrome DevTools MCP Testing Guide

This guide explains how to use the Chrome DevTools MCP server to test leptos-daisyui-rs components visually and functionally.

## Prerequisites

1. **Demo Server Running**
```bash
cd demo
trunk serve
# Server runs at http://localhost:3000
```

2. **Chrome DevTools MCP Available**
   - Configure MCP server in Claude Code settings
   - Available tools: navigate_page, click, fill, take_screenshot, evaluate_script, etc.

3. **Claude for Chrome Extension** (Optional)
   - Install from Chrome Web Store
   - Provides additional automation capabilities

---

## Testing Workflow

### 1. Start Testing Session

```bash
# In Claude Code, use ToolSearch to find chrome-devtools tools
ToolSearch: "chrome"

# Then use chrome-devtools MCP tools directly
```

### 2. Navigate to Component

```
Use navigate_page tool:
- URL: http://localhost:3000/components/{component-name}
- Wait for load
```

### 3. Interact and Verify

```
Use available tools:
- click: Click elements
- fill: Fill form inputs
- evaluate_script: Run JavaScript
- take_screenshot: Capture visuals
- get_snapshot: Get DOM snapshot
```

---

## Component Testing Scripts

### BaseThemeSelector

```javascript
// Navigate to component
await navigate_page({ url: 'http://localhost:3000/components/base_theme_selector' });

// Wait for grid to load
await evaluate_script({
  script: `
    new Promise(resolve => {
      const checkGrid = () => {
        const grid = document.querySelector('.grid.grid-cols-2');
        if (grid && grid.children.length > 0) resolve(true);
        else setTimeout(checkGrid, 100);
      };
      checkGrid();
    })
  `
});

// Test theme selection
await click({ selector: '[data-theme="cyberpunk"]' });

// Verify theme applied
const result = await evaluate_script({
  script: `
    document.documentElement.getAttribute('data-theme') === 'cyberpunk'
  `
});

// Take screenshot
await take_screenshot({ path: 'screenshots/base_theme_selector.png' });
```

### ColorCustomizer

```javascript
// Navigate
await navigate_page({ url: 'http://localhost:3000/components/component_customizer' });

// Open color customizer accordion
await click({ selector: 'button:has-text("Brand Colors")' });

// Test primary color change
await fill({ selector: 'input[data-color="primary"]', value: '#3b82f6' });

// Verify CSS variable updated
const cssVarUpdated = await evaluate_script({
  script: `
    const primaryColor = getComputedStyle(document.documentElement)
      .getPropertyValue('--p');
    primaryColor.includes('oklch')
  `
});

// Verify preview button updated
const buttonColor = await evaluate_script({
  script: `
    const btn = document.querySelector('.preview .btn-primary');
    const bg = getComputedStyle(btn).backgroundColor;
    bg.length > 0
  `
});

// Take screenshot of preview
await take_screenshot({ selector: '.preview', path: 'screenshots/color_preview.png' });
```

### TypographyCustomizer

```javascript
// Navigate
await navigate_page({ url: 'http://localhost:3000/components/typography_customizer' });

// Change base font size
await evaluate_script({
  script: `
    const slider = document.querySelector('[data-slider="base-size"]');
    slider.value = 18;
    slider.dispatchEvent(new Event('input', { bubbles: true }));
  `
});

// Verify font size applied
const fontSizeApplied = await evaluate_script({
  script: `
    const fontSize = getComputedStyle(document.body).fontSize;
    parseFloat(fontSize) === 18
  `
});

// Test type scale
await evaluate_script({
  script: `
    const scaleSlider = document.querySelector('[data-slider="type-scale"]');
    scaleSlider.value = 1.250;
    scaleSlider.dispatchEvent(new Event('input', { bubbles: true }));
  `
});

// Take screenshot
await take_screenshot({ path: 'screenshots/typography_customizer.png' });
```

### Icon Component

```javascript
// Navigate
await navigate_page({ url: 'http://localhost:3000/components/icon' });

// Verify icon renders as SVG
const isSvg = await evaluate_script({
  script: `
    const icon = document.querySelector('[data-component="icon"]');
    icon && icon.tagName === 'svg'
  `
});

// Test size variants
const sizes = ['xs', 'sm', 'md', 'lg', 'xl'];
for (const size of sizes) {
  await click({ selector: `[data-icon-size="${size}"]` });

  const sizeCorrect = await evaluate_script({
    script: `
      const icon = document.querySelector('[data-component="icon"]');
      const width = icon.getAttribute('width');
      width !== null
    `
  });
}

// Test rotation
await click({ selector: '[data-icon-rotate="90"]' });
const rotated = await evaluate_script({
  script: `
    const icon = document.querySelector('[data-component="icon"]');
    const transform = getComputedStyle(icon).transform;
    transform.includes('matrix')
  `
});
```

### Tag Component

```javascript
// Navigate
await navigate_page({ url: 'http://localhost:3000/components/tag' });

// Verify tag renders
const tagExists = await evaluate_script({
  script: `document.querySelector('.badge') !== null`
});

// Test removable tag
await click({ selector: '.badge .btn-close' });

const tagRemoved = await evaluate_script({
  script: `
    new Promise(resolve => {
      setTimeout(() => {
        resolve(document.querySelector('.badge') === null);
      }, 500);
    })
  `
});

// Take screenshots of variants
await take_screenshot({ selector: '.tag-variants', path: 'screenshots/tag_variants.png' });
```

### Persona Component

```javascript
// Navigate
await navigate_page({ url: 'http://localhost:3000/components/persona' });

// Verify structure
const structure = await evaluate_script({
  script: `
    const persona = document.querySelector('[data-component="persona"]');
    const avatar = persona.querySelector('.avatar');
    const text = persona.querySelector('.persona-text');
    const status = persona.querySelector('.badge');

    !!(avatar && text && status)
  `
});

// Test size variants
const sizes = ['sm', 'md', 'lg'];
for (const size of sizes) {
  await click({ selector: `[data-persona-size="${size}"]` });

  const screenshot = await take_screenshot({
    selector: '[data-component="persona"]',
    path: `screenshots/persona_${size}.png`
  });
}
```

### ThemeExportImport

```javascript
// Navigate
await navigate_page({ url: 'http://localhost:3000/components/theme_export_import' });

// Test export
await click({ selector: '[data-action="export"]' });

// Verify download initiated (check for success message)
const exportSuccess = await evaluate_script({
  script: `
    new Promise(resolve => {
      setTimeout(() => {
        const successBadge = document.querySelector('.badge-success');
        resolve(successBadge !== null);
      }, 1000);
    })
  `
});

// Test clipboard export
await click({ selector: '[data-action="copy"]' });

const copiedToClipboard = await evaluate_script({
  script: `
    navigator.clipboard.readText().then(text => {
      return text.length > 0 && text.startsWith('{');
    })
  `
});
```

### LoadingBar

```javascript
// Navigate
await navigate_page({ url: 'http://localhost:3000/components/loading_bar' });

// Test progress update
await evaluate_script({
  script: `
    const progressBar = document.querySelector('[data-component="loading-bar"]');
    const setValue = (value) => {
      const event = new CustomEvent('progress-change', { detail: { value } });
      progressBar.dispatchEvent(event);
    };
    setValue(75);
  `
});

// Verify progress
const progressCorrect = await evaluate_script({
  script: `
    const fill = document.querySelector('.loading-bar-fill');
    const width = getComputedStyle(fill).width;
    const parentWidth = getComputedStyle(fill.parentElement).width;
    const percentage = (parseFloat(width) / parseFloat(parentWidth)) * 100;
    Math.abs(percentage - 75) < 2
  `
});

// Test indeterminate mode
await click({ selector: '[data-loading-mode="indeterminate"]' });

const isAnimating = await evaluate_script({
  script: `
    const fill = document.querySelector('.loading-bar-fill');
    const animation = getComputedStyle(fill).animation;
    animation !== 'none'
  `
});
```

### MessageBar

```javascript
// Navigate
await navigate_page({ url: 'http://localhost:3000/components/message_bar' });

// Test message types
const types = ['info', 'success', 'warning', 'error'];
for (const type of types) {
  await click({ selector: `[data-message-type="${type}"]` });

  const hasCorrectClass = await evaluate_script({
    script: `
      const alert = document.querySelector('.alert');
      alert.classList.contains('alert-${type}')
    `
  });

  await take_screenshot({ path: `screenshots/message_bar_${type}.png` });
}

// Test close button
await click({ selector: '.alert .btn-close' });

const closed = await evaluate_script({
  script: `
    new Promise(resolve => {
      setTimeout(() => {
        resolve(document.querySelector('.alert') === null);
      }, 500);
    })
  `
});

// Test auto-dismiss
await click({ selector: '[data-message-dismiss="3000"]' });

const autoDismissed = await evaluate_script({
  script: `
    new Promise(resolve => {
      setTimeout(() => {
        resolve(document.querySelector('.alert') === null);
      }, 3500);
    })
  `
});
```

---

## Visual Regression Testing

### Taking Baseline Screenshots

```javascript
// For each component, take baseline screenshot
const components = [
  'base_theme_selector',
  'color_customizer',
  'typography_customizer',
  'icon',
  'tag',
  'persona',
  'loading_bar',
  'message_bar'
];

for (const component of components) {
  await navigate_page({ url: `http://localhost:3000/components/${component}` });
  await take_screenshot({ path: `baselines/${component}.png` });
}
```

### Comparing Against Baselines

```javascript
// After changes, compare new screenshots
const currentScreenshot = await take_screenshot();

// Use external tool or API to compare
const diff = await compareImages(
  currentScreenshot,
  'baselines/component.png'
);

// Diff should be < 1% for visual regression
assert(diff < 0.01, `Visual diff ${diff} exceeds threshold`);
```

---

## Responsive Testing

### Test at Different Viewport Sizes

```javascript
const viewports = [
  { width: 375, height: 667, name: 'mobile' },
  { width: 768, height: 1024, name: 'tablet' },
  { width: 1920, height: 1080, name: 'desktop' }
];

for (const viewport of viewports) {
  await resize_page(viewport);

  await take_screenshot({
    path: `screenshots/${component}_${viewport.name}.png`
  });

  // Verify responsive behavior
  const isResponsive = await evaluate_script({
    script: `
      const grid = document.querySelector('.grid');
      const columns = getComputedStyle(grid).gridTemplateColumns;
      columns.split(' ').length
    `
  });
}
```

---

## Accessibility Testing

### Keyboard Navigation

```javascript
// Test tab navigation
await press_key({ key: 'Tab' });
const firstFocused = await evaluate_script({
  script: `document.activeElement.tagName`
});

// Test Enter activation
await press_key({ key: 'Enter' });

// Verify action occurred
```

### Screen Reader Support

```javascript
// Check ARIA attributes
const ariaLabels = await evaluate_script({
  script: `
    const elements = document.querySelectorAll('[role], [aria-label], [aria-labelledby]');
    Array.from(elements).map(el => ({
      role: el.getAttribute('role'),
      label: el.getAttribute('aria-label'),
      tag: el.tagName
    }))
  `
});

// Verify all interactive elements have labels
```

### Color Contrast

```javascript
// Check contrast ratios
const contrastIssues = await evaluate_script({
  script: `
    const getContrast = (fg, bg) => {
      // Simplified contrast calculation
      // Use actual WCAG algorithm in production
      return calculateContrastRatio(fg, bg);
    };

    const buttons = document.querySelectorAll('.btn');
    const issues = [];

    buttons.forEach(btn => {
      const fg = getComputedStyle(btn).color;
      const bg = getComputedStyle(btn).backgroundColor;
      const contrast = getContrast(fg, bg);

      if (contrast < 4.5) {
        issues.push({ element: btn.className, contrast });
      }
    });

    issues
  `
});
```

---

## Performance Testing

### Measure Render Time

```javascript
const renderTime = await evaluate_script({
  script: `
    performance.mark('component-start');

    // Trigger component render
    const component = document.querySelector('[data-component]');

    performance.mark('component-end');
    performance.measure('component-render', 'component-start', 'component-end');

    const measure = performance.getEntriesByName('component-render')[0];
    measure.duration
  `
});

// Should be < 100ms for optimal UX
assert(renderTime < 100, `Render time ${renderTime}ms exceeds threshold`);
```

### Measure Paint Time

```javascript
const paintMetrics = await evaluate_script({
  script: `
    const paint = performance.getEntriesByType('paint');
    {
      firstPaint: paint.find(p => p.name === 'first-paint')?.startTime,
      firstContentfulPaint: paint.find(p => p.name === 'first-contentful-paint')?.startTime
    }
  `
});
```

---

## Test Automation Examples

### Complete Test Suite for a Component

```javascript
async function testBaseThemeSelector() {
  console.log('Starting BaseThemeSelector tests...');

  // 1. Navigate
  await navigate_page({ url: 'http://localhost:3000/components/base_theme_selector' });

  // 2. Wait for load
  await waitForElement('.grid.grid-cols-2');

  // 3. Take baseline screenshot
  await take_screenshot({ path: 'baselines/base_theme_selector.png' });

  // 4. Test theme selection
  const themes = ['light', 'dark', 'cyberpunk', 'cupcake'];
  for (const theme of themes) {
    await click({ selector: `[data-theme="${theme}"]` });

    const applied = await evaluate_script({
      script: `document.documentElement.getAttribute('data-theme') === '${theme}'`
    });

    assert(applied, `Theme ${theme} should be applied`);

    await take_screenshot({ path: `screenshots/theme_${theme}.png` });
  }

  // 5. Test responsive grid
  const viewports = [
    { width: 375, name: 'mobile', expectedCols: 2 },
    { width: 768, name: 'tablet', expectedCols: 3 },
    { width: 1920, name: 'desktop', expectedCols: 4 }
  ];

  for (const vp of viewports) {
    await resize_page({ width: vp.width, height: 800 });

    const cols = await evaluate_script({
      script: `
        const grid = document.querySelector('.grid');
        getComputedStyle(grid).gridTemplateColumns.split(' ').length
      `
    });

    assert(cols === vp.expectedCols, `Should have ${vp.expectedCols} columns at ${vp.name}`);
  }

  // 6. Test accessibility
  const a11y = await evaluate_script({
    script: `
      const cards = document.querySelectorAll('.card');
      Array.from(cards).every(card => {
        const theme = card.getAttribute('data-theme');
        return theme !== null;
      })
    `
  });

  assert(a11y, 'All cards should have data-theme attribute');

  console.log('✅ All BaseThemeSelector tests passed!');
}
```

---

## Claude for Chrome Extension Usage

### Interactive Testing

When using Claude for Chrome extension:

1. **Open demo page in Chrome**
2. **Activate Claude extension** (click icon or keyboard shortcut)
3. **Ask Claude to test components:**

```
Example prompts:

"Test the BaseThemeSelector by clicking on the cyberpunk theme and verify the data-theme attribute updates"

"Fill in the primary color in ColorCustomizer with #3b82f6 and verify the CSS variable updates"

"Take screenshots of all Tag component variants"

"Test keyboard navigation through the TypographyCustomizer form"

"Verify all buttons have sufficient color contrast"

"Measure the render time of the Icon component"
```

### Automated Test Execution

```
"Run the complete test suite for BaseThemeSelector and report results"

"Compare current screenshots with baselines and report any visual regressions"

"Test all theming components and generate a test report"
```

---

## Test Report Template

```markdown
# Component Test Report: [Component Name]

**Date:** [Date]
**Tested By:** Claude Code / Human
**Browser:** Chrome [Version]
**Viewport:** [Dimensions]

## Test Results

### Rendering
- [ ] Component renders without errors
- [ ] All variants display correctly
- [ ] Responsive behavior works

### Functionality
- [ ] All interactions work as expected
- [ ] State updates correctly
- [ ] Event handlers fire

### Visual
- [ ] Matches design specifications
- [ ] No visual regressions
- [ ] Screenshots attached

### Accessibility
- [ ] Keyboard navigation works
- [ ] ARIA attributes present
- [ ] Color contrast meets WCAG AA

### Performance
- Render time: [X]ms
- Paint time: [X]ms
- Bundle size: [X]kb

## Issues Found
- Issue 1: Description
- Issue 2: Description

## Screenshots
![Baseline](baselines/component.png)
![Current](screenshots/component.png)

## Notes
Additional observations...
```

---

## Best Practices

1. **Always start demo server before testing**
2. **Wait for components to load completely**
3. **Test in clean browser state (no extensions except test tools)**
4. **Use data attributes for test selectors** (`data-testid`, `data-component`)
5. **Take screenshots at consistent viewport sizes**
6. **Test both mouse and keyboard interactions**
7. **Verify accessibility in every test**
8. **Measure performance for complex components**
9. **Document any flaky tests**
10. **Keep baseline screenshots up to date**

---

## Troubleshooting

### Component Not Found
- Verify demo server is running
- Check component route exists
- Wait longer for component to load

### Screenshot Differs from Baseline
- Check if intentional design change
- Verify same viewport size
- Check for animation timing issues

### Test Fails Intermittently
- Add explicit wait times
- Check for race conditions
- Verify network requests complete

---

## Next Steps

1. Run test suite for all components
2. Generate baseline screenshots
3. Set up CI/CD integration
4. Create automated regression tests
5. Add performance budgets
6. Implement continuous visual regression
