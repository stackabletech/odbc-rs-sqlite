use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLPutData(
    _statement_handle: *mut c_void,
    _data: *mut c_void,
    _str_len_or_ind: isize,
) -> SqlReturn {
    println!("SQLPutData INFO");
    SqlReturn::SUCCESS
}
