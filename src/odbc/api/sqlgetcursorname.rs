use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLGetCursorNameW(
    _statement_handle: *mut c_void,
    _cursor_name: *mut u16,
    _buffer_length: i16,
    _name_length_ptr: *mut i16,
) -> SqlReturn {
    println!("SQLGetCursorNameW INFO");
    SqlReturn::SUCCESS
}
