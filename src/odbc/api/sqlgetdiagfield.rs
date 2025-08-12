use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLGetDiagFieldW(
    _handle_type: i16,
    _handle: *mut c_void,
    _record_number: i16,
    _diag_identifier: i16,
    _diag_info_ptr: *mut c_void,
    _buffer_length: i16,
    _string_length_ptr: *mut i16,
) -> SqlReturn {
    info!("SQLGetDiagFieldW");
    SqlReturn::SUCCESS
}
