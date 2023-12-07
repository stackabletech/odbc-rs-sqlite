use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLDescribeCol(
    _statement_handle: *mut c_void,
    _column_number: u16,
    _column_name: *mut u16,
    _buffer_length: i16,
    _name_length_ptr: *mut i16,
    _data_type_ptr: *mut i16,
    _column_size_ptr: *mut usize,
    _decimal_digits_ptr: *mut i16,
    _nullable_ptr: *mut i16,
) -> SqlReturn {
    println!("SQLDescribeCol INFO");
    SqlReturn::SUCCESS
}
