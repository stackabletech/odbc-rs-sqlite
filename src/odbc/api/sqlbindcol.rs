use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLBindCol(
    _statement_handle: *mut c_void,
    _column_number: u16,
    _target_type: i16,
    _target_value: *mut c_void,
    _buffer_length: isize,
    _length_or_indicator: *mut isize,
) -> SqlReturn {
    info!("SQLBindCol");
    SqlReturn::SUCCESS
}
