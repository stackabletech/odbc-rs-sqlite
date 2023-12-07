use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLNumParams(
    _statement_handle: *mut c_void,
    _parameter_count_ptr: *mut i16,
) -> SqlReturn {
    println!("SQLNumParams INFO");
    SqlReturn::SUCCESS
}
