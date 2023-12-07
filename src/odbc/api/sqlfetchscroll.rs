use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLFetchScroll(
    _statement_handle: *mut c_void,
    _fetch_orientation: i16,
    _fetch_offset: isize,
) -> SqlReturn {
    println!("SQLFetchScroll INFO");
    SqlReturn::SUCCESS
}
