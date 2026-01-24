# Tests Overview

This directory contains integration tests for core library behavior. Unit tests live alongside
modules under `src/`. The `demo/` workspace is also used as a validation step, but it is primarily
for showcasing workflows and example outputs.

## Structure

- `tests/algorithms.rs`
  - Determinism, floor production, and border preservation across registered algorithms.
  - Basic connectivity check for `glass_seam`.
- `tests/phase1_integration.rs`
  - Semantic constraints and requirement validation.
  - Basic vertical connectivity checks.
- `tests/phase2_integration.rs`
  - Pipeline condition evaluation, context bookkeeping, template plumbing.
  - Conditional pipeline execution (algorithm + parameter setting).
- `tests/phase3_integration.rs`
  - Spatial analysis utilities (distance fields, Dijkstra, flow field, morphology).
- `tests/phase4_integration.rs`
  - WFC behavior, Delaunay + graph analysis, prefab library/transformations.

## What These Tests Validate Well

- Determinism and basic output sanity for the registered algorithms.
- Core semantic requirement checks and metadata constraints.
- Spatial analysis primitives and morphology correctness on small grids.
- Prefab transformations and weighted selection behavior.

## Gaps / Additions Needed For Full Coverage

The current suite validates major subsystems but does not fully cover recent or high-risk paths:

1. **Unified ops facade**
   - Missing direct tests for `ops::generate`, `ops::effect`, and `ops::combine`, including
     parameter parsing (ranges, booleans, tiles, points).
   - Add: table-driven tests that call ops by name with params and verify expected grid changes.

2. **New pipeline step model**
   - `Pipeline` (step-based, name-driven) is not exercised in tests. Existing pipeline tests
     cover the older `ConditionalPipeline` and template helpers.
   - Add: tests for `Pipeline::add_algorithm`, `add_effect`, `add_combine_with_*`, `store_grid`,
     `add_if`, and `execute_seed` error handling.

3. **NoiseFill extensions**
   - No tests cover `scale`, `output_range`, `fill_range`, `octaves`, or the FBM path.
   - Add: deterministic fixtures that validate tile counts or expected ranges with known seeds.

4. **Effects additions**
   - `invert` and `resize` are not tested.
   - Add: small grid fixtures verifying exact tile swaps and correct padding/truncation.

5. **Blend mode Difference**
   - `BlendMode::Difference` behavior not tested.
   - Add: a layered test where a known mask removes floors from a base grid.

6. **Error paths and boundary behavior**
   - Missing tests for invalid algorithm/effect names, missing params (e.g., `clear_rect`),
     and empty/degenerate grids in pipeline/ops.
   - Add: negative tests that assert `OpError` messages and no panics.

7. **Cross-module integration**
   - There is no end-to-end test that combines algorithms, effects, semantic extraction, and
     constraints in a single pipeline run.
   - Add: a small "mini scenario" integration test that runs a pipeline and validates semantic
     requirements or connectivity.

## Notes On Demo Validation

The `demo/` workspace exercises real workflows and is useful for catching regressions in
configuration parsing and output rendering. It should be treated as a high-level smoke test,
not a substitute for deterministic, assertion-based tests in this folder.
