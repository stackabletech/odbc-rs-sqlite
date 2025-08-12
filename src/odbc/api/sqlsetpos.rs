use odbc_sys::SqlReturn;
use std::ffi::c_void;
use tracing::info;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub fn SQLSetPos(
    _statement_handle: *mut c_void,
    _row_number: i32,
    _operation: u64,
    _lock_type: u16,
) -> SqlReturn {
    info!("SQLSetPos");
    SqlReturn::SUCCESS
}

/*
      SQLUSMALLINT    Operation,

#ifdef _WIN64
typedef UINT64 SQLSETPOSIROW;
#else
#define SQLSETPOSIROW SQLUSMALLINT
#endif

 */
