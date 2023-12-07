use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLEndTran(
    _handle_type: i16,
    _handle: *mut c_void,
    _completion_type: i16,
) -> SqlReturn {
    println!("SQLEndTran INFO");
    SqlReturn::SUCCESS
}
