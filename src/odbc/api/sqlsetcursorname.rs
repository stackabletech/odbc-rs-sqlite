use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLSetCursorNameW(
    _statement_handle: *mut c_void,
    _cursor_name: *const u16,
    _cursor_name_length: i16,
) -> SqlReturn {
    println!("SQLSetCursorNameW INFO");
    SqlReturn::SUCCESS
}
