# Visual UI Testing Guide - Complete Strategy

This guide documents the complete visual UI testing strategy for web applications using Chrome DevTools MCP, including all workarounds and shortcuts discovered through production use.

## Overview

Test all UI components visually using Chrome browser automation via the `chrome-devtools` MCP server. This approach validates:
- Component rendering (layout, styling, visibility)
- Interactive functionality (clicks, inputs, state changes)
- Console errors/warnings
- Browser compatibility

## Quick Start

1. **Start dev server**: Ensure `trunk serve` or equivalent is running
2. **Verify server ready**: Check http://127.0.0.1:3000 responds
3. **Load MCP tools**: `ToolSearch` for "chrome-devtools" tools
4. **Spawn test agents**: 8-10 parallel agents, ~8 routes each
5. **Collect results**: Aggregate findings and create beads issues
6. **Test-fix-test**: Iterate until all tests pass

## Architecture

### Orchestration Pattern

```
Primary Agent (Orchestrator)
    ├─> Sub-Agent #1: Routes 1-8
    ├─> Sub-Agent #2: Routes 9-16
    ├─> Sub-Agent #3: Routes 17-24
    ├─> Sub-Agent #4: Routes 25-32
    ├─> Sub-Agent #5: Routes 33-40
    ├─> Sub-Agent #6: Routes 41-48
    ├─> Sub-Agent #7: Routes 49-56
    └─> Sub-Agent #8: Routes 57-64
```

**Orchestrator Role**: Plan → Delegate → Collect → Summarize (NO code editing)
**Sub-Agent Role**: Test → Report → Cleanup (NO code editing)

## Testing Protocol

### Per-Page Workflow

```javascript
// 1. Open fresh tab
pageIdx = new_page("http://127.0.0.1:3000/components/button")

// 2. Take snapshot
snapshot = take_snapshot()

// 3. Check console
messages = list_console_messages()

// 4. Verify rendering
// - All components visible
// - Proper styling applied
// - Layout correct
// - Interactive elements functional

// 5. Test interactivity (where applicable)
// - Click buttons
// - Toggle switches
// - Fill inputs
// - Trigger state changes

// 6. Check footer/status
// - Route display
// - Memory usage
// - Timestamp

// 7. Close tab immediately (critical for memory)
close_page(pageIdx)
```

### Final Cleanup

```javascript
// Before reporting, clean up all tabs
pages = list_pages()

// Close all except last tab (Chrome requires ≥1 page)
for (let i = 0; i < pages.length - 1; i++) {
    close_page(i)
}

// Verify: Should have exactly 1 tab remaining
```

## Key Learnings & Workarounds

### 1. Chrome DevTools MCP Snapshot Caching Issue

**Problem**: `take_snapshot()` can return stale/cached content from previous pages, causing false positives.

**Symptoms**:
- Snapshot shows content from a different route
- Page title doesn't match expected component
- Layout looks like wrong page

**Root Cause**: Chrome DevTools MCP caches accessibility tree snapshots and doesn't always invalidate on navigation.

**Workarounds**:
1. **Use `new_page()` instead of `navigate_page()`** - Fresh tab = fresh snapshot
2. **Cross-reference with console messages** - If console shows correct route, snapshot may be stale
3. **Manual verification** - For suspected false positives, manually test the route
4. **Mark false positives** - Close issues with reason explaining snapshot cache bug

**Example False Positive**:
```bash
# Issue: "Countdown page showing Range component"
# Reality: Routing worked fine, snapshot was cached from previous page
# Solution: Close with reason "Chrome DevTools MCP snapshot caching - false positive"
```

### 2. Browser Memory Management

**Problem**: Each open tab consumes 50-100MB. 77 tabs = 3-6GB memory.

**Solution**: Close tabs immediately after testing.

```javascript
// Good: Close after each test
pageIdx = new_page(url)
// ... test page ...
close_page(pageIdx)  // ✓ Immediate cleanup

// Bad: Keep all tabs open
for (url in urls) {
    new_page(url)  // ✗ Memory leak
}
```

**Best Practice**: Never have more than 2-3 tabs open simultaneously.

### 3. WASM vs Native Build Warnings

**Problem**: Some warnings only appear in WASM builds (`--target wasm32-unknown-unknown`), not in native `cargo check`.

**Example**:
```rust
// Warning in WASM only: unused import in cfg(target_arch = "wasm32") block
#[cfg(target_arch = "wasm32")]
{
    use wasm_bindgen::JsCast;  // May be unused
    // ...
}
```

**Solution**: Always test both build targets:
```bash
cargo check                                  # Native
cargo build --target wasm32-unknown-unknown  # WASM
```

### 4. File Lock Conflicts

**Problem**: Multiple concurrent cargo builds cause "Blocking waiting for file lock on build directory" errors.

**Symptoms**:
- Build hangs indefinitely
- Exit code 101
- "file lock on build directory" message

**Solutions**:
1. **Wait for current build to complete** before starting new one
2. **Kill competing processes**: `pkill -f "cargo|rustc|trunk"`
3. **Use background tasks** with proper timeout handling
4. **Coordinate agents**: Ensure only one build runs at a time

### 5. Dev Server Startup Timing

**Problem**: Tests start before dev server is fully ready, causing connection failures.

**Symptoms**:
- "Connection refused" errors
- "target machine actively refused" (Windows)
- 404 responses for valid routes

**Solution**: Verify server is ready before testing
```bash
# Poll until server responds
while ! curl -s http://127.0.0.1:3000 > /dev/null; do
    sleep 1
done

# Or wait fixed time (simpler but less reliable)
sleep 10
```

**Trunk-specific**: Watch for "server listening at" message in output.

### 6. Test-Fix-Test Cycle

**Strategy**: Iterate testing → fixing → re-testing until 100% pass.

```
Cycle 1: Test all → Find 6 issues → Create beads
Cycle 2: Fix issues → Re-test all → Find 2 more issues
Cycle 3: Fix remaining → Re-test all → 100% pass ✓
```

**Key Points**:
- **Create beads for all issues** during testing (no fixing yet)
- **Fix all issues** between test cycles
- **Re-test everything** each cycle (not just failed tests)
- **Track false positives** separately from real bugs
- **Commit after each successful cycle**

### 7. Beads Integration

**During Testing**: Create issues, don't fix
```bash
bd create --title="[Button] Click handler not firing" --type=bug --priority=1
bd create --title="[Modal] Background not dimmed" --type=bug --priority=2
```

**During Fixing**: Claim and resolve issues
```bash
bd update beads-xxx --status=in_progress --assignee=me
# ... make fixes ...
bd close beads-xxx
```

**Priorities**:
- **P0 (0)**: Critical - Component broken, errors, unusable
- **P1 (1)**: High - Major visual/functional issues
- **P2 (2)**: Medium - Minor issues, polish needed
- **P3 (3)**: Low - Suggestions, future improvements
- **P4 (4)**: Backlog - Nice-to-have enhancements

### 8. Parallel Testing Optimization

**Optimal Configuration**:
- **8-10 agents** for 60-80 routes
- **~8 routes per agent** for balanced load
- **Spawn all agents in parallel** using single message with multiple Task calls
- **General-purpose subagent** type works best

**Example**:
```rust
// Single message with 8 parallel Task calls
Task { subagent_type: "general-purpose", ... } // Agent #1
Task { subagent_type: "general-purpose", ... } // Agent #2
// ... 8 total
```

**Timing**: 8 agents testing 77 routes ≈ 15-30 minutes total.

## Common Issues & Solutions

### Issue: Routes Showing 404

**Possible Causes**:
1. Dev server not running
2. WASM build failed
3. Route not registered
4. Trunk cache corruption

**Solutions**:
```bash
# Check server is running
curl http://127.0.0.1:3000

# Check build logs
trunk build --release 2>&1 | grep error

# Rebuild clean
cd demo && cargo clean && trunk serve
```

### Issue: Styling Not Applied

**Possible Causes**:
1. Tailwind CSS not compiled
2. Classes not in `input.css`
3. daisyUI plugin not loaded
4. CSS purging removed classes

**Solutions**:
```bash
# Check Tailwind compiled
ls -lh demo/output.css  # Should exist and have reasonable size

# Check pre_build hook ran
grep "tailwindcss" demo/Trunk.toml

# Add missing classes
echo '@source inline("btn btn-primary ...")' >> demo/input.css
```

### Issue: Components Not Interactive

**Possible Causes**:
1. JavaScript not loaded (WASM)
2. Event handlers not registered
3. Component in read-only/disabled state
4. Browser compatibility issue

**Solutions**:
- Check console for WASM load errors
- Verify `wasm-bindgen` version compatibility
- Test in Chrome DevTools directly (not just MCP)
- Check component props: `disabled`, `read_only`, etc.

## Reporting Template

```markdown
# Visual UI Test Results

**Test Date**: 2026-01-31
**Routes Tested**: 77/77 (100%)
**Pass Rate**: 100%

## Summary
- ✅ All components render correctly
- ✅ Styling applied properly
- ✅ Interactive elements functional
- ✅ No console errors (critical)
- ⚠️ 2 console warnings (minor, P2/P3)

## Issues Found
1. **leptos-daisyui-rs-873** (P2): Reactive signal access outside tracking context
2. **leptos-daisyui-rs-fyh** (P3): performance.memory is not a function

## Components Tested
- accordion ✓
- alert ✓
- auto-complete ✓
... (all 77)

## Browser Cleanup
- Tabs opened: 77
- Tabs closed: 76
- Remaining: 1 ✓

## Next Steps
- Fix P2 console warning (reactive signal context)
- Consider fixing P3 (browser compatibility)
```

## Best Practices Summary

1. ✅ **Use `new_page()` for each route** - Fresh tab, fresh snapshot
2. ✅ **Close tabs immediately** - Memory management
3. ✅ **Verify server ready** - Avoid connection failures
4. ✅ **Test both native and WASM** - Catch all warnings
5. ✅ **Create beads during testing** - Don't fix yet
6. ✅ **Test-fix-test cycle** - Iterate to 100%
7. ✅ **Mark false positives** - Document snapshot cache issues
8. ✅ **Parallel agents** - 8-10 agents for optimal speed
9. ✅ **Final cleanup** - Leave exactly 1 tab open
10. ✅ **Commit after each cycle** - Track progress

## References

- Chrome DevTools Protocol: https://chromedevtools.github.io/devtools-protocol/
- MCP Chrome DevTools: https://github.com/modelcontextprotocol/servers/tree/main/src/chrome-devtools
- Beads CLI: https://github.com/cablehead/beads
