use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::{get_from_wrapper, maybe_utf16_to_string};
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLExecDirectW prepares and executes an SQL statement in one step.
///
/// This function combines the functionality of SQLPrepareW and SQLExecute,
/// preparing and executing the SQL statement immediately without the need
/// for separate preparation and execution calls.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLExecDirectW(
    statement_handle: *mut c_void,
    statement_text: *const u16,
    text_length: i32,
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
    let sql_text = match maybe_utf16_to_string(statement_text, text_length as i16) {
        Some(text) => text,
        None => {
            error!("Failed to convert SQL statement text");
            return SqlReturn::ERROR;
        }
    };

    debug!("Executing SQL: {}", sql_text);

    // For SELECT statements, prepare and execute with query()
    if sql_text.trim().to_uppercase().starts_with("SELECT") {
        // Prepare the statement
        match statement_handle.sqlite_connection.prepare(&sql_text) {
            Ok(stmt) => {
                debug!("SELECT statement prepared successfully");
                statement_handle.statement = Some(stmt);

                // Execute the prepared statement
                match statement_handle.statement {
                    Some(ref mut stmt) => match stmt.query([]) {
                        Ok(rows) => {
                            debug!("SELECT statement executed successfully");
                            statement_handle.rows = Some(rows);
                            SqlReturn::SUCCESS
                        }
                        Err(err) => {
                            error!("Failed to execute SELECT statement: {}", err);
                            SqlReturn::ERROR
                        }
                    },
                    None => {
                        error!("Failed to get prepared statement reference");
                        SqlReturn::ERROR
                    }
                }
            }
            Err(err) => {
                error!("Failed to prepare SELECT statement: {}", err);
                SqlReturn::ERROR
            }
        }
    } else {
        // For non-SELECT statements (INSERT, UPDATE, DELETE, etc.), use execute()
        match statement_handle.sqlite_connection.execute(&sql_text, []) {
            Ok(affected_rows) => {
                debug!(
                    "Non-SELECT statement executed successfully, affected rows: {}",
                    affected_rows
                );
                // Clear any previous result set since this wasn't a SELECT
                statement_handle.statement = None;
                statement_handle.rows = None;
                statement_handle.row = None;
                SqlReturn::SUCCESS
            }
            Err(err) => {
                error!("Failed to execute non-SELECT statement: {}", err);
                SqlReturn::ERROR
            }
        }
    }
}
