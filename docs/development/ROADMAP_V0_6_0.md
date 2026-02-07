# Roadmap v0.6.0 (Usability + API Consolidation)

This roadmap targets usability first, simplicity second, and reduced object‑oriented bloat
throughout the API surface.

## Status summary

Implemented in code:
- Unified ops facade + registry (`src/ops.rs`).
- Demo config delegated to ops (no duplicate parsing).
- Unified pipeline (`pipeline::Pipeline`) executing ops with full interoperability.
- Blend/combine alignment with `Difference` + new grid transforms (`invert`, `resize`).
- Legacy conditional pipeline effects now execute for real.
- Docs updated to use `ops::*` and unified pipeline across README/USAGE/API.
- Migration guide added (`docs/MIGRATION_0_6.md`).

Remaining work:
- Noise usability fixes and parameters.
- Deprecation plan for `compose::Pipeline` and older pipeline APIs.

## Goals

- Make common tasks trivial: generate, effect, combine, pipeline.
- Remove object‑graph heavy entry points in favor of small functional calls.
- Ensure *every* op is usable directly and in pipelines.

## Implemented milestones

### M1: Unified public façade

- `ops::generate(name, grid, seed?, params?)`
- `ops::effect(name, grid, params?, semantic?)`
- `ops::combine(mode, a, b)`
- `Params = HashMap<String, serde_json::Value>`

### M2: Registry promotion

- Name+params parsing migrated from demo into `ops`.
- Algorithms and effects share a single dispatch surface.

### M3: Pipeline unification

- `pipeline::Pipeline` executes `Step::Algorithm`, `Step::Effect`, `Step::Combine`, `Step::If`.
- Combine supports algorithms, saved grids, or inline grids.
- `Pipeline` implements `Algorithm<Tile>` for drop‑in use.
- Legacy `ConditionalPipeline` now executes effects via `ops`.

### M4: Grid ops and simple transforms

- Unary: `invert`, `resize` (crop/pad).
- Binary: `combine` supports `union`, `intersect`, `difference`.
- `BlendMode` extended with `Difference`.

## Remaining milestones

### M5: Noise + semantics

- Add scale/range controls to noise fill.
- Clarify/validate threshold semantics.
- Optional range‑based fill (`min..max`).

### M6: Docs + deprecations

- Update docs to show `ops::*` and `pipeline::Pipeline` patterns. ✅
- Add migration guide for v0.6.0. ✅
- Mark `compose::Pipeline` and old conditional pipeline APIs as legacy. (pending)

## Expected outcomes

- Users can call `ops::generate` / `ops::effect` without touching trait objects.
- Pipelines are simple to build and accept the same ops as direct usage.
- Layered blending and set operations are unified and discoverable.
- Demo and library stay in sync through a single registry.
