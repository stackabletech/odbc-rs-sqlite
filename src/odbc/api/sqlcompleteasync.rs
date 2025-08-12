use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLCompleteAsync(
    _handle_type: i16,
    _handle: *mut c_void,
    _async_ret_code_ptr: *mut i16,
) -> SqlReturn {
    info!("SQLCompleteAsync");
    SqlReturn::SUCCESS
}
