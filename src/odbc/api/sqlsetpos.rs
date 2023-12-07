use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub fn SQLSetPos(
    _statement_handle: *mut c_void,
    _row_number: i32,
    _operation: u64,
    _lock_type: u16,
) -> SqlReturn {
    println!("SQLSetPos INFO");
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
