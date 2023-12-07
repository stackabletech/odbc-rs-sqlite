use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, Pointer, SqlReturn};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLFetch(statement_handle: Pointer) -> SqlReturn {
    println!("SQLFetch INFO");

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(env) => env,
            Err(err) => {
                println!("SQLFetch ERROR: {}", err);
                return SqlReturn::ERROR;
            }
        };

    let foo = statement_handle.rows.as_mut().unwrap();
    let bar = foo.next().unwrap();
    match bar {
        None => {
            println!("no data");
            return SqlReturn::NO_DATA;
        }
        Some(row) => statement_handle.row = Some(row),
    }

    SqlReturn::SUCCESS
}
