use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::{get_from_wrapper, maybe_utf16_to_string};
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLPrepareW prepares an SQL statement for execution.
///
/// This function prepares the SQL statement but does not execute it.
/// The prepared statement can later be executed with SQLExecute.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLPrepareW(
    statement_handle: *mut c_void,
    statement_text: *const u16,
    text_length: i16,
) -> SqlReturn {
    info!("text_length={}", text_length);

    // Get the statement handle
    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    // Convert UTF-16 statement text to String
    let sql_text = match maybe_utf16_to_string(statement_text, text_length) {
        Some(text) => text,
        None => {
            error!("Failed to convert SQL statement text");
            return SqlReturn::ERROR;
        }
    };

    debug!("Preparing SQL: {}", sql_text);

    // Prepare the statement using rusqlite
    match statement_handle.sqlite_connection.prepare(&sql_text) {
        Ok(stmt) => {
            debug!("Statement prepared successfully");
            statement_handle.statement = Some(stmt);
            SqlReturn::SUCCESS
        }
        Err(err) => {
            error!("Failed to prepare statement: {}", err);
            SqlReturn::ERROR
        }
    }
}
