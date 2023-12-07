use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub fn SQLRowCount(_statement_handle: *mut c_void, _row_count: *mut isize) -> SqlReturn {
    println!("SQLRowCount INFO");

    unsafe { *_row_count = 1 }

    SqlReturn::SUCCESS
}
