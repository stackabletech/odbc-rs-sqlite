use odbc_sys::SqlReturn;
use std::os::raw::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLSetDescRecW(
    _descriptor_handle: *mut c_void,
    _rec_number: i16,
    _desc_type: i16,
    _subtype: i16,
    _length: isize,
    _precision: i16,
    _scale: i16,
    _data_ptr: *mut c_void, // TODO u16?
    _string_length_ptr: *mut isize,
    _indicator_ptr: *mut isize,
) -> SqlReturn {
    info!("SQLSetDescRecW");
    SqlReturn::SUCCESS
}
