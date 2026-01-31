# Testing Documentation

This directory contains comprehensive testing documentation for leptos-daisyui-rs components.

## Documentation Files

### [TEST_PLAN.md](./TEST_PLAN.md)
Complete testing plan for all components including:
- Unit test requirements
- Integration test specifications
- Visual test guidelines
- Chrome DevTools test scripts
- Success criteria and priorities

### [CHROME_DEVTOOLS_TESTING.md](./CHROME_DEVTOOLS_TESTING.md)
Detailed guide for visual and functional testing using Chrome DevTools MCP:
- Setup instructions
- Component-specific test scripts
- Visual regression testing
- Accessibility testing
- Performance testing
- Automated test examples

## Quick Start

### Running Unit Tests
```bash
# Run all unit tests
cargo test

# Run tests for specific component
cargo test base_theme_selector

# Run tests with output
cargo test -- --nocapture
```

### Running Visual Tests

1. **Start Demo Server**
```bash
cd demo
trunk serve
```

2. **Use Claude Code with Chrome DevTools MCP**
   - See [run_visual_tests.md](../../tests/visual/run_visual_tests.md) for detailed instructions
   - Use prompts from CHROME_DEVTOOLS_TESTING.md

3. **Manual Testing**
   - Open http://localhost:3000 in Chrome
   - Navigate to component demos
   - Use Claude for Chrome extension to verify behavior

## Test Coverage Status

### Theming Components
- [x] BaseThemeSelector - Unit tests implemented ✅ (5 tests)
- [x] ColorCustomizer - Unit tests implemented ✅ (6 tests)
- [x] TypographyCustomizer - Unit tests implemented ✅ (7 tests)
- [x] BorderCustomizer - Unit tests implemented ✅ (7 tests)
- [x] ComponentCustomizer - Unit tests implemented ✅ (7 tests)
- [x] ThemeExportImport - Unit tests implemented ✅ (8 tests)
- [x] ThemeShare - Unit tests implemented ✅ (8 tests)
- [x] PresetThemesGallery - Unit tests implemented ✅ (11 tests)

### Specialized Components
- [x] Icon - Unit tests implemented ✅ (9 tests)
- [x] Tag - Unit tests implemented ✅ (12 tests)
- [x] Persona - Unit tests implemented ✅ (10 tests)
- [x] ConfigProvider - Unit tests implemented ✅ (6 tests)
- [x] LoadingBar - Unit tests implemented ✅ (12 tests)
- [x] MessageBar - Unit tests implemented ✅ (14 tests)

## Testing Tools

### Available Tools
1. **Rust Testing Framework** - Unit and integration tests
2. **Chrome DevTools MCP** - Browser automation and visual testing
3. **Claude for Chrome** - Interactive testing assistant
4. **Leptos Testing Utilities** - Component testing helpers

### Test Directories
```
leptos-daisyui-rs/
├── src/components/*/tests.rs     # Unit tests
├── tests/
│   ├── integration/               # Integration tests
│   └── visual/                    # Visual test scripts
│       ├── baselines/            # Baseline screenshots
│       ├── screenshots/          # Current screenshots
│       ├── reports/              # Test reports
│       └── run_visual_tests.md   # Execution guide
└── doc/testing/                   # Test documentation
    ├── README.md                  # This file
    ├── TEST_PLAN.md              # Comprehensive test plan
    └── CHROME_DEVTOOLS_TESTING.md # DevTools testing guide
```

## Test Execution Workflow

### 1. Before Making Changes
```bash
# Run unit tests
cargo test

# Take baseline screenshots
# (Use Chrome DevTools MCP - see run_visual_tests.md)
```

### 2. After Making Changes
```bash
# Run unit tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Take new screenshots and compare with baselines
# (Use Chrome DevTools MCP for visual regression)
```

### 3. Before Committing
```bash
# Ensure all tests pass
cargo test

# Ensure no clippy warnings
cargo clippy -- -D warnings

# Verify demo still works
cd demo && trunk serve
# Visit http://localhost:3000 and spot-check components
```

## Writing New Tests

### Unit Test Template
```rust
// src/components/my_component/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_renders() {
        // Arrange
        // Act
        // Assert
    }

    #[test]
    fn test_props_validation() {
        // Test prop behavior
    }
}
```

### Adding Tests to Component
```rust
// src/components/my_component/mod.rs

pub use component::*;

mod component;

#[cfg(test)]
mod tests;  // Add this line
```

## Visual Testing Best Practices

1. **Always test with demo server running**
2. **Use consistent viewport sizes** for screenshots
3. **Test responsive behavior** at mobile, tablet, desktop sizes
4. **Verify accessibility** with keyboard navigation and screen readers
5. **Check color contrast** meets WCAG AA standards
6. **Measure performance** for complex components
7. **Document test results** in test reports
8. **Update baselines** when designs change intentionally

## Continuous Integration

### Current Status
- ✅ Unit tests run on every build
- ✅ Clippy runs with strict warnings
- ⚠️ Visual regression tests - Manual execution
- ⚠️ Accessibility tests - Manual execution
- ⚠️ Performance tests - Not implemented

### Future Improvements
- [ ] Automated visual regression in CI
- [ ] Automated accessibility testing
- [ ] Performance budgets and monitoring
- [ ] Test coverage reporting
- [ ] Integration test automation

## Contributing Tests

When adding tests for new components:

1. Create `tests.rs` file in component directory
2. Add `#[cfg(test)] mod tests;` to `mod.rs`
3. Write unit tests for core functionality
4. Add visual test scenarios to TEST_PLAN.md
5. Create Chrome DevTools test script in CHROME_DEVTOOLS_TESTING.md
6. Run tests and verify they pass
7. Document any special testing requirements
8. Update this README with test status

## Test Reports

Test reports are stored in `tests/visual/reports/` with format:
- `YYYYMMDD_test_report.md` - Daily test runs
- `YYYYMMDD_comprehensive_report.md` - Full suite execution
- `YYYYMMDD_regression_report.md` - Visual regression results

## Getting Help

- **Test Plan Questions**: See TEST_PLAN.md
- **Chrome DevTools Setup**: See CHROME_DEVTOOLS_TESTING.md
- **Visual Test Execution**: See tests/visual/run_visual_tests.md
- **Component Issues**: File issue on GitHub
- **Testing Best Practices**: Refer to Rust testing docs

## Resources

- [Leptos Testing Guide](https://book.leptos.dev/testing/)
- [Rust Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/)
- [WCAG Accessibility Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [Visual Regression Testing](https://www.browserstack.com/guide/visual-regression-testing)

## Next Steps

1. Implement unit tests for remaining components (Priority 1)
2. Execute full visual test suite using Chrome DevTools MCP
3. Generate baseline screenshots for all components
4. Create automated CI pipeline for visual regression
5. Add integration tests for complex workflows
6. Implement accessibility automated testing
7. Set up performance monitoring and budgets
