use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLFreeStmt(_statement_handle: *mut c_void, _option: u16) -> SqlReturn {
    println!("SQLFreeStmt INFO");
    SqlReturn::SUCCESS
}
