use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::{get_from_wrapper, maybe_utf16_to_string};
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLPrepareW prepares an SQL statement for execution.
///
/// The statement is eagerly executed and results are collected immediately,
/// so that `SQLNumResultCols`, `SQLColAttribute`, etc. work before `SQLExecute`.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLPrepareW(
    statement_handle: *mut c_void,
    statement_text: *const u16,
    text_length: i16,
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

    let sql_text = match maybe_utf16_to_string(statement_text, text_length) {
        Some(text) => text,
        None => {
            error!("Failed to convert SQL statement text");
            return SqlReturn::ERROR;
        }
    };

    debug!("Preparing SQL: {}", sql_text);

    match statement_handle.connection.prepare_statement(&sql_text) {
        Ok(stmt) => {
            debug!("Statement prepared successfully");
            statement_handle.active_statement = Some(stmt);
            SqlReturn::SUCCESS
        }
        Err(err) => {
            error!("Failed to prepare statement: {}", err);
            SqlReturn::ERROR
        }
    }
}
