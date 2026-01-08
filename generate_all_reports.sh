#!/bin/bash

# Generate All TerrainForge Test Reports
# Uses the original Saltglass Steppe tilegen-test-tool

echo "🚀 TerrainForge - Generating All Test Reports"
echo "=============================================="

cd /home/elias/Documents/my_repos/saltglass-steppe

# List of all report configurations
CONFIGS=(
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/single_algo.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/layered.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/pipeline.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/maze_patterns.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/seed_study.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/gsb_tests.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/constraint_tests.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/hybrid_algorithms.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/performance_tests.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/all_algorithms.json"
    "/home/elias/Documents/my_repos/terrain-forge/report_configs/map_types.json"
)

# Run each configuration
TOTAL=${#CONFIGS[@]}
SUCCESS=0
FAILED=0

for i in "${!CONFIGS[@]}"; do
    CONFIG="${CONFIGS[$i]}"
    CONFIG_NAME=$(basename "$CONFIG" .json)
    
    echo ""
    echo "[$((i+1))/$TOTAL] Generating: $CONFIG_NAME"
    echo "----------------------------------------"
    
    if cargo run --bin tilegen-test-tool -- --config "$CONFIG" 2>/dev/null; then
        echo "✅ SUCCESS: $CONFIG_NAME"
        ((SUCCESS++))
    else
        echo "❌ FAILED: $CONFIG_NAME"
        ((FAILED++))
    fi
done

echo ""
echo "=============================================="
echo "📊 Report Generation Summary:"
echo "  Total configs: $TOTAL"
echo "  Successful: $SUCCESS"
echo "  Failed: $FAILED"
echo ""
echo "📁 Reports generated in:"
echo "  /home/elias/Documents/my_repos/saltglass-steppe/test_reports/"
echo ""
echo "🌐 Open the HTML files to view the reports!"

# Copy reports to TerrainForge directory
echo ""
echo "📋 Copying reports to TerrainForge directory..."
cp -r /home/elias/Documents/my_repos/saltglass-steppe/test_reports/* /home/elias/Documents/my_repos/terrain-forge/test_reports/ 2>/dev/null || true
echo "✅ Reports copied to /home/elias/Documents/my_repos/terrain-forge/test_reports/"
