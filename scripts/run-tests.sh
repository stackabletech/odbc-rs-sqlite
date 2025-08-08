#!/bin/bash
set -euo pipefail

# run-tests.sh - Run comprehensive test suite for ODBC driver
# Usage: ./scripts/run-tests.sh [unit|integration|all]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_TYPE="${1:-all}"

cd "$PROJECT_ROOT"

echo "🧪 Running ODBC driver test suite..."
echo "📁 Project root: $PROJECT_ROOT"
echo "🎯 Test type: $TEST_TYPE"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored status
print_status() {
    local status=$1
    local message=$2
    case $status in
        "PASS") echo -e "${GREEN}✅ PASS${NC}: $message" ;;
        "FAIL") echo -e "${RED}❌ FAIL${NC}: $message" ;;
        "WARN") echo -e "${YELLOW}⚠️  WARN${NC}: $message" ;;
        "INFO") echo -e "${BLUE}ℹ️  INFO${NC}: $message" ;;
    esac
}

# Function to run unit tests
run_unit_tests() {
    echo ""
    echo "🔬 Running unit tests..."

    if cargo test --lib; then
        print_status "PASS" "All unit tests passed"
        return 0
    else
        print_status "FAIL" "Some unit tests failed"
        return 1
    fi
}

# Function to run integration tests
run_integration_tests() {
    echo ""
    echo "🔗 Running integration tests..."

    # Check if driver is built
    local driver_path="target/debug/libodbc_driver_rs.so"
    if [[ ! -f "$driver_path" ]]; then
        print_status "WARN" "Driver not built. Building first..."
        cargo build
        if [[ ! -f "$driver_path" ]]; then
            print_status "FAIL" "Failed to build driver"
            return 1
        fi
    fi

    # Check if test database exists
    if [[ ! -f "test_odbc.sqlite" ]]; then
        print_status "WARN" "Test database not found. Creating it..."
        ./scripts/setup-test-db.sh
    fi

    # Run integration tests (if any exist)
    if [[ -d "tests" ]] && ls tests/*.rs &> /dev/null; then
        if cargo test --test '*'; then
            print_status "PASS" "Integration tests passed"
        else
            print_status "FAIL" "Integration tests failed"
            return 1
        fi
    else
        print_status "INFO" "No integration tests found (tests/*.rs)"
    fi

    # Manual ODBC connection test (if isql is available)
    if command -v isql &> /dev/null; then
        echo ""
        echo "🔌 Testing ODBC connection..."

        # Update ODBC configuration
        ./scripts/build-and-setup.sh debug >/dev/null 2>&1

        # Test basic connection
        if timeout 10 isql -3 test_connection -b -v <<< "help;" >/dev/null 2>&1; then
            print_status "PASS" "ODBC connection successful"
        else
            print_status "FAIL" "ODBC connection failed"
            print_status "INFO" "Manual test: isql -3 test_connection -v"
            return 1
        fi
    else
        print_status "WARN" "isql not available for ODBC testing"
    fi

    return 0
}

# Function to run all tests
run_all_tests() {
    local unit_result=0
    local integration_result=0

    run_unit_tests || unit_result=1
    run_integration_tests || integration_result=1

    echo ""
    echo "📊 Test Summary:"
    if [[ $unit_result -eq 0 ]]; then
        print_status "PASS" "Unit tests"
    else
        print_status "FAIL" "Unit tests"
    fi

    if [[ $integration_result -eq 0 ]]; then
        print_status "PASS" "Integration tests"
    else
        print_status "FAIL" "Integration tests"
    fi

    if [[ $unit_result -eq 0 && $integration_result -eq 0 ]]; then
        echo ""
        print_status "PASS" "All tests completed successfully!"
        return 0
    else
        echo ""
        print_status "FAIL" "Some tests failed"
        return 1
    fi
}

# Main execution
case $TEST_TYPE in
    "unit")
        run_unit_tests
        ;;
    "integration")
        run_integration_tests
        ;;
    "all")
        run_jall_tests
        ;;
    *)
        echo "❌ Error: Unknown test type '$TEST_TYPE'"
        echo "Usage: $0 [unit|integration|all]"
        exit 1
        ;;
esac

exit $?
