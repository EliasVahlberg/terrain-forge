# Architecture Unification Notes

This document reflects the current unification work (v0.6.0 focus) and how the codebase has
been simplified to reduce object‑oriented bloat while keeping functionality intact.

## Current unified architecture (implemented)

### Ops facade + registry (`src/ops.rs`)

- `ops::generate`, `ops::effect`, `ops::combine` are the canonical entry points.
- The name+params registry previously in `demo/src/config.rs` now lives in the library
  (`ops::build_algorithm` + `ops::effect`).
- `Params` is a `HashMap<String, serde_json::Value>` for ergonomic, flexible config.

### Pipeline unification (`src/pipeline.rs`)

- New unified `pipeline::Pipeline` executes the same ops as `ops::*`.
- Steps are simple, data‑first enums:
  - `Step::Algorithm { name, seed?, params? }`
  - `Step::Effect { name, params? }`
  - `Step::Combine { mode, source }`
  - `Step::If { condition, then, else }`
  - `Step::StoreGrid`, `Step::Log`, `Step::SetParameter`
- Combine sources support prebuilt grids, saved grids, or algorithm specs.
- `Pipeline` implements `Algorithm<Tile>` so it can be used anywhere a generator is expected.

### Legacy pipeline support (still present, now functional)

- `ConditionalPipeline` remains for template/conditional workflows.
- `PipelineOperation::Effect` now dispatches through `ops::effect` (no longer a stub).

### Blend + grid ops alignment

- `BlendMode` now includes `Difference`.
- `ops::combine` uses the same semantics as layering and supports union/intersect/difference.
- `effects::invert` and `effects::resize` are now first‑class unary transforms.

### Demo config simplification

- `demo/src/config.rs` delegates algorithm/effect construction to `ops`.
- This removes duplicate parsing logic and keeps behavior aligned with the library API.

## Module map (current)

### Algorithms (`src/algorithms/*`)

- Still implement `Algorithm<Tile>` with concrete config structs.
- Access via `ops::build_algorithm` for name+params, or `algorithms::get` for defaults.

### Compose (`src/compose/*`)

- `LayeredGenerator` remains as a lightweight layering helper.
- `compose::Pipeline` is now effectively legacy; prefer `pipeline::Pipeline`.

### Effects (`src/effects/*`)

- Unary effects are dispatched through `ops::effect`.
- Binary set ops are unified in `ops::combine`.

### Noise (`src/noise/*`)

- Powerful but still not surfaced via a unified parameter schema beyond `noise_fill`.
- Further work needed to expose scale/range and clarify threshold semantics.

### Spatial (`src/spatial/*`)

- Remains separate; can be exposed via `ops::effect` over time if desired.

## Remaining gaps (next targets)

1. **Noise ergonomics**
   - Add scale/range options and clarify threshold behavior for `noise_fill`.
2. **Semantic requirements metadata**
   - Encode which ops need semantic layers so pipelines can validate up front.
3. **Legacy API cleanup**
   - Deprecate `compose::Pipeline` and simplify `ConditionalPipeline` over time.
4. **Docs/API refresh**
   - Update public docs to use `ops::*` and `pipeline::Pipeline` consistently.

## Outcome so far

- One registry drives name‑based lookups.
- One simple pipeline executes all ops with full interoperability.
- Usability improves by replacing trait‑object boilerplate with direct `ops::*` calls.
- Reduced duplication between demo and library logic.
