use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLNumParams(
    _statement_handle: *mut c_void,
    _parameter_count_ptr: *mut i16,
) -> SqlReturn {
    info!("SQLNumParams");
    SqlReturn::SUCCESS
}
