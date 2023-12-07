use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLDescribeParam(
    _statement_handle: *mut c_void,
    _parameter_number: u16,
    _data_type_ptr: *mut i16,
    _parameter_size_ptr: *mut usize,
    _decimal_digits_ptr: *mut i16,
    _nullable_ptr: *mut i16,
) -> SqlReturn {
    println!("SQLDescribeParam INFO");
    SqlReturn::SUCCESS
}
