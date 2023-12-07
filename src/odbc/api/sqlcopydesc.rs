use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLCopyDesc(
    _source_desc_handle: *mut c_void,
    _target_desc_handle: *mut c_void,
) -> SqlReturn {
    println!("SQLCopyDesc INFO");
    SqlReturn::SUCCESS
}
