use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use std::ptr;
use tracing::{debug, error, info};

/// SQLDescribeColW returns the result descriptor for one column in the result set.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLDescribeColW(
    statement_handle: *mut c_void,
    column_number: u16,
    column_name: *mut u16,
    buffer_length: i16,
    name_length_ptr: *mut i16,
    data_type_ptr: *mut i16,
    column_size_ptr: *mut usize,
    decimal_digits_ptr: *mut i16,
    nullable_ptr: *mut i16,
) -> SqlReturn {
    info!(
        "column_number={}, buffer_length={}",
        column_number, buffer_length
    );

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    let stmt = match &statement_handle.active_statement {
        Some(stmt) => stmt,
        None => {
            error!("No prepared statement found");
            return SqlReturn::ERROR;
        }
    };

    if column_number == 0 || column_number as usize > stmt.column_count() {
        error!(
            "Invalid column number {}. Valid range: 1-{}",
            column_number,
            stmt.column_count()
        );
        return SqlReturn::ERROR;
    }

    let col_index = (column_number - 1) as usize;

    let mut result = SqlReturn::SUCCESS;
    match stmt.column_name(col_index) {
        Ok(name) => {
            debug!("Column {} name: '{}'", column_number, name);

            let utf16_value: Vec<u16> = name.encode_utf16().collect();
            let utf16_len = utf16_value.len() as i16;

            if !name_length_ptr.is_null() {
                unsafe {
                    *name_length_ptr = utf16_len;
                }
            }

            // buffer_length is in characters (u16 elements) per ODBC spec
            if !column_name.is_null() && buffer_length > 0 {
                let max_chars = buffer_length as usize;
                let copy_len = std::cmp::min(utf16_value.len(), max_chars.saturating_sub(1));

                unsafe {
                    ptr::copy_nonoverlapping(utf16_value.as_ptr(), column_name, copy_len);
                    *column_name.add(copy_len) = 0;
                }

                if copy_len < utf16_value.len() {
                    result = SqlReturn::SUCCESS_WITH_INFO;
                }
            }
        }
        Err(err) => {
            error!("Could not get column name for index {}: {}", col_index, err);
            return SqlReturn::ERROR;
        }
    }

    if !data_type_ptr.is_null() {
        unsafe {
            *data_type_ptr = 12; // SQL_VARCHAR
        }
        debug!("Returning type: SQL_VARCHAR");
    }

    if !column_size_ptr.is_null() {
        unsafe {
            *column_size_ptr = 255;
        }
        debug!("Returning column size: 255");
    }

    if !decimal_digits_ptr.is_null() {
        unsafe {
            *decimal_digits_ptr = 0;
        }
    }

    if !nullable_ptr.is_null() {
        unsafe {
            *nullable_ptr = 1; // SQL_NULLABLE
        }
        debug!("Returning nullable: SQL_NULLABLE");
    }

    result
}
