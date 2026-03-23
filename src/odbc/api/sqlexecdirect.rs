use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::{get_from_wrapper, maybe_utf16_to_string};
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLExecDirectW prepares and executes an SQL statement in one step.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLExecDirectW(
    statement_handle: *mut c_void,
    statement_text: *const u16,
    text_length: i32,
) -> SqlReturn {
    info!("text_length={}", text_length);

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    let sql_text = match maybe_utf16_to_string(statement_text, text_length as i16) {
        Some(text) => text,
        None => {
            error!("Failed to convert SQL statement text");
            return SqlReturn::ERROR;
        }
    };

    debug!("Executing SQL: {}", sql_text);

    match statement_handle.connection.prepare_statement(&sql_text) {
        Ok(stmt) => {
            debug!("Statement executed successfully");
            statement_handle.active_statement = Some(stmt);
            SqlReturn::SUCCESS
        }
        Err(err) => {
            error!("Failed to execute statement: {}", err);
            SqlReturn::ERROR
        }
    }
}
