use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLDisconnect(_connection_handle: *mut c_void) -> SqlReturn {
    println!("SQLDisconnect INFO");
    SqlReturn::SUCCESS
}
