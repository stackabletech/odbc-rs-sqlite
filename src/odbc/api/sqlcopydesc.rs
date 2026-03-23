use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLCopyDesc(
    _source_desc_handle: *mut c_void,
    _target_desc_handle: *mut c_void,
) -> SqlReturn {
    info!("SQLCopyDesc");
    SqlReturn::SUCCESS
}
