use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub fn SQLTablePrivilegesW(
    _statement_handle: *mut c_void,
    _catalog_name: *const u16,
    _catalog_name_length: i16,
    _schema_name: *const u16,
    _schema_name_length: i16,
    _table_name: *const u16,
    _table_name_length: i16,
) -> SqlReturn {
    println!("SQLTablePrivilegesW INFO");
    SqlReturn::SUCCESS
}
