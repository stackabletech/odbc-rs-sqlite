use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLPrepareW(
    _statement_handle: *mut c_void,
    _statement_text: *const u16,
    _text_length: i16,
) -> SqlReturn {
    println!("SQLPrepareW INFO");
    SqlReturn::SUCCESS
}
