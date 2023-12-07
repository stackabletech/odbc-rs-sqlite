use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLProcedureColumnsW(
    _statement_handle: *mut c_void,

    _catalog_name: *const u16,
    _catalog_name_length: i16,

    _schema_name: *const u16,
    _schema_name_length: i16,

    _proc_name: *const u16,
    _proc_name_length: i16,

    _column_name: *const u16,
    _column_name_length: i16,
) -> SqlReturn {
    println!("SQLProcedureColumnsW INFO");
    SqlReturn::SUCCESS
}
