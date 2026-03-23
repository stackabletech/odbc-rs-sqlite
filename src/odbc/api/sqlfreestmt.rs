use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLFreeStmt(_statement_handle: *mut c_void, _option: u16) -> SqlReturn {
    info!("SQLFreeStmt");
    SqlReturn::SUCCESS
}
