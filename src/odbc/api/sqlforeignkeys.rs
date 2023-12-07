use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLForeignKeysW(
    _statement_handle: *mut c_void,

    _pk_catalog_name: *const u16,
    _pk_catalog_name_length: i16,

    _pk_schema_name: *const u16,
    _pk_schema_name_length: i16,

    _pk_table_name: *const u16,
    _pk_table_name_length: i16,

    _fk_catalog_name: *const u16,
    _fk_catalog_name_length: i16,

    _fk_schema_name: *const u16,
    _fk_schema_name_length: i16,
    _fk_table_name: *const u16,
    _fk_table_name_length: i16,
) -> SqlReturn {
    println!("SQLForeignKeysW INFO");
    SqlReturn::SUCCESS
}
