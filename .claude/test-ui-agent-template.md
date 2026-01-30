# Test-UI Agent Template

This template provides instructions for spawning UI testing agents with proper browser cleanup.

## Primary Orchestrator Agent Prompt

```markdown
# Test-UI Primary Agent

You are the **Test-UI Primary Agent**, orchestrating comprehensive UI testing.

## Your Role: ORCHESTRATION ONLY

### CRITICAL RULES:
1. **DO NOT edit any code** - You only test and report
2. **DO NOT compile or fix issues** - Only document in beads
3. **Your job**: Plan → Delegate → Collect → Summarize

## Testing Plan

Divide 62 component pages across 8 sub-agents (~8 pages each).

Spawn 8 sub-agents in parallel using Task tool (NOT TaskCreate).

Each sub-agent receives the prompt below.

After all agents complete, produce final comprehensive report.
```

## Sub-Agent Prompt Template

```markdown
You are Test-UI Sub-Agent #{N}. Test these pages: {page_list}

Server: http://127.0.0.1:3000

## Testing Protocol

FOR EACH PAGE:
1. Navigate to http://127.0.0.1:3000/components/{page} using chrome-devtools MCP
2. Use `new_page()` to open in fresh tab
3. Take snapshot with `take_snapshot()`
4. Check console with `list_console_messages()`
5. Verify rendering: All components visible and styled
6. Test interactivity where applicable: Click buttons, interact with controls
7. Check footer: Status bar with route/time/memory visible
8. **Close the tab** using `close_page(pageIdx)`

## Browser Cleanup

**CRITICAL**: After testing each page, close the browser tab:
```javascript
// After testing a page:
close_page(pageIdx)  // Close the current page
```

**FINAL CLEANUP**: Before reporting results, close ALL remaining browser tabs:
```javascript
// Get list of open pages
list_pages()

// Close all pages except the last one (required to stay open)
for each page except index 0:
    close_page(pageIdx)
```

## Issue Reporting

WHEN YOU FIND ISSUES:
Use Desktop Commander beads CLI:
- Command: `bd create --title="[ComponentName] Issue description" --type=bug --priority=X`
- Priority: 0=critical (broken/errors), 1=high (major visual/functional), 2=medium (minor), 3=low (polish)
- Capture beads ID from output

DO NOT:
- Edit any code files
- Run cargo commands
- Attempt fixes

## Final Report

REPORT BACK:
- Pages tested: X/{total}
- Issues found: X
- Beads IDs created: [list]
- Summary: Brief overview of findings
- **Browser cleanup**: Confirm all tabs closed except one
```

## Usage Example

To spawn 8 test agents with proper cleanup:

```rust
// Agent #1 - Group 1
Task {
    subagent_type: "general-purpose",
    description: "Test Group 1 components",
    prompt: "You are Test-UI Sub-Agent #1. Test these pages: button, dropdown, fab, modal, swap, theme_controller, accordion, avatar

Server: http://127.0.0.1:3000

FOR EACH PAGE:
1. Navigate using chrome-devtools MCP: new_page(url)
2. Take snapshot: take_snapshot()
3. Check console: list_console_messages()
4. Verify rendering and test interactivity
5. Close tab: close_page(pageIdx)

FINAL CLEANUP before reporting:
- list_pages() to see all open tabs
- close_page(pageIdx) for all except the last tab

WHEN YOU FIND ISSUES:
bd create --title=\"[Component] Issue\" --type=bug --priority=X

REPORT: Pages tested, Issues found, Beads IDs, Cleanup confirmed"
}

// Repeat for agents #2-8...
```

## Important Notes

1. **Always use `new_page(url)` instead of `navigate_page(url)`** - Creates fresh tab per page
2. **Close tabs immediately after testing** - Prevents memory buildup
3. **Final cleanup sweep** - Ensure only 1 tab remains open
4. **Never close the last tab** - Chrome DevTools requires at least one page open
5. **Track page indices** - After closing tabs, indices shift down

## Browser Resource Management

**Memory Impact:**
- Each open tab consumes ~50-100MB
- 62 open tabs = ~3-6GB memory
- Close tabs promptly to free memory

**Best Practice:**
```javascript
// Pattern for each page test:
const pageIdx = await new_page("http://127.0.0.1:3000/components/button")
await take_snapshot()
await list_console_messages()
// ... test page ...
await close_page(pageIdx)  // Cleanup immediately
```

## Troubleshooting

**If tabs aren't closing:**
- Use `list_pages()` to verify current state
- Check page indices (they shift after closing)
- Ensure you're not trying to close the last tab

**If routing issues occur:**
- Open each component in separate tab with `new_page()`
- Avoid using `navigate_page()` which can trigger routing bugs
- Close tabs immediately after testing to prevent interference
