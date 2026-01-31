# Visual Test Execution Guide

This guide provides step-by-step instructions for running visual tests on all new components using Chrome DevTools MCP.

## Prerequisites

1. Start the demo server:
```bash
cd demo
trunk serve
# Wait for "Finished" message, server at http://localhost:3000
```

2. Ensure Chrome DevTools MCP is available in Claude Code

## Running Tests with Claude Code

Use the following prompts to test components:

### BaseThemeSelector

```
Using chrome-devtools MCP:

1. Navigate to http://localhost:3000/components/base_theme_selector
2. Wait for the grid to load completely
3. Click on the "cyberpunk" theme card
4. Verify document.documentElement.getAttribute('data-theme') === 'cyberpunk'
5. Take a screenshot and save to screenshots/base_theme_selector_cyberpunk.png
6. Test 3 more themes: dark, cupcake, forest
7. For each theme, verify the selected card has border-primary and ring-2 classes
8. Test responsive grid: resize to 375px width (should show 2 columns), 768px (3 columns), 1920px (4 columns)
9. Report any visual or functional issues
```

### ColorCustomizer

```
Using chrome-devtools MCP:

1. Navigate to http://localhost:3000/components/theming  (or wherever ColorCustomizer is shown)
2. Expand the "Brand Colors" accordion
3. Fill the primary color input with #3b82f6
4. Verify getComputedStyle(document.documentElement).getPropertyValue('--p') contains 'oklch'
5. Check that preview buttons reflect the new primary color
6. Test the reset button - verify primary color returns to default
7. Test all 4 color categories: Brand, Base, State, and verify each updates CSS variables
8. Take screenshots of each color category before and after changes
9. Report the test results
```

### TypographyCustomizer

```
Using chrome-devtools MCP:

1. Navigate to the Typography Customizer demo
2. Change the base font size slider to 18px
3. Verify getComputedStyle(document.body).fontSize equals '18px'
4. Set type scale to 1.333 (Perfect Fourth)
5. Verify heading sizes scale correctly with the new ratio
6. Select "Inter, sans-serif" for heading font family
7. Verify getComputedStyle(document.querySelector('h1')).fontFamily includes 'Inter'
8. Test the preview section updates reactively
9. Take screenshots showing before/after typography changes
10. Test reset button functionality
```

### Icon Component

```
Using chrome-devtools MCP:

1. Navigate to http://localhost:3000/components/icon
2. Verify the default icon renders as an SVG element
3. Test all size variants (xs, sm, md, lg, xl) by clicking size buttons
4. For each size, verify the SVG width/height attributes are correct
5. Test rotation: click 90°, 180°, 270° buttons
6. Verify transform CSS property updates with rotation
7. Test color variants: primary, secondary, accent, neutral
8. Take screenshots of all combinations
9. Test accessibility: verify SVG has role="img" and aria-label
```

### Tag Component

```
Using chrome-devtools MCP:

1. Navigate to http://localhost:3000/components/tag
2. Verify tags render with correct badge classes
3. Test all color variants appear correctly
4. Test size variants (sm, md, lg)
5. Click the close button on a removable tag
6. Verify the tag is removed from the DOM
7. Test disabled state - verify opacity reduces and interactions are disabled
8. Take screenshots of all variants
9. Test keyboard navigation with Tab and Enter keys
```

### Persona Component

```
Using chrome-devtools MCP:

1. Navigate to http://localhost:3000/components/persona
2. Verify persona has avatar, name text, and status badge
3. Test all size variants (sm, md, lg)
4. Verify avatar image loads and displays correctly
5. Test presence indicators (online, offline, away, busy)
6. Verify status badge colors match presence type
7. Test with and without secondary text
8. Take screenshots of all variants
9. Verify text alignment and spacing
```

### ThemeExportImport

```
Using chrome-devtools MCP:

1. Navigate to the theme export/import demo
2. Click "Export Theme" button
3. Verify a success message or badge appears
4. Check that a download was triggered (check Downloads)
5. Click "Copy to Clipboard" button
6. Verify clipboard contains valid JSON using: await navigator.clipboard.readText()
7. Parse the JSON and verify it has baseTheme and overrides properties
8. Test import: create a test JSON file and upload it
9. Verify the theme updates after import
10. Test error handling: upload invalid JSON, verify error message displays
```

### LoadingBar

```
Using chrome-devtools MCP:

1. Navigate to http://localhost:3000/components/loading_bar
2. Set progress to 0%, verify bar is empty
3. Set progress to 50%, verify bar fills to halfway
4. Set progress to 100%, verify bar is completely filled
5. Test all color variants (primary, secondary, accent, success, warning, error)
6. Test indeterminate mode - verify animation is running
7. Measure fill width vs container width to verify accuracy
8. Test size variants if available
9. Take screenshots at 25%, 50%, 75%, 100% progress
```

### MessageBar

```
Using chrome-devtools MCP:

1. Navigate to http://localhost:3000/components/message_bar
2. Test all message types: info, success, warning, error
3. Verify correct alert classes and icon colors for each type
4. Click the close button, verify message dismisses
5. Test auto-dismiss: trigger a 3-second timeout message
6. Wait 3.5 seconds, verify message auto-dismisses
7. Test with action buttons - verify buttons render and are clickable
8. Test multiline messages and long text wrapping
9. Take screenshots of all message types
10. Verify accessibility: check for role="alert" and appropriate aria attributes
```

## Generating Test Report

After running all tests, create a report:

```
Please generate a test report with:

1. Summary of all components tested
2. Pass/Fail status for each test case
3. Screenshots attached for visual verification
4. List of any bugs or issues found
5. Performance metrics (render times if measured)
6. Accessibility findings
7. Recommendations for improvements

Format the report in markdown and save to tests/visual/reports/YYYYMMDD_test_report.md
```

## Automated Test Execution

For automated execution, use this prompt:

```
Using chrome-devtools MCP, execute the complete visual test suite:

1. For each component [BaseThemeSelector, ColorCustomizer, TypographyCustomizer, Icon, Tag, Persona, ThemeExportImport, LoadingBar, MessageBar]:
   a. Navigate to the component demo page
   b. Execute all test cases listed in run_visual_tests.md
   c. Take baseline screenshots
   d. Verify all functionality
   e. Check accessibility
   f. Measure performance

2. Generate a comprehensive test report
3. Create a summary table showing pass/fail for each component
4. List all issues found with severity (critical, major, minor)
5. Provide recommendations for fixes

Save all screenshots to tests/visual/screenshots/YYYYMMDD/
Save the report to tests/visual/reports/YYYYMMDD_comprehensive_report.md
```

## Visual Regression Testing

To check for visual regressions:

```
Using chrome-devtools MCP:

1. Load baseline screenshots from tests/visual/baselines/
2. For each component:
   a. Navigate to component demo
   b. Take new screenshot with same viewport size
   c. Compare with baseline
   d. Calculate pixel difference percentage
   e. Report if diff > 1%

3. Generate visual regression report showing:
   - Components with no changes (diff < 0.1%)
   - Components with minor changes (0.1% < diff < 1%)
   - Components with major changes (diff > 1%)
   - Side-by-side comparison images for changed components
```

## CI/CD Integration

For continuous integration, create a script that:

1. Starts demo server in background
2. Waits for server readiness
3. Runs all visual tests via Chrome DevTools MCP
4. Compares screenshots with baselines
5. Generates test report
6. Fails build if critical issues or visual regressions > 1%
7. Uploads test artifacts (screenshots, reports)

## Troubleshooting

### Demo Server Not Responding
- Verify trunk serve is running
- Check http://localhost:3000 loads in browser
- Check console for build errors

### Chrome DevTools MCP Not Available
- Verify MCP server configured in Claude Code settings
- Check MCP server is running
- Try restarting Claude Code

### Tests Failing Intermittently
- Add explicit wait times before interactions
- Check for network requests that may delay rendering
- Verify animations complete before taking screenshots

### Screenshots Don't Match Baselines
- Ensure same viewport size
- Check if fonts loaded completely
- Verify images loaded
- Check for animation timing issues

## Next Steps

1. Execute full test suite
2. Create baseline screenshots
3. Document all test results
4. File issues for bugs found
5. Update components based on findings
6. Re-run tests to verify fixes
7. Update baselines if designs changed intentionally
