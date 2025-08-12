use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub fn SQLSetStmtAttrW(
    _statement_handle: *mut c_void,
    _attribute: i32,
    _value_ptr: *mut c_void,
    _string_length: i32,
) -> SqlReturn {
    info!("SQLSetStmtAttrW");
    SqlReturn::SUCCESS
}
