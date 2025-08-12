use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLParamData(
    _statement_handle: *mut c_void,
    _value_ptr_ptr: *mut c_void,
) -> SqlReturn {
    info!("SQLParamData");
    SqlReturn::SUCCESS
}
