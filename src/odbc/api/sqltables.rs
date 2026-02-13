use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{error, info};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLTablesW(
    statement_handle: *mut c_void,
    _catalog_name: *const u16,
    catalog_name_length: i16,
    _schema_name: *const u16,
    schema_name_length: i16,
    _table_name: *const u16,
    table_name_length: i16,
    _table_type: *const u16,
    table_type_length: i16,
) -> SqlReturn {
    info!(
        "catalog_name_length={}, schema_name_length={}, table_name_length={}, table_type_length={}",
        catalog_name_length, schema_name_length, table_name_length, table_type_length
    );

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(env) => env,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    // Prepare the query to list tables
    let stmt = match statement_handle
        .sqlite_connection
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
    {
        Ok(stmt) => stmt,
        Err(err) => {
            error!("Failed to prepare table listing query: {}", err);
            return SqlReturn::ERROR;
        }
    };
    statement_handle.statement = Some(stmt);

    if let Some(ref mut stmt) = statement_handle.statement {
        match stmt.query([]) {
            Ok(rows) => {
                statement_handle.rows = Some(rows);
            }
            Err(err) => {
                error!("Failed to execute table listing query: {}", err);
                return SqlReturn::ERROR;
            }
        }
    }

    SqlReturn::SUCCESS
}
