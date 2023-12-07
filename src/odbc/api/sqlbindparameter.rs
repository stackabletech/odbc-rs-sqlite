use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLBindParameter(
    _statement_handle: *mut c_void,
    _parameter_number: u16,
    _input_output_type: i16,
    _value_type: i16,
    _parameter_type: i16,
    _column_size: usize,
    _decimal_digits: i16,
    _parameter_value_ptr: *mut c_void,
    _buffer_length: isize,
    _length_or_indicator: *mut isize,
) -> SqlReturn {
    println!("SQLBindParameter INFO");
    SqlReturn::SUCCESS
}
