#!/bin/bash
set -euo pipefail

# build-and-setup.sh - Build ODBC driver and configure ODBC settings
# Usage: ./scripts/build-and-setup.sh [release|debug]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_TYPE="${1:-debug}"

echo "Building ODBC driver (${BUILD_TYPE} mode)..."

cd "$PROJECT_ROOT"

# Build the driver
if [[ "$BUILD_TYPE" == "release" ]]; then
    cargo build --release
    DRIVER_PATH="$PROJECT_ROOT/target/release/libodbc_driver_rs.so"
else
    cargo build
    DRIVER_PATH="$PROJECT_ROOT/target/debug/libodbc_driver_rs.so"
fi

if [[ ! -f "$DRIVER_PATH" ]]; then
    echo "Error: Driver library not found at $DRIVER_PATH"
    exit 1
fi

# Write test_data/odbcinst.ini with the absolute path to the built driver.
# This file is gitignored because the path is machine-specific.
# ODBCSYSINI in .cargo/config.toml points cargo test at test_data/ so that
# unixODBC finds this file instead of ~/.odbcinst.ini.
cat > "$PROJECT_ROOT/test_data/odbcinst.ini" << EOF
[odbcrs_sqlite]
Driver = $DRIVER_PATH
Description = Experimental Rust SQLite ODBC Driver
Threading = 2
EOF

echo "Driver registered in test_data/odbcinst.ini ($DRIVER_PATH)"
echo "Setup complete. Run 'cargo test' to verify."
