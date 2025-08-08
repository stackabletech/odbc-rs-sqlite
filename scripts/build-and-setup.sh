#!/bin/bash
set -euo pipefail

# build-and-setup.sh - Build ODBC driver and configure ODBC settings
# Usage: ./scripts/build-and-setup.sh [release|debug]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_TYPE="${1:-debug}"

echo "🔨 Building ODBC driver (${BUILD_TYPE} mode)..."

cd "$PROJECT_ROOT"

# Build the driver
if [[ "$BUILD_TYPE" == "release" ]]; then
    echo "🚀 Building release version..."
    cargo build --release
    DRIVER_PATH="$PROJECT_ROOT/target/release/libodbc_driver_rs.so"
else
    echo "🐛 Building debug version..."
    cargo build
    DRIVER_PATH="$PROJECT_ROOT/target/debug/libodbc_driver_rs.so"
fi

# Verify the driver was built
if [[ ! -f "$DRIVER_PATH" ]]; then
    echo "❌ Error: Driver library not found at $DRIVER_PATH"
    echo "   Build may have failed. Check the output above."
    exit 1
fi

echo "✅ Driver built successfully: $DRIVER_PATH"

# ODBC configuration
ODBCINST_INI="$HOME/.odbcinst.ini"
ODBC_INI="$HOME/.odbc.ini"
TEST_DB_PATH="$PROJECT_ROOT/test_odbc.sqlite"

echo ""
echo "🔧 Configuring ODBC settings..."

# Create or update .odbcinst.ini (driver configuration)
echo "📝 Updating driver configuration: $ODBCINST_INI"

# Create backup if file exists and no backup exists yet
if [[ -f "$ODBCINST_INI" ]] && [[ ! -f "${ODBCINST_INI}.backup" ]]; then
    echo "📋 Creating one-time backup: ${ODBCINST_INI}.backup"
    cp "$ODBCINST_INI" "${ODBCINST_INI}.backup"
fi

# Remove existing odbcrs_sqlite section and add new one
if [[ -f "$ODBCINST_INI" ]]; then
    # Remove existing section if present
    sed -i '/^\[odbcrs_sqlite\]/,/^\[/{ /^\[odbcrs_sqlite\]/d; /^\[/!d; }' "$ODBCINST_INI" 2>/dev/null || true
fi

# Add our driver configuration
cat >> "$ODBCINST_INI" << EOF

[odbcrs_sqlite]
Driver = $DRIVER_PATH
Description = Experimental Rust SQLite ODBC Driver
Threading = 2
UsageCount = 1
EOF

echo "✅ Driver registered in $ODBCINST_INI"

# Create or update .odbc.ini (data source configuration)
echo "📝 Updating data source configuration: $ODBC_INI"

# Create backup if file exists and no backup exists yet
if [[ -f "$ODBC_INI" ]] && [[ ! -f "${ODBC_INI}.backup" ]]; then
    echo "📋 Creating one-time backup: ${ODBC_INI}.backup"
    cp "$ODBC_INI" "${ODBC_INI}.backup"
fi

# Remove existing test_connection section and add new one
if [[ -f "$ODBC_INI" ]]; then
    sed -i '/^\[test_connection\]/,/^\[/{ /^\[test_connection\]/d; /^\[/!d; }' "$ODBC_INI" 2>/dev/null || true
fi

# Add our data source configuration
cat >> "$ODBC_INI" << EOF

[test_connection]
Driver = odbcrs_sqlite
Database = $TEST_DB_PATH
Description = Test connection for ODBC Rust driver development
EOF

echo "✅ Data source configured in $ODBC_INI"

# Verify test database exists
if [[ ! -f "$TEST_DB_PATH" ]]; then
    echo "⚠️  Test database not found. Creating it..."
    ./scripts/setup-test-db.sh
fi

# Verify ODBC installation
echo ""
echo "🔍 Verifying ODBC installation..."

# Check if odbcinst is available
if command -v odbcinst &> /dev/null; then
    echo "📋 Installed drivers:"
    odbcinst -q -d | grep -E "(odbcrs_sqlite|SQLite)" || echo "  (No SQLite drivers found)"
    
    echo ""
    echo "📋 Configured data sources:"
    odbcinst -q -s | grep -E "(test_connection)" || echo "  (test_connection not found)"
else
    echo "⚠️  odbcinst command not found. Install unixODBC development tools:"
    echo "     Ubuntu/Debian: sudo apt-get install unixodbc-dev"
    echo "     Arch Linux: sudo pacman -S unixodbc"
fi

# Check if isql is available for testing
if command -v isql &> /dev/null; then
    echo ""
    echo "🎯 Ready for testing! Try these commands:"
    echo "   isql -3 test_connection -v    # Test connection"
    echo "   help;                         # List tables (should work)"
    echo "   SELECT * FROM users;          # Query data (may not work yet)"
else
    echo ""
    echo "⚠️  isql command not found. Install unixODBC:"
    echo "     Ubuntu/Debian: sudo apt-get install unixodbc"
    echo "     Arch Linux: sudo pacman -S unixodbc"
fi

echo ""
echo "📁 Configuration summary:"
echo "   Driver: $DRIVER_PATH"
echo "   Database: $TEST_DB_PATH"  
echo "   Driver config: $ODBCINST_INI"
echo "   Data source config: $ODBC_INI"

echo ""
echo "🚀 Build and setup complete!"
echo "   Run 'cargo test' to verify unit tests still pass"
echo "   Use './scripts/run-tests.sh' for integration testing (when available)"