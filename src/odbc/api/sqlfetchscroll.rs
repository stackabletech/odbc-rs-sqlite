use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLFetchScroll(
    _statement_handle: *mut c_void,
    _fetch_orientation: i16,
    _fetch_offset: isize,
) -> SqlReturn {
    info!("SQLFetchScroll");
    SqlReturn::SUCCESS
}
