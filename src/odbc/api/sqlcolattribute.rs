use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{Desc, HandleType, SqlReturn};
use std::ffi::c_void;
use std::ptr;
use tracing::{debug, error, info};

/// SQLColAttributeW returns descriptor information for a column in a result set.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn SQLColAttributeW(
    statement_handle: *mut c_void,
    column_number: u16,
    field_identifier: u16,
    character_attribute_ptr: *mut c_void,
    buffer_length: i16,
    string_length_ptr: *mut i16,
    numeric_attribute_ptr: *mut isize,
) -> SqlReturn {
    info!(
        "column_number={}, field_identifier={}, buffer_length={}",
        column_number, field_identifier, buffer_length
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

    let desc_result = match field_identifier {
        1001 => Some(Desc::Count),
        1011 => Some(Desc::Name),
        1002 => Some(Desc::Type),
        2 => Some(Desc::ConciseType),
        1003 => Some(Desc::Length),
        1013 => Some(Desc::OctetLength),
        6 => Some(Desc::DisplaySize),
        1008 => Some(Desc::Nullable),
        1012 => Some(Desc::Unnamed),
        18 => Some(Desc::Label),
        _ => None,
    };

    let desc = match desc_result {
        Some(desc) => desc,
        None => {
            info!("Unsupported field identifier {}", field_identifier);
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 0;
                }
            }
            return SqlReturn::SUCCESS;
        }
    };

    // Convert to 0-indexed for the trait method
    let col_index = (column_number - 1) as usize;

    debug!("Processing {:?} for column {}", desc, col_index);

    match desc {
        Desc::Count => {
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = stmt.column_count() as isize;
                }
                debug!("Returning column count: {}", stmt.column_count());
            }
            SqlReturn::SUCCESS
        }
        Desc::Name => match stmt.column_name(col_index) {
            Ok(name) => return_string_attribute(
                &name,
                character_attribute_ptr,
                buffer_length,
                string_length_ptr,
            ),
            Err(err) => {
                error!("Could not get column name for index {}: {}", col_index, err);
                SqlReturn::ERROR
            }
        },
        Desc::Type | Desc::ConciseType => {
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 12; // SQL_VARCHAR
                }
                debug!("Returning type: VARCHAR");
            }
            SqlReturn::SUCCESS
        }
        Desc::Length | Desc::OctetLength => {
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 255;
                }
                debug!("Returning length: 255");
            }
            SqlReturn::SUCCESS
        }
        Desc::DisplaySize => {
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 25;
                }
                debug!("Returning display size: 25");
            }
            SqlReturn::SUCCESS
        }
        Desc::Nullable => {
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 1; // SQL_NULLABLE
                }
                debug!("Returning nullable: true");
            }
            SqlReturn::SUCCESS
        }
        Desc::Unnamed => {
            if !numeric_attribute_ptr.is_null() {
                let is_named = stmt.column_name(col_index).is_ok();
                unsafe {
                    *numeric_attribute_ptr = if is_named { 0 } else { 1 };
                }
                debug!("Returning unnamed: {}", !is_named);
            }
            SqlReturn::SUCCESS
        }
        Desc::Label => match stmt.column_name(col_index) {
            Ok(name) => return_string_attribute(
                &name,
                character_attribute_ptr,
                buffer_length,
                string_length_ptr,
            ),
            Err(err) => {
                error!(
                    "Could not get column label for index {}: {}",
                    col_index, err
                );
                SqlReturn::ERROR
            }
        },
        _ => {
            info!("Unsupported field identifier {:?}, returning default", desc);
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 0;
                }
            }
            SqlReturn::SUCCESS
        }
    }
}

fn return_string_attribute(
    value: &str,
    character_attribute_ptr: *mut c_void,
    buffer_length: i16,
    string_length_ptr: *mut i16,
) -> SqlReturn {
    debug!("Returning string: '{}'", value);

    let utf16_value: Vec<u16> = value.encode_utf16().collect();
    let utf16_len = utf16_value.len() as i16;

    if !string_length_ptr.is_null() {
        unsafe {
            *string_length_ptr = utf16_len * 2; // Length in bytes
        }
    }

    if !character_attribute_ptr.is_null() && buffer_length > 0 {
        let max_chars = (buffer_length / 2) as usize;
        let copy_len = std::cmp::min(utf16_value.len(), max_chars);

        unsafe {
            ptr::copy_nonoverlapping(
                utf16_value.as_ptr() as *const c_void,
                character_attribute_ptr,
                copy_len * 2,
            );
        }

        if copy_len < utf16_value.len() {
            return SqlReturn::SUCCESS_WITH_INFO;
        }
    }

    SqlReturn::SUCCESS
}
