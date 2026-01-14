# Composite TerrainForge Demo Use Cases

Purpose: reference scenarios for new demo configurations that chain multiple algorithms to highlight TerrainForge’s composition strengths.

## Overworld + POI Focus

- **“Saltglass Overworld”**: `bsp > voronoi > glass_seam` to produce macro regions then ensure connectivity; good for overworld travel maps with distinct biomes and guaranteed road loops.
- **“Trade Route Survey”**: `rooms | voronoi` to carve hubs then partition hinterlands; use semantic markers to flag choke points and place caravans/outposts.

## Dungeon Progression

- **“Layered Stronghold”**: `bsp > rooms > cellular` to mix ordered entry halls with organic lower levels; ideal for multi-stage raids or quests that escalate from structure to chaos.
- **“Vault With Fissures”**: `rooms > glass_seam > drunkard` for structured vaults split by fractures; showcases the glass-seam connector repairing deliberate breaches.

## Cave Ecology

- **“Breathing Caverns”**: `cellular > percolation` to create dense caverns then open air pockets; attach semantic “habitat” tags for fauna placement.
- **“Stratified Caves”**: `cellular > voronoi` to assign strata/biomes inside a single cave; demonstrates weighted region labeling.

## Set-Piece Encounters

- **“Siege Approaches”**: `agent > bsp` to carve approach tunnels that terminate in ordered courtyards; great for tactical encounter layouts.
- **“Ritual Nexus”**: `wfc > glass_seam` for patterned chambers that still ensure pathing; supports puzzle/key-lock scenarios.

## Large-Format Showcases

- **“Epic Canyon Run”**: `drunkard > glass_seam` at 240x180 with connectivity viz to highlight long, winding paths fixed by seam bridging.
- **“Hybrid Megadungeon”**: `bsp > rooms | cellular > voronoi` to contrast room clusters with organic sub-biomes; export masks/regions for slides.

## Analysis & QA

- **“Constraint Stress Test”**: `rooms > percolation` plus connectivity and density validation to benchmark constraint scores across seeds.
- **“Semantic Storyboard”**: any pipeline + `--semantic --regions` to generate colored layers for documentation or art direction boards.
