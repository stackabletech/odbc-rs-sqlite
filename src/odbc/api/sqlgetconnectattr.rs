use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLGetConnectAttrW(
    _connection_handle: *mut c_void,
    _attribute: i32,
    _value_ptr: *mut c_void,
    _buffer_length: i32,
    _string_length_ptr: *mut i32,
) -> SqlReturn {
    info!("SQLGetConnectAttrW");
    SqlReturn::SUCCESS
}
