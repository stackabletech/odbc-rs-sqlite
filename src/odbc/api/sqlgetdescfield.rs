use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLGetDescFieldW(
    _descriptor_handle: *mut c_void,
    _record_number: i16,
    _field_identifier: i16,
    _value_ptr: *mut c_void,
    _buffer_length: i32,
    _string_length_ptr: *mut i32,
) -> SqlReturn {
    info!("SQLGetDescFieldW");
    SqlReturn::SUCCESS
}
