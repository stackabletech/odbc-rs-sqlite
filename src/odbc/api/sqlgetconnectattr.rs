use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLGetConnectAttrW(
    _connection_handle: *mut c_void,
    _attribute: i32,
    _value_ptr: *mut c_void,
    _buffer_length: i32,
    _string_length_ptr: *mut i32,
) -> SqlReturn {
    println!("SQLGetConnectAttrW INFO");
    SqlReturn::SUCCESS
}
