# Report Generator System (Archived)

## Purpose

The report-generator binary was a visual testing and documentation tool for TerrainForge algorithms. It generated HTML reports with PNG visualizations of procedurally generated maps.

## Usage

```bash
cargo run --bin report-generator -- --config my_config.json
```

### Config Format

```json
{
  "title": "Algorithm Comparison",
  "output_dir": "./reports/comparison",
  "entries": [
    {
      "name": "BSP Dungeon",
      "seed": 12345,
      "width": 80,
      "height": 60,
      "algorithm": "bsp"
    }
  ]
}
```

### Output

- PNG image per entry (grayscale: floor=light, wall=dark)
- `report.html` with grid layout showing all entries
- Metrics per entry: generation time, connectivity score, floor count

## Goal

Enable visual comparison of algorithms across different seeds and parameters for:
- Algorithm development and debugging
- Documentation screenshots
- Performance benchmarking
- Quality validation

## Removal Reason

The report system was designed for the old dual-system architecture and relied on external config files that were removed during cleanup. The core functionality (algorithm testing, PNG output) is covered by `tilegen-test-tool`. Integration tests now validate algorithm behavior programmatically.
