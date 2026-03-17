//!
//! <https://learn.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function?view=sql-server-ver16>
//!
//! ```c
//! SQLRETURN SQLConnect(
//!      SQLHDBC        ConnectionHandle,
//!      SQLCHAR *      ServerName,
//!      SQLSMALLINT    NameLength1,
//!      SQLCHAR *      UserName,
//!      SQLSMALLINT    NameLength2,
//!      SQLCHAR *      Authentication,
//!      SQLSMALLINT    NameLength3);
//! ```

use crate::odbc::handles::{ConnectionHandle, factory};
use crate::odbc::utils::{get_from_wrapper, get_private_profile_string, maybe_utf16_to_string};
use odbc_sys::{HandleType, SmallInt, SqlReturn, WChar};
use std::ffi::c_void;
use tracing::{error, info};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLConnectW(
    connection_handle: *mut c_void,
    server_name: *const WChar,
    server_name_length: SmallInt,
    user_name: *const WChar,
    user_name_length: SmallInt,
    authentication: *const WChar,
    authentication_length: SmallInt,
) -> SqlReturn {
    info!("SQLConnectW");

    if connection_handle.is_null() {
        error!("connection_handle is null, can't set error details");
        return SqlReturn::INVALID_HANDLE;
    }

    let connection_handle: &mut ConnectionHandle =
        match get_from_wrapper(&HandleType::Dbc, connection_handle) {
            Ok(env) => env,
            Err(e) => {
                error!("Failed to get connection handle: {}", e);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    // TODO: This can't be null (not sure if i meant only server_name or all of them)
    let server_name = match maybe_utf16_to_string(server_name, server_name_length) {
        Some(result) => result,
        None => {
            error!("Error converting serverName");
            return SqlReturn::ERROR;
        }
    };
    let user_name = maybe_utf16_to_string(user_name, user_name_length);
    let authentication = maybe_utf16_to_string(authentication, authentication_length);

    let database = match get_private_profile_string(&server_name, "Database", "odbc.ini", 1024) {
        Ok(Some(db)) => db,
        Ok(None) => {
            error!("Database setting not found for DSN '{}'", server_name);
            return SqlReturn::ERROR;
        }
        Err(e) => {
            error!("Failed to look up DSN '{}': {}", server_name, e);
            return SqlReturn::ERROR;
        }
    };

    let _ = (user_name, authentication); // unused by SQLite

    match factory().create_from_path(&database) {
        Ok(conn) => {
            connection_handle.connection = Some(conn);
            SqlReturn::SUCCESS
        }
        Err(e) => {
            error!("Failed to open database '{}': {}", database, e);
            SqlReturn::ERROR
        }
    }
}
