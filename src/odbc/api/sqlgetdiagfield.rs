use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLGetDiagFieldW(
    _handle_type: i16,
    _handle: *mut c_void,
    _record_number: i16,
    _diag_identifier: i16,
    _diag_info_ptr: *mut c_void,
    _buffer_length: i16,
    _string_length_ptr: *mut i16,
) -> SqlReturn {
    println!("SQLGetDiagFieldW INFO");
    SqlReturn::SUCCESS
}
