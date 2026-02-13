use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use std::ptr;
use tracing::{debug, error, info};

/// SQLDescribeColW returns the result descriptor for one column in the result set.
///
/// This function provides column metadata including name, data type, size,
/// decimal digits, and nullability. It must be called after a statement has
/// been prepared (via SQLPrepareW or SQLExecDirectW).
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLDescribeColW(
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

    // Get the statement handle
    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    // Check if we have a prepared statement
    let stmt = match &statement_handle.statement {
        Some(stmt) => stmt,
        None => {
            error!("No prepared statement found");
            return SqlReturn::ERROR;
        }
    };

    // Validate column number (1-indexed in ODBC)
    if column_number == 0 || column_number as usize > stmt.column_count() {
        error!(
            "Invalid column number {}. Valid range: 1-{}",
            column_number,
            stmt.column_count()
        );
        return SqlReturn::ERROR;
    }

    // Convert to 0-indexed for SQLite
    let col_index = (column_number - 1) as usize;

    // Write column name as UTF-16
    let mut result = SqlReturn::SUCCESS;
    match stmt.column_name(col_index) {
        Ok(name) => {
            debug!("Column {} name: '{}'", column_number, name);

            let utf16_value: Vec<u16> = name.encode_utf16().collect();
            let utf16_len = utf16_value.len() as i16;

            // Set the actual name length in characters (excluding null terminator)
            if !name_length_ptr.is_null() {
                unsafe {
                    *name_length_ptr = utf16_len;
                }
            }

            // Copy the name string if buffer is provided
            // buffer_length is in characters (u16 elements) per ODBC spec
            if !column_name.is_null() && buffer_length > 0 {
                let max_chars = buffer_length as usize;
                let copy_len = std::cmp::min(utf16_value.len(), max_chars.saturating_sub(1));

                unsafe {
                    ptr::copy_nonoverlapping(utf16_value.as_ptr(), column_name, copy_len);
                    // Null-terminate
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

    // Set data type - SQLite is dynamically typed, report VARCHAR for all columns
    if !data_type_ptr.is_null() {
        unsafe {
            *data_type_ptr = 12; // SQL_VARCHAR
        }
        debug!("Returning type: SQL_VARCHAR");
    }

    // Set column size - default VARCHAR length
    if !column_size_ptr.is_null() {
        unsafe {
            *column_size_ptr = 255;
        }
        debug!("Returning column size: 255");
    }

    // Set decimal digits - 0 for VARCHAR
    if !decimal_digits_ptr.is_null() {
        unsafe {
            *decimal_digits_ptr = 0;
        }
    }

    // Set nullable - SQLite columns are generally nullable
    if !nullable_ptr.is_null() {
        unsafe {
            *nullable_ptr = 1; // SQL_NULLABLE
        }
        debug!("Returning nullable: SQL_NULLABLE");
    }

    result
}
