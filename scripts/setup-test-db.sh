#!/bin/bash
set -euo pipefail

# setup-test-db.sh - Creates standardized test SQLite databases for ODBC driver testing
# Usage: ./scripts/setup-test-db.sh [database_name]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_DATA_DIR="$PROJECT_ROOT/test_data"
SCHEMA_FILE="$TEST_DATA_DIR/schema.sql"

# Default database name
DB_NAME="${1:-test_odbc.sqlite}"
DB_PATH="$PROJECT_ROOT/$DB_NAME"

echo "🗄️  Setting up test database: $DB_NAME"
echo "📁 Location: $DB_PATH"
echo "📋 Schema: $SCHEMA_FILE"

# Check if schema file exists
if [[ ! -f "$SCHEMA_FILE" ]]; then
    echo "❌ Error: Schema file not found at $SCHEMA_FILE"
    echo "   Make sure you're running this from the project root or scripts directory"
    exit 1
fi

# Remove existing database if present
if [[ -f "$DB_PATH" ]]; then
    echo "🗑️  Removing existing database: $DB_PATH"
    rm "$DB_PATH"
fi

# Create new database with schema
echo "🏗️  Creating database and applying schema..."
sqlite3 "$DB_PATH" < "$SCHEMA_FILE"

# Verify database creation
if [[ -f "$DB_PATH" ]]; then
    echo "✅ Database created successfully!"
    
    # Show database info
    echo ""
    echo "📊 Database summary:"
    sqlite3 "$DB_PATH" "
    SELECT 'Tables: ' || COUNT(*) as info FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'
    UNION ALL
    SELECT 'Views: ' || COUNT(*) FROM sqlite_master WHERE type='view'
    UNION ALL
    SELECT 'Indexes: ' || COUNT(*) FROM sqlite_master WHERE type='index' AND name NOT LIKE 'sqlite_%';
    "
    
    echo ""
    echo "📋 Tables created:"
    sqlite3 "$DB_PATH" "SELECT '  - ' || name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name;"
    
    echo ""
    echo "👥 Sample data:"
    echo "  - Users: $(sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM users;") records"
    echo "  - Products: $(sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM products;") records" 
    echo "  - Orders: $(sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM orders;") records"
    echo "  - Order Items: $(sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM order_items;") records"
    
else
    echo "❌ Failed to create database!"
    exit 1
fi

echo ""
echo "🎯 Next steps:"
echo "  1. Update ~/.odbc.ini with: Database = $DB_PATH"
echo "  2. Test with: isql -3 test_connection -v"
echo "  3. Run: SELECT * FROM users; -- to verify data"
echo ""
echo "🔧 For development testing:"
echo "  cargo test  # Run unit tests"
echo "  ./scripts/run-tests.sh  # Run integration tests (when available)"

# Create a simple test query file for manual verification
TEST_QUERIES_FILE="$TEST_DATA_DIR/sample_queries.sql"
cat > "$TEST_QUERIES_FILE" << 'EOF'
-- Sample queries for manual testing with isql
-- Run these after connecting: isql -3 test_connection -v

-- Basic table listing (should work with current driver)
help;

-- Simple data queries (may not work yet - depends on driver implementation)
SELECT COUNT(*) FROM users;
SELECT username, email FROM users;
SELECT name, price FROM products WHERE price > 50;

-- Join query for advanced testing
SELECT u.username, COUNT(o.order_id) as order_count 
FROM users u 
LEFT JOIN orders o ON u.user_id = o.user_id 
GROUP BY u.username;

-- View query
SELECT * FROM user_order_summary;
EOF

echo "💡 Sample queries saved to: $TEST_QUERIES_FILE"
echo "   Use these for manual testing with isql"