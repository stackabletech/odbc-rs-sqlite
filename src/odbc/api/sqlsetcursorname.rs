use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLSetCursorNameW(
    _statement_handle: *mut c_void,
    _cursor_name: *const u16,
    _cursor_name_length: i16,
) -> SqlReturn {
    info!("SQLSetCursorNameW");
    SqlReturn::SUCCESS
}
