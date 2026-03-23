use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLFetch advances the cursor to the next row in the result set.
///
/// Returns SQL_NO_DATA when no more rows are available.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLFetch(statement_handle: *mut c_void) -> SqlReturn {
    info!("Fetching next row");

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    match statement_handle.active_statement.as_mut() {
        Some(stmt) => match stmt.fetch_next_row() {
            Ok(true) => {
                debug!("Successfully fetched row");
                SqlReturn::SUCCESS
            }
            Ok(false) => {
                info!("No more data available");
                SqlReturn::NO_DATA
            }
            Err(err) => {
                error!("Failed to fetch next row: {}", err);
                SqlReturn::ERROR
            }
        },
        None => {
            error!("No result set available. Call SQLExecute first.");
            SqlReturn::ERROR
        }
    }
}
