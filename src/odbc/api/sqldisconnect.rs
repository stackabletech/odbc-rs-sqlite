use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLDisconnect(_connection_handle: *mut c_void) -> SqlReturn {
    info!("SQLDisconnect");
    SqlReturn::SUCCESS
}
