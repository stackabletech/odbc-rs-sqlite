use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLRowCount returns the number of rows affected by an UPDATE, INSERT, or DELETE statement.
///
/// For SELECT statements and other statements that don't modify rows, this function
/// should return -1 to indicate that the row count is not available or not applicable.
///
/// Note: According to ODBC spec, SQLRowCount only applies to statements that modify data.
/// For SELECT statements, use SQLNumResultCols to get column count or iterate through
/// SQLFetch calls to count rows.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLRowCount(
    statement_handle: *mut c_void,
    row_count_ptr: *mut isize,
) -> SqlReturn {
    info!("Getting row count");

    // Validate row_count_ptr is not null
    if row_count_ptr.is_null() {
        error!("row_count_ptr is null");
        return SqlReturn::ERROR;
    }

    // Get the statement handle
    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    // Check if we have a prepared statement
    match &statement_handle.statement {
        Some(_) => {
            debug!("Found prepared statement");

            // Try to get the number of rows affected
            // Note: SQLite's changes() function returns the number of rows affected by the most recent
            // INSERT, UPDATE, or DELETE statement. For SELECT statements, it's not applicable.
            let changes = statement_handle.sqlite_connection.changes();

            debug!("SQLite changes() returned: {}", changes);

            // According to ODBC spec:
            // - For UPDATE, INSERT, DELETE: return actual row count
            // - For SELECT and other statements: return -1 (not available)
            // Since we can't easily determine the statement type here, we'll use SQLite's changes()
            // but note that it may return 0 for SELECT statements

            unsafe {
                if changes > 0 {
                    *row_count_ptr = changes as isize;
                    debug!("Returning row count: {}", changes);
                } else {
                    // For statements that don't modify data (like SELECT), return -1
                    // This indicates that the row count is not available or not applicable
                    *row_count_ptr = -1;
                    debug!("Returning -1 (row count not applicable for this statement type)");
                }
            }

            SqlReturn::SUCCESS
        }
        None => {
            error!("SQLRowCount ERROR: No prepared statement found");
            unsafe {
                *row_count_ptr = -1;
            }
            SqlReturn::ERROR
        }
    }
}
