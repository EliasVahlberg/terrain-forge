#!/bin/bash

# Manifest-based demo runner
# Proxies to the Rust CLI demo subcommand that reads demo/manifest.toml

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEMO_DIR="$(dirname "$SCRIPT_DIR")"
cd "$DEMO_DIR"

if [[ "$1" == "" || "$1" == "help" || "$1" == "--help" || "$1" == "-h" ]]; then
    echo "Usage: ./scripts/demo.sh <demo-id> [--run <name>] [--manifest <path>] [--list]"
    echo "       ./scripts/demo.sh all"
    echo "Examples:"
    echo "  ./scripts/demo.sh --list"
    echo "  ./scripts/demo.sh semantic"
    echo "  ./scripts/demo.sh png --run cellular_views"
    echo "  ./scripts/demo.sh all"
    exit 0
fi

if [[ "$1" == "all" ]]; then
    shift
    cargo run --bin terrain-forge-demo -- demo --all "$@"
else
    cargo run --bin terrain-forge-demo -- demo "$@"
fi
