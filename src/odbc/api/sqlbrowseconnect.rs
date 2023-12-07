use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLBrowseConnectW(
    _connection_handle: *mut c_void,
    _in_connection_string: *const u16,
    _in_string_length: i16,
    _out_connection_string: *const u16,
    _buffer_length: i16,
    _out_string_length: *mut i16,
) -> SqlReturn {
    println!("SQLBrowseConnectW INFO");
    SqlReturn::SUCCESS
}
