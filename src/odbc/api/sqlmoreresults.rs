use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLMoreResults determines whether more results are available on a statement containing
/// SELECT, UPDATE, INSERT, or DELETE statements and, if so, initializes processing for those results.
///
/// For SQLite, multiple result sets are not supported in the traditional sense (unlike SQL Server
/// stored procedures or MySQL batch statements). SQLite executes one statement at a time, so this
/// function will typically return SQL_NO_DATA to indicate no additional result sets are available.
///
/// Note: This implementation assumes single result set per statement execution, which is
/// appropriate for SQLite's architecture.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLMoreResults(statement_handle: *mut c_void) -> SqlReturn {
    info!("Checking for additional result sets");

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

            // SQLite doesn't support multiple result sets from a single statement execution
            // Unlike SQL Server stored procedures or MySQL batch operations, SQLite processes
            // one statement at a time. Therefore, after processing the initial result set,
            // there are no additional result sets available.

            info!("No additional result sets available (SQLite limitation)");
            SqlReturn::NO_DATA
        }
        None => {
            error!("No prepared statement found");
            SqlReturn::ERROR
        }
    }
}
