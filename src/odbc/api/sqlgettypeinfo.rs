use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLGetTypeInfo(_statement_handle: *mut c_void, _data_type: i16) -> SqlReturn {
    println!("SQLGetTypeInfo INFO");
    SqlReturn::SUCCESS
}
