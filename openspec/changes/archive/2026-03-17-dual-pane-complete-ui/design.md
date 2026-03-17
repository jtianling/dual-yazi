## Context

Currently, `root.lua` renders the UI as a vertical stack: `[Header(1 line) | Main(fill) | Status(1 line)]`. In dual-pane mode, `Main` becomes a `DualPane` component containing two `Tab` children, but Header and Status remain at the root level, always bound to `cx.active` (the active pane). This means the inactive pane has no visible directory path, mode indicator, or file permissions.

The Header and Status components already accept a `tab` parameter and are stateless renderers — they just need to be instantiated per-pane with the correct tab data.

## Goals / Non-Goals

**Goals:**
- Each pane in dual-pane mode has its own Header and Status bar
- Single-pane mode remains unchanged
- Minimal code changes — reuse existing Header and Status components as-is

**Non-Goals:**
- Redesigning Header or Status component internals
- Adding new information to Header or Status
- Changing visual styling of Header or Status

## Decisions

### Decision 1: Embed Header/Status inside DualPane rather than duplicating at Root

**Choice**: In dual-pane mode, the Root layout changes to `[DualPane(fill)]` (no root-level header/status). Each pane column inside DualPane becomes a vertical sub-layout: `[Header(1 line) | Tab(fill) | Status(1 line)]`.

**Alternative considered**: Keep root-level header/status and add a second set. Rejected because it would require splitting the single header/status line into left/right halves, which is more complex and wastes the separator column.

**Rationale**: DualPane already owns the horizontal split. Adding vertical sub-structure per pane is the natural extension. Header and Status are already parameterized by `tab`, so no changes to those components are needed.

### Decision 2: Root layout conditionally omits header/status rows

**Choice**: In `root.lua`, when `cx.tabs.single_pane` is false, the layout uses a single `Fill(1)` constraint instead of `[Length(1), Fill(1), Length(1)]`. The DualPane component takes full screen height.

**Rationale**: Avoids rendering empty header/status rows at root level that would waste 2 lines of vertical space.

## Risks / Trade-offs

- **[Risk] Plugins that modify root Header/Status via `children_add` won't affect dual-pane mode** → Acceptable trade-off. Plugins can be updated to hook into per-pane Header/Status. This is a known limitation of the dual-pane fork.
- **[Risk] 2 lines less vertical space per pane (header + status each take 1 line from pane height)** → No net change in total lines used. Root layout previously used 2 lines for global header/status. Now each pane uses 2 lines internally, but the root saves those 2 lines, so each pane loses only 1 line compared to before.
