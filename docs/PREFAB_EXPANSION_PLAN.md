# Prefab Expansion Implementation Plan (v0.6.x)

## Goals
- **Easy to use**: first‑class support in `ops` facade and pipeline steps.
- **Easy to configure**: JSON prefabs + minimal params + safe defaults.
- **Simple + logical**: small, incremental API changes with backward compatibility.

## Scope
- Expand prefab data model to allow multi‑symbol patterns and optional metadata.
- Improve placement options (modes + tag filtering) without adding game logic.
- Emit optional markers/masks (game‑agnostic) for downstream consumers.

## Non‑Goals
- No game‑specific spawning logic inside terrain‑forge.
- No gameplay item/enemy selection inside the library.

---

## Phase 1 — Data Model + Schema (Backward‑compatible)

### Changes
- Extend `PrefabData` to support:
  - `legend` (optional): map symbol → tile/marker/mask.
  - Keep `pattern` as `Vec<String>`.
- Default behavior remains: `.` = floor, `#` = wall/empty.

### Backward compatibility
- If `legend` is missing, interpret `.`/`#` exactly as today.
- Existing `prefab_library.json` and demo files continue to load unchanged.

---

## Phase 2 — Ops Facade Usability

### New `prefab` params (all optional)
- `library_path`: JSON file path for prefab library.
- `library_paths`: list of JSON files to load and merge.
- `library_dir`: directory containing JSON prefab libraries.
- `tags`: string or list, to filter selection.
- `max_prefabs`, `min_spacing`, `allow_rotation`, `allow_mirroring`, `weighted_selection` (existing).
- `placement_mode`: `overwrite | merge | paint_floor | paint_wall`.

### Behavior
- Inline `prefabs` remain supported (current config style).
- If no prefabs provided, fall back to a simple default prefab.

---

## Phase 3 — Placement Rules (Simple + Predictable)

### Placement modes
- **overwrite**: always apply prefab tiles.
- **merge**: only place on walls/empty tiles.
- **paint_floor**: only apply to floor cells (no changes to walls).
- **paint_wall**: only apply to wall cells.

### Constraints (minimal)
- Tag filter (any of the listed tags).
- Keep `min_spacing` logic unchanged.

---

## Phase 4 — Marker + Mask Emission

### Marker support
- Allow prefab cells to emit marker tags (e.g., `loot_slot`, `npc_slot`).
- Markers are emitted into semantic layers; **no spawning inside terrain‑forge**.

### Mask support
- Optional mask cells (e.g., `no_spawn`, `reserved`) for downstream use.

---

## Phase 5 — Library Ergonomics

### Loading improvements
- Support loading multiple JSON prefab files (list or folder).
- Add tag index rebuild when loading.

---

## Phase 6 — Docs + Examples

- Update `examples/advanced_prefabs.rs` to show legend + markers.
- Add prefab schema to `docs/API.md`.
- Include ops example in `docs/DEMO_FRAMEWORK_OVERVIEW.md`.

---

## Suggested Order of Implementation (Concrete Steps)

1) **Phase 2 first** (Ops facade + library loading + tag filters).
2) **Phase 3** (placement modes + weighted selection toggle).
3) **Phase 1** (legend schema + parsing; keep default behavior).
4) **Phase 4** (marker/mask emission).
5) **Phase 5** (multi‑file library loading).
6) **Phase 6** (docs + examples).

This order keeps usability improvements available early while schema changes mature.
