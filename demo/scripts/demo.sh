#!/bin/bash

# TerrainForge Demo Runner
# Provides easy access to all demo scripts

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEMO_DIR="$(dirname "$SCRIPT_DIR")"

cd "$DEMO_DIR"

show_help() {
    echo "TerrainForge Demo Runner"
    echo
    echo "Usage: $0 <command>"
    echo
    echo "Commands:"
    echo "  semantic     - Run comprehensive semantic analysis demo"
    echo "  png          - Generate PNG visualizations"
    echo "  viz          - Advanced visualization examples"
    echo "  simple       - Basic visualization tutorial"
    echo "  test         - Run algorithm validation tests"
    echo "  all          - Run all demos"
    echo "  clean        - Clean output directories"
    echo
    echo "Examples:"
    echo "  $0 semantic"
    echo "  $0 png"
    echo "  $0 all"
}

case "${1:-help}" in
    semantic)
        echo "Running semantic analysis demo..."
        ./scripts/run_semantic_demo.sh
        ;;
    png)
        echo "Running PNG visualization demo..."
        ./scripts/run_png_demo.sh
        ;;
    viz)
        echo "Running advanced visualization demo..."
        ./scripts/run_visualization_demo.sh
        ;;
    simple)
        echo "Running simple visualization demo..."
        ./scripts/simple_viz_demo.sh
        ;;
    test)
        echo "Running algorithm tests..."
        ./scripts/run_tests.sh
        ;;
    all)
        echo "Running all demos..."
        ./scripts/run_semantic_demo.sh
        echo
        ./scripts/run_png_demo.sh
        echo
        ./scripts/run_tests.sh
        ;;
    clean)
        echo "Cleaning output directories..."
        rm -rf output/semantic/* output/png_visualizations/* output/algorithms/* output/validation/*
        echo "Output directories cleaned."
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo "Unknown command: $1"
        echo
        show_help
        exit 1
        ;;
esac
