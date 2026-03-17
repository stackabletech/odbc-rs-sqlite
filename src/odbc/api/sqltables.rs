use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::implementation::tables::impl_get_tables;
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
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    match impl_get_tables(statement_handle) {
        Ok(()) => SqlReturn::SUCCESS,
        Err(err) => {
            error!("impl_get_tables failed: {}", err);
            SqlReturn::ERROR
        }
    }
}
