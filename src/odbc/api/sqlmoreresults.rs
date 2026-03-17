use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{error, info};

/// SQLMoreResults returns SQL_NO_DATA — SQLite does not support multiple result sets.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLMoreResults(statement_handle: *mut c_void) -> SqlReturn {
    info!("Checking for additional result sets");

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    match &statement_handle.active_statement {
        Some(_) => {
            info!("No additional result sets available (SQLite limitation)");
            SqlReturn::NO_DATA
        }
        None => {
            error!("No prepared statement found");
            SqlReturn::ERROR
        }
    }
}
