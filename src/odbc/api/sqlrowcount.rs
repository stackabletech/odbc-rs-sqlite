use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLRowCount returns the number of rows affected by an UPDATE, INSERT, or DELETE statement.
///
/// Returns -1 for SELECT statements or when no statement is active.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLRowCount(
    statement_handle: *mut c_void,
    row_count_ptr: *mut isize,
) -> SqlReturn {
    info!("Getting row count");

    if row_count_ptr.is_null() {
        error!("row_count_ptr is null");
        return SqlReturn::ERROR;
    }

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    match &statement_handle.active_statement {
        Some(stmt) => {
            let changes = stmt.row_changes();
            debug!("row_changes={}", changes);
            unsafe {
                *row_count_ptr = if changes > 0 {
                    debug!("Returning row count: {}", changes);
                    changes as isize
                } else {
                    // SELECT or no DML — row count not applicable
                    debug!("Returning -1 (row count not applicable)");
                    -1
                };
            }
            SqlReturn::SUCCESS
        }
        None => {
            error!("No prepared statement found");
            unsafe {
                *row_count_ptr = -1;
            }
            SqlReturn::ERROR
        }
    }
}
