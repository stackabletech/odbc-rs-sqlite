use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLNumResultCols(
    statement_handle: *mut c_void,
    column_count_ptr: &mut i16,
) -> SqlReturn {
    println!("SQLNumResultCols INFO");

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(env) => env,
            Err(err) => {
                println!("SQLNumResultsCol ERROR: {}", err);
                return SqlReturn::ERROR;
            }
        };

    let num_cols = statement_handle.statement.as_ref().unwrap().column_count();

    *column_count_ptr = num_cols as i16;

    SqlReturn::SUCCESS
}
