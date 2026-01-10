# TerrainForge Demo Framework Proposal

## Overview

A standalone CLI tool for visualizing, testing, and comparing procedural generation approaches. Lives in a separate `demo/` directory, uses TerrainForge as a dependency (not part of the library).

## Goals

1. **Visual indication** - Quick PNG output of any algorithm/composition
2. **Game testing** - Test configurations for Saltglass Steppe integration
3. **Compare approaches** - Save/load configs, generate comparison grids

## Design Principles

- **Detached**: Separate crate in `demo/`, depends on `terrain-forge` like any user would
- **Mimics end-user**: If it's hard to use here, it's hard for users
- **Minimal**: Single binary, simple JSON configs, basic PNG output

## Structure

```
terrain-forge/
├── src/           # Library (unchanged)
├── demo/          # Demo framework (separate crate)
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── configs/   # Saved configurations
└── Cargo.toml     # Workspace
```

## Usage

### Quick Generation
```bash
cd demo
cargo run -- gen bsp                     # Generate with defaults
cargo run -- gen cellular -s 42          # Custom seed
cargo run -- gen maze -o maze.png        # Custom output
```

### Compositions
```bash
cargo run -- gen "bsp + cellular"        # Pipeline
cargo run -- gen "bsp | drunkard"        # Union blend
```

### Save/Load Configs
```bash
cargo run -- save dungeon_v1             # Save current to configs/dungeon_v1.json
cargo run -- load dungeon_v1             # Load and generate
cargo run -- list                        # List saved configs
```

### Compare
```bash
cargo run -- compare bsp cellular maze   # Side-by-side grid
cargo run -- compare --configs a b c     # Compare saved configs
```

## Config Format

```json
{
  "name": "cave_system_v2",
  "width": 80,
  "height": 60,
  "seed": 12345,
  "generation": {
    "type": "pipeline",
    "steps": ["cellular", "erode", "connect"]
  }
}
```

Or simple single algorithm:
```json
{
  "name": "basic_bsp",
  "width": 80,
  "height": 60,
  "seed": null,
  "generation": "bsp"
}
```

## Output

### Single Generation
- `output.png` - Grayscale map (floor=light, wall=dark)
- Terminal: seed used, floor %, connectivity score, generation time

### Comparison Grid
- `compare.png` - Tiled grid with labels
- Terminal: metrics table for each

## Implementation Sketch

```rust
// demo/src/main.rs
use terrain_forge::{Grid, Tile, algorithms, constraints};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Gen {
        algo: String,
        #[arg(short, long)]
        seed: Option<u64>,
        #[arg(short, long, default_value = "output.png")]
        output: String,
    },
    Save { name: String },
    Load { name: String },
    List,
    Compare { items: Vec<String> },
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Gen { algo, seed, output } => {
            let seed = seed.unwrap_or_else(random_seed);
            let mut grid = Grid::new(80, 60);
            
            // Parse algo string (handles "bsp + cellular" etc)
            generate(&mut grid, &algo, seed);
            
            save_png(&grid, &output);
            print_metrics(&grid, seed);
        }
        // ... other commands
    }
}
```

## Why This Approach

1. **Separate crate** = No library bloat, clear dependency direction
2. **CLI-first** = Fast iteration, scriptable, no GUI complexity
3. **JSON configs** = Human-readable, version-controllable, shareable
4. **Composition strings** = Quick experimentation without editing files
5. **Comparison output** = Visual A/B testing in one image

## What It Tests

- Algorithm registry API (`algorithms::get`)
- Grid creation and manipulation
- Constraint validation
- Composition (pipeline, blending)
- Real-world usage patterns

## Not Included (Intentionally)

- GUI/TUI - Adds complexity, PNG output is sufficient
- Live preview - Out of scope, use external image viewer
- Algorithm parameters - Use saved configs for complex setups
- Multiple cell types - Focus on `Tile` for simplicity

## Next Steps

1. Create `demo/` crate with workspace setup
2. Implement `gen` command with basic algo support
3. Add composition string parsing
4. Implement save/load
5. Add compare command
