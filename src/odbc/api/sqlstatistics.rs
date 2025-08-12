use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub fn SQLStatisticsW(
    _statement_handle: *mut c_void,
    _catalog_name: *const u16,
    _catalog_name_length: i16,
    _schema_name: *const u16,
    _schema_name_length: i16,
    _table_name: *const u16,
    _table_name_length: i16,
    _unique: u16,
    _accuracy: u16,
) -> SqlReturn {
    info!("SQLStatisticsW");
    SqlReturn::SUCCESS
}

// TODO sql.h differs from docs in name of last parameter
