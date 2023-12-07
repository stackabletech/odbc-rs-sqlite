use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLExecDirectW(
    _statement_handle: *mut c_void,
    _statement_text: *const u16,
    _text_length: i32,
) -> SqlReturn {
    println!("SQLExecDirectW INFO");
    SqlReturn::SUCCESS
}
