use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLFetch fetches the next row of data from the result set.
///
/// This function advances the cursor to the next row in the result set and
/// retrieves the data for all bound columns. If there are no more rows,
/// it returns SQL_NO_DATA.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLFetch(statement_handle: *mut c_void) -> SqlReturn {
    info!("Fetching next row");

    // Get the statement handle
    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    // Check if we have rows available (from previous SQLExecute call)
    match statement_handle.rows.as_mut() {
        Some(rows) => {
            debug!("Found result set, fetching next row");

            // Try to get the next row
            match rows.next() {
                Ok(row_option) => match row_option {
                    Some(row) => {
                        debug!("Successfully fetched row");
                        statement_handle.row = Some(row);
                        SqlReturn::SUCCESS
                    }
                    None => {
                        info!("No more data available");
                        SqlReturn::NO_DATA
                    }
                },
                Err(err) => {
                    error!("Failed to fetch next row: {}", err);
                    SqlReturn::ERROR
                }
            }
        }
        None => {
            error!("No result set available. Call SQLExecute first.");
            SqlReturn::ERROR
        }
    }
}
