use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{Desc, HandleType, SqlReturn};
use std::ffi::c_void;
use std::ptr;
use tracing::{debug, error, info};

/// SQLColAttributeW returns descriptor information for a column in a result set.
///
/// This function provides metadata about columns such as name, type, length, precision, etc.
/// It can return both numeric and string attributes depending on the field identifier.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLColAttributeW(
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

    // Match field identifier directly since Desc doesn't have TryFrom
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
            // For unsupported attributes, return success with default values
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 0;
                }
            }
            return SqlReturn::SUCCESS;
        }
    };

    // Convert to 0-indexed for SQLite
    let col_index = (column_number - 1) as usize;

    debug!("Processing {:?} for column {}", desc, col_index);

    match desc {
        Desc::Count => {
            // Return total number of columns
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = stmt.column_count() as isize;
                }
                debug!("Returning column count: {}", stmt.column_count());
            }
            SqlReturn::SUCCESS
        }
        Desc::Name => {
            // Return column name
            match stmt.column_name(col_index) {
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
            }
        }
        Desc::Type | Desc::ConciseType => {
            // Return SQL data type
            if !numeric_attribute_ptr.is_null() {
                // For now, we'll return VARCHAR for all types since SQLite is dynamically typed
                // In a more complete implementation, we'd examine the column declaration type
                unsafe {
                    *numeric_attribute_ptr = 12; // SQL_VARCHAR
                }
                debug!("Returning type: VARCHAR");
            }
            SqlReturn::SUCCESS
        }
        Desc::Length | Desc::OctetLength => {
            // Return column length - for SQLite, we'll use a reasonable default
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 255; // Default VARCHAR length
                }
                debug!("Returning length: 255");
            }
            SqlReturn::SUCCESS
        }
        Desc::DisplaySize => {
            // Return display size for the column
            // TODO: Make dependent on the column type
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 25; // Default display size
                }
                debug!("Returning display size: 25");
            }
            SqlReturn::SUCCESS
        }
        Desc::Nullable => {
            // For SQLite, columns can generally be nullable
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 1; // SQL_NULLABLE
                }
                debug!("Returning nullable: true");
            }
            SqlReturn::SUCCESS
        }
        Desc::Unnamed => {
            // Return whether column is named or unnamed
            if !numeric_attribute_ptr.is_null() {
                let is_named = stmt.column_name(col_index).is_ok();
                unsafe {
                    *numeric_attribute_ptr = if is_named { 0 } else { 1 }; // SQL_NAMED = 0, SQL_UNNAMED = 1
                }
                debug!("Returning unnamed: {}", !is_named);
            }
            SqlReturn::SUCCESS
        }
        Desc::Label => {
            // Return column label (same as name for SQLite)
            match stmt.column_name(col_index) {
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
            }
        }
        _ => {
            info!("Unsupported field identifier {:?}, returning default", desc);
            // For unsupported attributes, return success with default values
            if !numeric_attribute_ptr.is_null() {
                unsafe {
                    *numeric_attribute_ptr = 0;
                }
            }
            SqlReturn::SUCCESS
        }
    }
}

/// Helper function to return string attributes
fn return_string_attribute(
    value: &str,
    character_attribute_ptr: *mut c_void,
    buffer_length: i16,
    string_length_ptr: *mut i16,
) -> SqlReturn {
    debug!("Returning string: '{}'", value);

    // Convert to UTF-16
    let utf16_value: Vec<u16> = value.encode_utf16().collect();
    let utf16_len = utf16_value.len() as i16;

    // Set the actual length
    if !string_length_ptr.is_null() {
        unsafe {
            *string_length_ptr = utf16_len * 2; // Length in bytes
        }
    }

    // Copy the string if buffer is provided and large enough
    if !character_attribute_ptr.is_null() && buffer_length > 0 {
        let max_chars = (buffer_length / 2) as usize; // Convert bytes to UTF-16 chars
        let copy_len = std::cmp::min(utf16_value.len(), max_chars);

        unsafe {
            ptr::copy_nonoverlapping(
                utf16_value.as_ptr() as *const c_void,
                character_attribute_ptr,
                copy_len * 2, // Copy length in bytes
            );
        }

        if copy_len < utf16_value.len() {
            // String was truncated
            return SqlReturn::SUCCESS_WITH_INFO;
        }
    }

    SqlReturn::SUCCESS
}

/*

   println!(
       "SQLColAttributeW(column_number={}, field_identifier={:?}, buffer_length={})",
       column_number, field_identifier, buffer_length
   );

   match field_identifier {
       Desc::Count => {}
       Desc::Type => {}
       Desc::Length => {}
       Desc::OctetLengthPtr => {}
       Desc::Precision => {}
       Desc::Scale => {}
       Desc::DatetimeIntervalCode => {}
       Desc::Nullable => {}
       Desc::IndicatorPtr => {}
       Desc::DataPtr => {}
       Desc::Name => {}
       Desc::Unnamed => {}
       Desc::OctetLength => {}
       Desc::AllocType => {}
       Desc::ArraySize => {}
       Desc::ArrayStatusPtr => {}
       Desc::AutoUniqueValue => {}
       Desc::BaseColumnName => {}
       Desc::BaseTableName => {}
       Desc::BindOffsetPtr => {}
       Desc::BindType => {}
       Desc::CaseSensitive => {}
       Desc::CatalogName => {}
       Desc::ConciseType => {}
       Desc::DatetimeIntervalPrecision => {}
       Desc::DisplaySize => unsafe {
           *numeric_attribute_ptr = 20;
       },
       Desc::FixedPrecScale => {}
       Desc::Label => {
           if !character_attribute_ptr.is_null() {
               let os_string =
                   U16CString::from_str("foobar").expect("U16CString::from_str failed");

               // Make sure 'buffer_length' is the maximum length you can handle in wide characters
               if buffer_length >= os_string.len() as i16 {
                   // Set string_length_ptr to the length of the wide string
                   if !string_length_ptr.is_null() {
                       unsafe {
                           *string_length_ptr = os_string.len() as i16;
                       }
                   }

                   // Copy the wide string into the memory pointed to by character_attribute_ptr
                   unsafe {
                       ptr::copy_nonoverlapping(
                           os_string.as_ptr() as *const c_void,
                           character_attribute_ptr,
                           os_string.len() * 2, // Each character is 2 bytes in UTF-16
                       );
                   }
               } else {
                   // Handle buffer too small error
               }
           }
       }
       Desc::LiteralPrefix => {}
       Desc::LiteralSuffix => {}
       Desc::LocalTypeName => {}
       Desc::MaximumScale => {}
       Desc::MinimumScale => {}
       Desc::NumPrecRadix => {}
       Desc::ParameterType => {}
       Desc::RowsProcessedPtr => {}
       Desc::RowVer => {}
       Desc::SchemaName => {}
       Desc::Searchable => {}
       Desc::TypeName => {}
       Desc::TableName => {}
       Desc::Unsigned => {}
       Desc::Updatable => {}
   }

   SqlReturn::SUCCESS
*/
