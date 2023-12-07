use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLTablesW(
    statement_handle: *mut c_void,
    _catalog_name: *const u16,
    catalog_name_length: i16,
    _schema_name: *const u16,
    schema_name_length: i16,
    _table_name: *const u16,
    table_name_length: i16,
    _table_type: *const u16,
    table_type_length: i16,
) -> SqlReturn {
    println!("SQLTablesW INFO: catalog_name_length={}, schema_name_length={}, table_name_length={}, table_type_length={}", catalog_name_length, schema_name_length, table_name_length, table_type_length);

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(env) => env,
            Err(err) => {
                println!("SQLTablesW ERROR: {}", err);
                return SqlReturn::ERROR;
            }
        };

    // Assuming `statement_handle` is mutable and has fields for storing `stmt` and `rows`
    let stmt = statement_handle
        .sqlite_connection
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .unwrap();
    statement_handle.statement = Some(stmt);

    if let Some(ref mut stmt) = statement_handle.statement {
        statement_handle.rows = Some(stmt.query([]).unwrap());
    }

    //let catalog_name = maybe_utf16_to_string(catalog_name, catalog_name_length).unwrap_or("DEFAULT".to_string());

    SqlReturn::SUCCESS
}
