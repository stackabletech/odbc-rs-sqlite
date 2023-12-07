use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLSetDescFieldW(
    _description_handle: *mut c_void,
    _rec_number: i16,
    _field_identifier: i16,
    _value_ptr: *mut c_void,
    _buffer_length: i32,
) -> SqlReturn {
    println!("SQLSetDescFieldW INFO");
    SqlReturn::SUCCESS
}
