use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLBulkOperations(_statement_handle: *mut c_void, _operation: u16) -> SqlReturn {
    println!("SQLBulkOperations INFO");
    SqlReturn::SUCCESS
}
