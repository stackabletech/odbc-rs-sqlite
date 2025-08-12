//!
//! https://learn.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function
//!
//! ```c
//! SQLRETURN SQLDriverConnect(
//!      SQLHDBC         ConnectionHandle,
//!      SQLHWND         WindowHandle,
//!      SQLCHAR *       InConnectionString,
//!      SQLSMALLINT     StringLength1,
//!      SQLCHAR *       OutConnectionString,
//!      SQLSMALLINT     BufferLength,
//!      SQLSMALLINT *   StringLength2Ptr,
//!      SQLUSMALLINT    DriverCompletion);
//! ```

use crate::odbc::implementation::alloc_handles::ConnectionHandle;
use crate::odbc::implementation::connect::impl_connect;
use crate::odbc::utils::{get_from_wrapper, maybe_utf16_to_string};
use odbc_sys::{HandleType, SmallInt, SqlReturn, USmallInt, WChar};
use std::ffi::c_void;
use tracing::{debug, error, info};

/// SQLDriverConnect establishes connections to a driver and a data source using a connection string
///
/// This is the primary connection method used by most ODBC applications.
/// It's more flexible than SQLConnect as it accepts a full connection string.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLDriverConnectW(
    connection_handle: *mut c_void,
    _window_handle: *mut c_void, // HWND - unused in our case as we won't create any dialogs
    in_connection_string: *const WChar,
    string_length1: SmallInt,
    out_connection_string: *mut WChar,
    buffer_length: SmallInt,
    string_length2_ptr: *mut SmallInt,
    driver_completion: USmallInt,
) -> SqlReturn {
    info!(
        "connection_string_length={}, buffer_length={}, driver_completion={}",
        string_length1, buffer_length, driver_completion
    );

    if connection_handle.is_null() {
        error!("SQLDriverConnectW ERROR: connection_handle is null, can't set error details");
        return SqlReturn::INVALID_HANDLE;
    }

    let connection_handle: &mut ConnectionHandle =
        match get_from_wrapper(&HandleType::Dbc, connection_handle) {
            Ok(conn) => conn,
            Err(e) => {
                error!("Error getting connection handle {}", e);
                return SqlReturn::ERROR;
            }
        };

    // Parse the connection string
    let connection_string = match maybe_utf16_to_string(in_connection_string, string_length1) {
        Some(result) => result,
        None => {
            error!("Failed to convert connection string");
            return SqlReturn::ERROR;
        }
    };

    debug!("Connection string: {}", connection_string);

    // TODO: Parse connection string properly (DSN=..., Database=..., etc.)
    // For now, just extract database path from a simple connection string

    // Basic connection string parsing - look for Database= parameter
    let database_path = if connection_string.contains("Database=") {
        // Extract database path from connection string
        connection_string
            .split(';')
            .find(|part| part.starts_with("Database="))
            .and_then(|part| part.strip_prefix("Database="))
            .map(|path| path.trim().to_string())
    } else {
        // No database specified - this is an error
        None
    };

    match database_path {
        Some(db_path) => {
            info!("Connecting to database: {}", db_path);

            // Use existing connection logic
            impl_connect(connection_handle, db_path, None, None);

            // TODO: Copy connection string to output buffer if provided
            if !out_connection_string.is_null() && buffer_length > 0 {
                // For now, just indicate that we're not filling the output buffer
                if !string_length2_ptr.is_null() {
                    unsafe {
                        *string_length2_ptr = 0;
                    }
                }
            }

            info!("Connection established");
            SqlReturn::SUCCESS
        }
        None => {
            error!("Missing required 'Database=' parameter in connection string");
            SqlReturn::ERROR
        }
    }
}
