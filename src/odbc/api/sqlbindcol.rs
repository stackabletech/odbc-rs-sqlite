use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLBindCol(
    _statement_handle: *mut c_void,
    _column_number: u16,
    _target_type: i16,
    _target_value: *mut c_void,
    _buffer_length: isize,
    _length_or_indicator: *mut isize,
) -> SqlReturn {
    println!("SQLBindCol INFO");
    SqlReturn::SUCCESS
}
