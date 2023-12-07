use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLGetDescRecW(
    _descriptor_handle: *mut c_void,
    _record_number: i16,
    _name: *mut u16,
    _buffer_length: i16,
    _string_length_ptr: *mut i16,
    _type_ptr: *mut i16,
    _sub_type_ptr: *mut i16,
    _length_ptr: *mut isize,
    _precision_ptr: *mut i16,
    _scale_ptr: *mut i16,
    _nullable_ptr: *mut i16,
) -> SqlReturn {
    println!("SQLGetDescRecW INFO");
    SqlReturn::SUCCESS
}
