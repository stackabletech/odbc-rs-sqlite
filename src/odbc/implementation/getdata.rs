use crate::odbc::implementation::alloc_handles::StatementHandle;
use odbc_sys::CDataType;

pub(crate) fn impl_getdata(
    statement_handle: &StatementHandle,
    target_type: &CDataType,
    col_or_param: u16,
) -> String {
    let row = statement_handle.row.unwrap();
    match target_type {
        CDataType::Char => {}
        _ => {}
    }

    match row.get((col_or_param - 1) as usize) {
        Ok(a) => {
            return a;
        }
        Err(err) => println!("{}", err),
    }

    "ERROR".to_string()
}
