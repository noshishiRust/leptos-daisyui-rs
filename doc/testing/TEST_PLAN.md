# Component Testing Plan

This document outlines the comprehensive testing strategy for all leptos-daisyui-rs components, including visual testing using Chrome DevTools MCP and Claude for Chrome.

## Testing Categories

### 1. Unit Tests
- Component rendering
- Props validation
- Signal reactivity
- Event handlers
- Edge cases

### 2. Integration Tests
- Component interactions
- Theme context integration
- State management
- localStorage persistence

### 3. Visual Tests
- Rendering correctness
- Responsive behavior
- Theme application
- Accessibility

### 4. Browser Tests (Chrome DevTools MCP)
- DOM structure validation
- CSS class application
- JavaScript functionality
- User interactions

---

## New Components Testing Status

### Theming Components

#### 1. BaseThemeSelector

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Renders all 32 themes
- [ ] Theme selection updates context
- [ ] Callback fires on theme change
- [ ] Current theme shows visual indicator

**Visual Tests:**
- [ ] Grid layout responsive (2/3/4 columns)
- [ ] Selected theme has border and ring
- [ ] Hover effects work
- [ ] Color swatches display correctly

**Chrome DevTools Tests:**
```javascript
// Test theme selection
await click('.card[data-theme="cyberpunk"]');
await waitFor(() => document.documentElement.getAttribute('data-theme') === 'cyberpunk');

// Verify visual feedback
const selectedCard = document.querySelector('.card.border-primary');
assert(selectedCard !== null, 'Selected theme should have border-primary class');
```

---

#### 2. ColorCustomizer

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Color picker updates theme config
- [ ] Hex to OKLCH conversion
- [ ] Reset button clears customizations
- [ ] Preview shows live changes

**Visual Tests:**
- [ ] Color input shows current value
- [ ] Reset button per color category
- [ ] Live preview updates immediately
- [ ] Accordion sections expand/collapse

**Chrome DevTools Tests:**
```javascript
// Test color customization
await fillInput('[data-color="primary"]', '#3b82f6');
await waitFor(() => getComputedStyle(document.documentElement)
  .getPropertyValue('--p').includes('oklch'));

// Test reset
await click('[data-reset="primary-colors"]');
const primaryColor = getComputedStyle(document.documentElement).getPropertyValue('--p');
assert(!primaryColor.includes('oklch(0.'), 'Color should be reset');
```

---

#### 3. TypographyCustomizer

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Font family selection
- [ ] Base font size slider (14-20px)
- [ ] Type scale ratio slider (1.125-1.414)
- [ ] Preview updates reactively

**Visual Tests:**
- [ ] Font family dropdown shows options
- [ ] Slider values display correctly
- [ ] Preview text scales properly
- [ ] Reset restores defaults

**Chrome DevTools Tests:**
```javascript
// Test typography changes
await selectOption('[data-select="heading-font"]', 'Inter, sans-serif');
await setSliderValue('[data-slider="base-size"]', 16);

const bodyFont = getComputedStyle(document.body).fontFamily;
assert(bodyFont.includes('Inter'), 'Font family should be applied');
```

---

#### 4. BorderCustomizer

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Border width slider (0-8px)
- [ ] Border radius presets
- [ ] Spacing scale (0.5x-2.0x)
- [ ] Preview components update

**Visual Tests:**
- [ ] Slider labels show current values
- [ ] Preview input/button reflect changes
- [ ] Reset clears all customizations

**Chrome DevTools Tests:**
```javascript
// Test border customization
await setSliderValue('[data-slider="border-width"]', 2);
const inputBorder = getComputedStyle(document.querySelector('.preview .input'))
  .borderWidth;
assert(inputBorder === '2px', 'Border width should be 2px');
```

---

#### 5. ComponentCustomizer

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Button border radius selection
- [ ] Card border radius selection
- [ ] Input border width selection
- [ ] Preview updates per component

**Visual Tests:**
- [ ] Component sections render
- [ ] Select dropdowns show options
- [ ] Live preview shows changes
- [ ] Reset button works

**Chrome DevTools Tests:**
```javascript
// Test component override
await selectOption('[data-component="button"] select', '9999px');
const buttonRadius = getComputedStyle(document.querySelector('.preview .btn'))
  .borderRadius;
assert(buttonRadius === '9999px', 'Button should be pill-shaped');
```

---

#### 6. ThemeExportImport

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Export generates valid JSON
- [ ] Import parses JSON correctly
- [ ] Import validates theme structure
- [ ] Download triggers file save

**Visual Tests:**
- [ ] Export button downloads file
- [ ] Import file input accepts JSON
- [ ] Status messages display
- [ ] Error handling shows feedback

**Chrome DevTools Tests:**
```javascript
// Test export
await click('[data-action="export"]');
await waitFor(() => document.querySelector('.badge-success'));

// Test import (requires file upload simulation)
const fileInput = document.querySelector('input[type="file"]');
const testTheme = { baseTheme: 'dark', overrides: {} };
await uploadFile(fileInput, new File([JSON.stringify(testTheme)], 'theme.json'));
```

---

#### 7. ThemeShare

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Generate shareable URL
- [ ] Base64 encoding/decoding
- [ ] Copy to clipboard
- [ ] Load from URL hash

**Visual Tests:**
- [ ] Share button generates URL
- [ ] Copy button works
- [ ] Success feedback displays
- [ ] URL loads theme correctly

**Chrome DevTools Tests:**
```javascript
// Test share URL generation
await click('[data-action="share"]');
const shareUrl = document.querySelector('[data-share-url]').value;
assert(shareUrl.includes('#theme='), 'URL should contain theme hash');

// Test clipboard
await click('[data-action="copy"]');
const clipboardText = await navigator.clipboard.readText();
assert(clipboardText === shareUrl, 'Clipboard should contain share URL');
```

---

#### 8. PresetThemesGallery

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Loads 6 preset themes
- [ ] Apply button updates theme
- [ ] Preview shows theme colors
- [ ] Descriptions render

**Visual Tests:**
- [ ] Grid layout (2 columns)
- [ ] Color swatches display
- [ ] Apply button per theme
- [ ] Hover effects

**Chrome DevTools Tests:**
```javascript
// Test preset application
await click('[data-preset="vibrant"] .btn-primary');
const primary = getComputedStyle(document.documentElement).getPropertyValue('--p');
assert(primary.length > 0, 'Primary color should be set');
```

---

### Specialized Components

#### 9. Icon

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Renders SVG icon
- [ ] Size variants (xs, sm, md, lg, xl)
- [ ] Color props apply
- [ ] Rotation prop works

**Visual Tests:**
- [ ] Icon displays correctly
- [ ] Sizes scale properly
- [ ] Colors apply via classes
- [ ] Rotation transforms

**Chrome DevTools Tests:**
```javascript
// Test icon rendering
const icon = document.querySelector('[data-component="icon"]');
assert(icon.tagName === 'svg', 'Icon should be SVG element');

// Test size
await click('[data-icon-size="lg"]');
const size = icon.getAttribute('width');
assert(size === '32', 'Large icon should be 32px');
```

---

#### 10. Tag

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Renders with text
- [ ] Color variants
- [ ] Size variants
- [ ] Removable with callback
- [ ] Disabled state

**Visual Tests:**
- [ ] Tag displays inline
- [ ] Colors apply correctly
- [ ] Close button appears when removable
- [ ] Disabled opacity

**Chrome DevTools Tests:**
```javascript
// Test tag rendering
const tag = document.querySelector('.badge');
assert(tag !== null, 'Tag should render');

// Test removal
await click('.badge .btn-close');
await waitFor(() => !document.querySelector('.badge'));
```

---

#### 11. Persona

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Renders avatar with text
- [ ] Size variants
- [ ] Status indicator
- [ ] Presence badge
- [ ] Secondary text

**Visual Tests:**
- [ ] Avatar image displays
- [ ] Text alignment
- [ ] Status colors
- [ ] Badge positioning

**Chrome DevTools Tests:**
```javascript
// Test persona rendering
const persona = document.querySelector('[data-component="persona"]');
const avatar = persona.querySelector('.avatar');
const text = persona.querySelector('.persona-text');
assert(avatar && text, 'Persona should have avatar and text');
```

---

#### 12. ConfigProvider

**Status**: ✅ Has Tests

**Unit Tests:** ✓ Implemented

**Visual Tests Needed:**
- [ ] Provides config to children
- [ ] Nested providers work
- [ ] Theme override cascading

**Chrome DevTools Tests:**
```javascript
// Test config provider context
const configuredElement = document.querySelector('[data-config-consumer]');
const config = configuredElement._leptos_config;
assert(config !== undefined, 'Config should be provided');
```

---

#### 13. LoadingBar

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Progress value updates
- [ ] Color variants
- [ ] Size variants
- [ ] Indeterminate mode
- [ ] Animation

**Visual Tests:**
- [ ] Bar fills to correct percentage
- [ ] Colors apply
- [ ] Smooth transitions
- [ ] Indeterminate animation

**Chrome DevTools Tests:**
```javascript
// Test loading bar
await setProgress(75);
const bar = document.querySelector('.loading-bar-fill');
const width = getComputedStyle(bar).width;
const parentWidth = getComputedStyle(bar.parentElement).width;
const percentage = (parseFloat(width) / parseFloat(parentWidth)) * 100;
assert(Math.abs(percentage - 75) < 1, 'Progress should be 75%');
```

---

#### 14. MessageBar

**Status**: ⚠️ Needs Tests

**Unit Tests Needed:**
- [ ] Renders message
- [ ] Type variants (info, success, warning, error)
- [ ] Closeable with callback
- [ ] Auto-dismiss timer
- [ ] Action buttons

**Visual Tests:**
- [ ] Message displays
- [ ] Icon matches type
- [ ] Colors match severity
- [ ] Close button works
- [ ] Actions render

**Chrome DevTools Tests:**
```javascript
// Test message bar
const messageBar = document.querySelector('.alert');
assert(messageBar.classList.contains('alert-info'), 'Should have info type');

// Test auto-dismiss
await waitFor(3000);
assert(!document.querySelector('.alert'), 'Should auto-dismiss after timeout');

// Test close
await click('.alert .btn-close');
assert(!document.querySelector('.alert'), 'Should close on button click');
```

---

## Visual Testing Guidelines

### Using Chrome DevTools MCP

1. **Setup**
```bash
# Install chrome-devtools MCP server (if not already)
# Configure in ~/.claude/mcp_servers.json
```

2. **Test Execution**
```javascript
// Navigate to demo page
await navigate('http://localhost:3000/components/base_theme_selector');

// Wait for component to load
await waitFor('.grid.grid-cols-2');

// Perform actions
await click('[data-theme="cyberpunk"]');

// Verify results
const theme = document.documentElement.getAttribute('data-theme');
assert(theme === 'cyberpunk');
```

3. **Screenshot Verification**
```javascript
// Take screenshot
const screenshot = await takeScreenshot();

// Compare with baseline (visual regression)
const diff = await compareScreenshots(screenshot, 'baseline/base_theme_selector.png');
assert(diff < 0.01, 'Visual diff should be < 1%');
```

### Using Claude for Chrome Extension

1. **Manual Testing Steps**
   - Open demo page in Chrome
   - Activate Claude for Chrome extension
   - Ask Claude to verify component behavior
   - Claude can inspect DOM, trigger events, read computed styles

2. **Example Test Prompts**
```
"Click the 'cyberpunk' theme card and verify the data-theme attribute changes"

"Test all color customizer inputs and verify CSS variables update"

"Take a screenshot of the typography customizer and compare font sizes"
```

---

## Test Harness Structure

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_renders() {
        // Arrange
        let component = create_component();

        // Act
        let view = component.into_view();

        // Assert
        assert!(view.is_some());
    }

    #[test]
    fn test_props_apply() {
        // Test prop validation
    }

    #[test]
    fn test_signal_reactivity() {
        // Test reactive updates
    }
}
```

### Integration Test Template

```rust
// tests/integration/component_name.rs
#[cfg(test)]
mod component_integration_tests {
    use leptos::*;
    use leptos_daisyui_rs::components::*;

    #[test]
    fn test_theme_context_integration() {
        // Test component with ThemeProvider
    }
}
```

### Visual Test Script Template

```javascript
// tests/visual/component_name.test.js
describe('ComponentName Visual Tests', () => {
    beforeEach(async () => {
        await navigate('http://localhost:3000/components/component-name');
    });

    it('should render correctly', async () => {
        const element = await waitFor('[data-testid="component"]');
        expect(element).toBeTruthy();
    });

    it('should match baseline screenshot', async () => {
        const screenshot = await takeScreenshot();
        const diff = await compareScreenshots(screenshot, 'baseline.png');
        expect(diff).toBeLessThan(0.01);
    });
});
```

---

## Priority Testing Roadmap

### Phase 1: Critical Components (High Priority)
1. ✅ ConfigProvider - Has tests
2. ⚠️ BaseThemeSelector - Add unit + visual tests
3. ⚠️ ColorCustomizer - Add unit + visual tests
4. ⚠️ ThemeExportImport - Add unit + integration tests

### Phase 2: User-Facing Components (Medium Priority)
5. ⚠️ Icon - Add unit + visual tests
6. ⚠️ Tag - Add unit + visual tests
7. ⚠️ Persona - Add unit + visual tests
8. ⚠️ MessageBar - Add unit + integration tests

### Phase 3: Advanced Features (Lower Priority)
9. ⚠️ TypographyCustomizer - Add visual tests
10. ⚠️ BorderCustomizer - Add visual tests
11. ⚠️ ComponentCustomizer - Add visual tests
12. ⚠️ ThemeShare - Add integration tests
13. ⚠️ PresetThemesGallery - Add visual tests
14. ⚠️ LoadingBar - Add visual tests

---

## Running Tests

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration
```

### Visual Tests (Chrome DevTools MCP)
```bash
# Start demo server
cd demo && trunk serve

# In another terminal, run visual tests
claude-code --mcp chrome-devtools -- "Run visual tests for BaseThemeSelector"
```

### Manual Visual Verification
```bash
# Start demo
cd demo && trunk serve

# Open http://localhost:3000
# Use Claude for Chrome extension to verify components
```

---

## Success Criteria

### For Each Component

✅ **Unit Tests**
- Renders without errors
- Props validated
- Edge cases handled
- >80% code coverage

✅ **Integration Tests**
- Context integration works
- State management correct
- localStorage persistence (if applicable)

✅ **Visual Tests**
- Renders correctly across breakpoints
- Theme applies properly
- Accessibility standards met
- Visual regression < 1% diff

✅ **Documentation**
- Test plan documented
- Chrome DevTools test script provided
- Manual testing steps included
- Known issues listed

---

## Continuous Integration

### CI Pipeline (Future)
1. Run unit tests (`cargo test`)
2. Run clippy (`cargo clippy -- -D warnings`)
3. Build demo (`trunk build`)
4. Run visual regression tests
5. Generate coverage report

---

## Notes for Claude Code

When testing components with Chrome DevTools MCP:

1. Always start demo server first: `cd demo && trunk serve`
2. Navigate to specific component: `http://localhost:3000/components/{name}`
3. Wait for component load before interactions
4. Verify reactive updates with signal changes
5. Take screenshots for visual regression
6. Test responsive behavior at different viewports
7. Verify accessibility with screen reader

---

## Contributing Tests

When adding tests for new components:

1. Create unit test file in component directory
2. Add integration test in `tests/integration/`
3. Create visual test script in `tests/visual/`
4. Update this TEST_PLAN.md with test cases
5. Add baseline screenshots to `tests/visual/baselines/`
6. Document any special testing requirements

---

## Test Status Legend

- ✅ Tests implemented and passing
- ⚠️ Tests needed
- 🚧 Tests in progress
- ❌ Tests failing
- ⏭️ Tests skipped/not applicable
