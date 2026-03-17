use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLNumResultCols returns the number of columns in a result set.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SQLNumResultCols(
    statement_handle: *mut c_void,
    column_count_ptr: *mut i16,
) -> SqlReturn {
    info!("Getting number of result cols");

    if column_count_ptr.is_null() {
        error!("column_count_ptr is null");
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
            let num_cols = stmt.column_count();
            debug!("Found {} columns", num_cols);
            unsafe {
                *column_count_ptr = num_cols as i16;
            }
            SqlReturn::SUCCESS
        }
        None => {
            error!("No prepared statement found");
            unsafe {
                *column_count_ptr = 0;
            }
            SqlReturn::ERROR
        }
    }
}
