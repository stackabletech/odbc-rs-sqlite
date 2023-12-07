use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLCompleteAsync(
    _handle_type: i16,
    _handle: *mut c_void,
    _async_ret_code_ptr: *mut i16,
) -> SqlReturn {
    println!("SQLCompleteAsync INFO");
    SqlReturn::SUCCESS
}
