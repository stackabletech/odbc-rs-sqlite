use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLExecute executes a prepared statement.
///
/// This function executes a statement that was prepared with SQLPrepareW.
/// The statement is executed with the current parameter values.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLExecute(statement_handle: *mut c_void) -> SqlReturn {
    info!("SQLExecute INFO");

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
    if statement_handle.statement.is_none() {
        error!("No prepared statement found");
        return SqlReturn::ERROR;
    }

    debug!("Executing prepared statement");

    // Execute the prepared statement
    match statement_handle.statement {
        Some(ref mut stmt) => match stmt.query([]) {
            Ok(rows) => {
                debug!("Statement executed successfully");
                statement_handle.rows = Some(rows);
                SqlReturn::SUCCESS
            }
            Err(err) => {
                error!("Failed to execute statement: {}", err);
                SqlReturn::ERROR
            }
        },
        None => {
            // This should not happen due to the check above, but handle it anyway
            error!("Prepared statement is None");
            SqlReturn::ERROR
        }
    }
}
