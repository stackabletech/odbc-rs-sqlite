use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case, clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub fn SQLSpecialColumnsW(
    _statement_handle: *mut c_void,
    _identifier_type: i16,
    _catalog_name: *const u16,
    _catalog_name_length: i16,
    _schema_name: *const u16,
    _schema_name_length: i16,
    _table_name: *const u16,
    _table_name_length: i16,
    _scope: i16,
    _nullable: i16,
) -> SqlReturn {
    info!("SQLSpecialColumnsW");
    SqlReturn::SUCCESS
}

// TODO sql.h differs from docs in type of identifier_type unsigned vs signed
