use crate::odbc::implementation::alloc_handles::StatementHandle;
use odbc_sys::CDataType;
use rusqlite::types::ValueRef;
use tracing::{error, warn};

pub(crate) fn impl_getdata(
    statement_handle: &StatementHandle,
    target_type: &CDataType,
    col_or_param: u16,
) -> String {
    let row = match &statement_handle.row {
        Some(row) => row,
        None => {
            error!("No current row available");
            return "ERROR".to_string();
        }
    };

    let col_index = (col_or_param - 1) as usize;

    // Get the raw value from SQLite
    let value_ref: ValueRef = match row.get_ref(col_index) {
        Ok(value_ref) => value_ref,
        Err(err) => {
            error!("Failed to get column {}: {}", col_index, err);
            return "ERROR".to_string();
        }
    };

    // Convert the SQLite value to the requested ODBC type
    match target_type {
        CDataType::Char => {
            // Convert any SQLite type to string representation
            match value_ref {
                ValueRef::Null => "NULL".to_string(),
                ValueRef::Integer(i) => i.to_string(),
                ValueRef::Real(f) => f.to_string(),
                ValueRef::Text(s) => match std::str::from_utf8(s) {
                    Ok(text) => text.to_string(),
                    Err(err) => {
                        error!("UTF-8 conversion failed: {}", err);
                        "ERROR".to_string()
                    }
                },
                ValueRef::Blob(b) => {
                    // Convert blob to hex string representation
                    b.iter()
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<String>()
                }
            }
        }
        CDataType::SLong => {
            // Convert to integer
            match value_ref {
                ValueRef::Integer(i) => i.to_string(),
                ValueRef::Real(f) => (f as i64).to_string(),
                ValueRef::Text(s) => match std::str::from_utf8(s) {
                    Ok(text) => match text.parse::<i64>() {
                        Ok(i) => i.to_string(),
                        Err(_) => "0".to_string(),
                    },
                    Err(_) => "0".to_string(),
                },
                ValueRef::Null => "0".to_string(),
                ValueRef::Blob(_) => "0".to_string(),
            }
        }
        CDataType::Double => {
            // Convert to float
            match value_ref {
                ValueRef::Real(f) => f.to_string(),
                ValueRef::Integer(i) => (i as f64).to_string(),
                ValueRef::Text(s) => match std::str::from_utf8(s) {
                    Ok(text) => match text.parse::<f64>() {
                        Ok(f) => f.to_string(),
                        Err(_) => "0.0".to_string(),
                    },
                    Err(_) => "0.0".to_string(),
                },
                ValueRef::Null => "0.0".to_string(),
                ValueRef::Blob(_) => "0.0".to_string(),
            }
        }
        _ => {
            // For unsupported types, convert to string as fallback
            warn!(
                "Unsupported target type {:?}, converting to string",
                target_type
            );
            match value_ref {
                ValueRef::Null => "NULL".to_string(),
                ValueRef::Integer(i) => i.to_string(),
                ValueRef::Real(f) => f.to_string(),
                ValueRef::Text(s) => match std::str::from_utf8(s) {
                    Ok(text) => text.to_string(),
                    Err(err) => {
                        error!("UTF-8 conversion failed: {}", err);
                        "ERROR".to_string()
                    }
                },
                ValueRef::Blob(b) => b
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<String>(),
            }
        }
    }
}
