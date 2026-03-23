use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLGetTypeInfo(_statement_handle: *mut c_void, _data_type: i16) -> SqlReturn {
    info!("SQLGetTypeInfo");
    SqlReturn::SUCCESS
}
