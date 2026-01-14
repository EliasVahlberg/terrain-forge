# Demo Use Cases for Composite Algorithms

Reference scenarios that justify why each composite demo matters before we add more runnable examples. Pair these with visual outputs (`--regions`, `--masks`, `--connectivity`, `--semantic`) to illustrate the value.

## Overworld & Travel Maps
- **Saltglass Overworld** — `bsp > voronoi > glass_seam`; biomes with guaranteed looped roads for open-world fast travel and quest routing.
- **Trade Route Survey** — `rooms | voronoi`; hub towns carved first, hinterlands partitioned second; highlight choke points for caravans/outposts.

## Dungeon Progression
- **Layered Stronghold** — `bsp > rooms > cellular`; formal entry + messy lower depths; good for multi-stage raids with escalating risk.
- **Vault With Fissures** — `rooms > glass_seam > drunkard`; orderly vaults split by fractures, then reconnected; demonstrates repair + sabotage narratives.

## Cave Ecology & Biomes
- **Breathing Caverns** — `cellular > percolation`; dense caves opened into air pockets; use semantic markers for fauna nests or hazards.
- **Stratified Caves** — `cellular > voronoi`; assign strata/biomes inside one cave; perfect for resource distribution or lighting gradients.

## Set-Piece Encounters
- **Siege Approaches** — `agent > bsp`; winding approach tunnels terminating in ordered courtyards; supports tactical defense scenarios.
- **Ritual Nexus** — `wfc > glass_seam`; patterned chambers with enforced connectivity; good for puzzle/key-lock layouts.

## Large-Format Showcases
- **Epic Canyon Run** — `drunkard > glass_seam` at 240x180 with connectivity viz; demonstrates long paths stabilized by seam bridging.
- **Hybrid Megadungeon** — `bsp > rooms | cellular > voronoi`; contrasts room clusters with organic sub-biomes; great for slide decks or posters.

## Analysis & QA
- **Constraint Stress Test** — `rooms > percolation` plus connectivity/density validation; benchmarks pipeline robustness across seeds.
- **Semantic Storyboard** — any pipeline + `--semantic --regions`; produces colored layers for design documentation or art direction boards.
