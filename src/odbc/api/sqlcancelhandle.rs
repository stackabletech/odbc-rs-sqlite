use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLCancelHandle(_handle_type: i16, _handle: *mut c_void) -> SqlReturn {
    info!("SQLCancelHandle");
    SqlReturn::SUCCESS
}
