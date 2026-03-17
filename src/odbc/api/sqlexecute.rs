use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLExecute executes a prepared statement.
///
/// Because the driver executes eagerly at prepare time, this is effectively
/// a no-op — it verifies a statement exists and returns SUCCESS.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLExecute(statement_handle: *mut c_void) -> SqlReturn {
    info!("SQLExecute");

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    match statement_handle.active_statement.as_mut() {
        Some(stmt) => match stmt.execute() {
            Ok(()) => {
                debug!("Statement executed successfully");
                SqlReturn::SUCCESS
            }
            Err(err) => {
                error!("Failed to execute statement: {}", err);
                SqlReturn::ERROR
            }
        },
        None => {
            error!("No prepared statement found");
            SqlReturn::ERROR
        }
    }
}
