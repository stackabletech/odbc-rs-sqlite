use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLMoreResults(_statement_handle: *mut c_void) -> SqlReturn {
    println!("SQLMoreResults INFO");
    SqlReturn::SUCCESS
}
