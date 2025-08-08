# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an experimental ODBC driver implementation written in Rust that connects to SQLite. It serves as a proof-of-concept to demonstrate the feasibility of writing ODBC drivers in Rust. The project is not intended as a production SQLite driver, but rather as a foundation for creating reusable ODBC driver patterns and boilerplate.

**Key characteristics:**
- Targets ODBC 3.8 (ODBC 2.x is NOT supported)
- Only implements Unicode (UTF-16) methods with `W` suffix
- Built as a `cdylib` (C dynamic library) for ODBC driver managers
- Experimental/hobby project status

## Development Commands

### Building
```bash
cargo build
```
This produces `target/debug/libodbc_driver_rs.so` (the ODBC driver library).

### Testing with unixODBC
The driver is tested using `isql` from unixODBC:

1. Configure `~/.odbcinst.ini`:
```ini
[odbcrs_sqlite]
Driver = <path to libodbc_driver_rs.so>
```

2. Configure `~/.odbc.ini`:
```ini
[test_connection]
Driver = odbcrs_sqlite
Database = <path to sqlite database>
```

3. Test connection:
```bash
isql -3 test_connection -v
```

Currently, only the `help` command works in `isql` (lists SQLite tables).

## Code Architecture

### Module Structure
```
src/
├── lib.rs              # Main library entry point, FFI linking configuration
├── connection.rs       # ConnectionClass definition (minimal)
└── odbc/               # ODBC implementation modules
    ├── api/            # Individual ODBC function implementations (~45 functions)
    ├── implementation/ # Core business logic modules
    ├── def/            # Type definitions and constants
    └── utils.rs        # Common utilities for handle management
```

### Core Architecture Patterns

**ODBC Function Organization:**
- Each ODBC function (SQLAllocHandle, SQLConnect, etc.) has its own module in `src/odbc/api/`
- Functions are exported with `#[no_mangle]` and `extern "C"` for FFI
- All functions include debug logging via `println!` macros

**Handle Management:**
- Environment, Connection, and Statement handles are managed in `implementation/alloc_handles.rs`
- Handles are wrapped/unwrapped using utilities in `utils.rs`
- Handle allocation follows ODBC's hierarchical model

**Implementation Strategy:**
- Many functions are stub implementations that log method names when called
- Core functionality focuses on connection establishment and basic table listing
- Uses rusqlite for SQLite database operations

**Dependencies:**
- `odbc-sys`: Provides ODBC type definitions and constants (forked version at `https://github.com/lfrancke/odbc-sys.git`, branch `stackable`)
- `rusqlite`: SQLite interface with bundled SQLite
- `widestring`: UTF-16 string handling for ODBC Unicode API
- `snafu`: Error handling
- `num_enum`: Enum conversions

## 🔧 Local odbc-sys Fork Setup

**CRITICAL:** We use a forked version of `odbc-sys` with custom changes. The fork is available locally:

### Location and Access
- **Local copy:** `deps/odbc-sys/` (cloned during development session)
- **Remote:** `https://github.com/lfrancke/odbc-sys.git` (branch: `stackable`)
- **Usage:** Referenced in `Cargo.toml` as git dependency

### Key Files to Reference
- `deps/odbc-sys/src/info_type.rs` - Complete `InfoType` enum and return type mappings
- `deps/odbc-sys/src/sqlreturn.rs` - SqlReturn codes and error handling
- `deps/odbc-sys/src/lib.rs` - Main exports and type definitions

### Why This Matters
1. **API Visibility:** Can see exact ODBC type definitions instead of working "blind"
2. **Type Safety:** Know which InfoTypes return `SqlUSmallInt` vs `SqlUInteger` vs `String`
3. **ODBC Compliance:** Verify implementations against correct type specifications
4. **Future Extensions:** Can add new InfoType variants or modify behavior if needed

### When to Update Fork
- **Adding new ODBC functionality** - May need new InfoType variants or constants
- **Fixing type mismatches** - If ODBC spec requires different return types
- **Bug fixes** - Issues discovered in type mappings or enum definitions

### How to Update Fork (if needed)
```bash
cd deps/odbc-sys
# Make changes to source files
git add -A
git commit -m "Description of changes"
git push origin stackable
# Changes will be picked up on next cargo build
```

### Integration Notes
- Fork is in `.gitignore` - not committed to main repository
- Any Claude session can clone locally: `git clone https://github.com/lfrancke/odbc-sys.git deps/odbc-sys && cd deps/odbc-sys && git checkout stackable`
- Changes to fork affect all development immediately via Cargo.toml git dependency

### Development Workflow

The recommended development approach is:
1. Use a SQL client (like `isql`) to trigger ODBC function calls
2. Monitor debug output to see which functions are called in sequence
3. Implement minimal functionality for the next required function
4. Iterate until desired functionality works

Example connection sequence:
1. SQLAllocHandle (Environment)
2. SQLSetEnvAttr (ODBC version)
3. SQLGetEnvAttr (version check)
4. SQLAllocHandle (Connection)  
5. SQLSetConnectAttr (Unicode handling)
6. SQLConnectW (actual connection)

### Key Implementation Files
- `src/odbc/api/sqlallochandle.rs` - Handle allocation logic
- `src/odbc/implementation/connect.rs` - Connection establishment
- `src/odbc/implementation/alloc_handles.rs` - Handle management
- `src/odbc/api/sqltables.rs` - Table listing functionality

## 🚀 Project Progress Management

**CRITICAL:** This is a multi-month project requiring careful progress tracking across many Claude Code sessions.

### 📋 Always Check plan.md First

**Before starting any work, ALWAYS:**
1. **Read `plan.md`** - Contains the complete improvement roadmap
2. **Check current phase and tasks** - Identify where we are in the plan
3. **Look for `[DONE]` markers** - See completed tasks
4. **Find `[IN PROGRESS]` markers** - Resume interrupted work
5. **Update progress markers** - Record your accomplishments

### 🎯 Progress Tracking Methodology

**When working on any task:**

#### Starting Work:
1. Mark task as `[IN PROGRESS - Session YYYY-MM-DD]` in plan.md
2. Create detailed sub-tasks if the main task is complex
3. Record your initial approach and any blockers discovered

#### During Work:
1. **Record intermediate results** in plan.md under the task
2. **Document any issues encountered** and their solutions
3. **Update code file references** when you modify or create files
4. **Note any architectural decisions** or pattern changes

#### Completing Work:
1. Mark task as `[DONE - Session YYYY-MM-DD]` in plan.md
2. **Record key outcomes** and files modified
3. **Note any follow-up tasks** that were discovered
4. **Update next steps** and dependencies

#### Example Progress Format:
```markdown
- [IN PROGRESS - 2024-01-15] **Create test database setup**
  - APPROACH: Building scripts/setup-test-db.sh and test_data/schema.sql
  - FILES: Created scripts/setup-test-db.sh, test_data/schema.sql  
  - BLOCKER: Need to decide on test database schema structure
  - NEXT: Complete schema.sql with users, products, orders tables
  
- [DONE - 2024-01-16] **Fix failing unit test** 
  - SOLUTION: Integer size mismatch in sqlgetinfo.rs:322 - changed assertion from 4 to 2
  - FILES: Modified src/odbc/api/sqlgetinfo.rs:322
  - OUTCOME: All 13/13 unit tests now pass
  - FOLLOW-UP: Need to investigate why size is 2 instead of expected 4
```

### 🔄 Session Continuity

**When resuming work in a new session:**

#### Session Startup Checklist:
1. **Read plan.md completely** - Understand current project state
2. **Run `cargo test`** - Verify current functionality still works
3. **Check for `[IN PROGRESS]` tasks** - Identify interrupted work
4. **Review recent `[DONE]` items** - Understand what was accomplished
5. **Run any test scripts** if they exist (scripts/run-tests.sh)

#### Context Recovery:
1. **Read code comments** in recently modified files
2. **Check git log** for recent changes and commit messages  
3. **Look for TODO comments** in code that indicate next steps
4. **Review any test failures** to understand current state

#### Before Making Changes:
1. **Update plan.md** with your session date and planned work
2. **Confirm current phase objectives** align with your planned work
3. **Identify dependencies** that might affect your work
4. **Document your approach** before implementing

### 📁 Key Files to Monitor

**Always check these files for project context:**
- `plan.md` - Master project roadmap and progress
- `CLAUDE.md` - This file, project documentation  
- `Cargo.toml` - Dependencies and project configuration
- `src/lib.rs` - Main entry point and FFI configuration
- `src/odbc/utils.rs` - Core handle management patterns
- Any files in `scripts/` directory - Build and test automation

### ⚠️ Critical Patterns to Maintain

**Code Patterns:**
- Handle management via `wrap_and_set()` and `get_from_wrapper()` in `utils.rs`
- Error handling with `snafu` crate, avoid `panic!` in FFI functions
- All FFI functions use `#[no_mangle]` and `extern "C"`
- UTF-16 string handling via `utf16_to_string()` and related functions

**Testing Patterns:**
- Direct FFI tests in individual modules (see `src/odbc/api/sqlallochandle.rs`)
- Integration tests should go in `tests/` directory
- Build automation scripts in `scripts/` directory
- Test databases in `test_data/` directory

**Progress Patterns:**
- Always update plan.md with progress markers
- Record architectural decisions and their rationale
- Document blockers and their solutions
- Note follow-up tasks discovered during implementation

### 🏗️ Current Project Status

**Phase:** Foundation & Safety (Phase 1)
**Priority:** Automated Testing Infrastructure (1.1) → Development Tooling (1.2) → Critical Safety Issues (1.3)

**Key Context:**
- SQLFreeHandle memory leak is intentionally deferred to Phase 2.2
- Focus on testing infrastructure before safety fixes
- Manual `isql` testing should be eliminated ASAP
- Multi-month timeline expected, plan for continuity

**Next Session Should:**
1. Check plan.md for latest progress
2. Start/continue Phase 1.1 tasks
3. Focus on automated testing foundation
4. Record all progress and discoveries in plan.md