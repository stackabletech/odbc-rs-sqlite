use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLParamData(
    _statement_handle: *mut c_void,
    _value_ptr_ptr: *mut c_void,
) -> SqlReturn {
    println!("SQLParamData INFO");
    SqlReturn::SUCCESS
}
