use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLGetDiagRecW(
    _handle_type: i16,
    _handle: *mut c_void,
    record_number: i16,
    _sql_state: *mut u16,
    _native_error_ptr: *mut i32,
    _message_text: *mut u32,
    _buffer_length: i16,
    _text_length_ptr: *mut i16,
) -> SqlReturn {
    println!("SQLGetDiagRecW INFO: record_number={}", record_number);

    // We don't have any diagnostic records implemented yet
    // Return NO_DATA to indicate no more records available
    // Otherwise clients using this can run into an infinite loop
    SqlReturn::NO_DATA
}
