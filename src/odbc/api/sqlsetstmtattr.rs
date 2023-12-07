use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub fn SQLSetStmtAttrW(
    _statement_handle: *mut c_void,
    _attribute: i32,
    _value_ptr: *mut c_void,
    _string_length: i32,
) -> SqlReturn {
    println!("SQLSetStmtAttrW INFO");
    SqlReturn::SUCCESS
}
