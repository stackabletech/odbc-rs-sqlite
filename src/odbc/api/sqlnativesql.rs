use odbc_sys::SqlReturn;
use std::os::raw::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLNativeSqlW(
    _connection_handle: *mut c_void,
    _in_statement_text: *mut u16,
    _in_statement_text_length: i32,
    _out_statement_text: *mut u16,
    _out_statement_text_length: *mut i32,
) -> SqlReturn {
    info!("SQLNativeSqlW ");
    SqlReturn::SUCCESS
}

// TODO: This is missing in odbc-sys, not sure if the char things are correct
