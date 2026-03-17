use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::{get_from_wrapper, maybe_utf16_to_string};
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLColumnsW(
    statement_handle: *mut c_void,
    _catalog_name: *const u16,
    _catalog_name_length: i16,
    _schema_name: *const u16,
    _schema_name_length: i16,
    table_name: *const u16,
    table_name_length: i16,
    _column_name: *const u16,
    _column_name_length: i16,
) -> SqlReturn {
    info!("SQLColumnsW");

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    let table_name = match maybe_utf16_to_string(table_name, table_name_length) {
        Some(name) => name,
        None => {
            error!("Table name is required for SQLColumnsW");
            return SqlReturn::ERROR;
        }
    };

    debug!("Getting columns for table: {}", table_name);

    match statement_handle.connection.get_columns(&table_name) {
        Ok(stmt) => {
            statement_handle.active_statement = Some(stmt);
            SqlReturn::SUCCESS
        }
        Err(err) => {
            error!("impl_get_columns failed: {}", err);
            SqlReturn::ERROR
        }
    }
}
