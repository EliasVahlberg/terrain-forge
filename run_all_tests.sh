#!/bin/bash

# TerrainForge - Run All Test Configurations

echo "TerrainForge - Running All Test Configurations"
echo "=============================================="

# Create configs directory if it doesn't exist
mkdir -p configs

# Create basic test configurations
cat > configs/bsp_basic.json << 'EOF'
{
  "name": "BSP Basic Test",
  "seed": 1001,
  "width": 100,
  "height": 60,
  "algorithm": "bsp",
  "parameters": {},
  "constraints": ["connectivity"],
  "output_formats": ["html", "png"]
}
EOF

cat > configs/cellular_basic.json << 'EOF'
{
  "name": "Cellular Automata Basic",
  "seed": 2001,
  "width": 100,
  "height": 60,
  "algorithm": "cellular_automata",
  "parameters": {},
  "constraints": ["connectivity"],
  "output_formats": ["html", "png"]
}
EOF

cat > configs/bsp_large.json << 'EOF'
{
  "name": "BSP Large Scale",
  "seed": 1002,
  "width": 200,
  "height": 120,
  "algorithm": "bsp",
  "parameters": {},
  "constraints": ["connectivity"],
  "output_formats": ["html", "png"]
}
EOF

cat > configs/cellular_dense.json << 'EOF'
{
  "name": "Cellular Dense Configuration",
  "seed": 2002,
  "width": 150,
  "height": 100,
  "algorithm": "cellular_automata",
  "parameters": {},
  "constraints": ["connectivity"],
  "output_formats": ["html", "png"]
}
EOF

# List of all test configurations
CONFIGS=(
    "bsp_basic.json"
    "cellular_basic.json"
    "bsp_large.json"
    "cellular_dense.json"
)

# Run each configuration
TOTAL=${#CONFIGS[@]}
SUCCESS=0
FAILED=0

for i in "${!CONFIGS[@]}"; do
    CONFIG="${CONFIGS[$i]}"
    echo ""
    echo "[$((i+1))/$TOTAL] Running: $CONFIG"
    echo "----------------------------------------"
    
    if cargo run --bin tilegen-test-tool -- --config "configs/$CONFIG"; then
        echo "✓ SUCCESS: $CONFIG"
        ((SUCCESS++))
    else
        echo "✗ FAILED: $CONFIG"
        ((FAILED++))
    fi
done

echo ""
echo "=============================================="
echo "Test Summary:"
echo "  Total tests: $TOTAL"
echo "  Successful: $SUCCESS"
echo "  Failed: $FAILED"
echo ""
echo "Results available in test_results/ directory"
echo "Open the HTML files in a browser to view reports"
