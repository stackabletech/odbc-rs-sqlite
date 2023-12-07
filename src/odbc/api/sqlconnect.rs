//!
//! https://learn.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function?view=sql-server-ver16
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

use crate::odbc::implementation::alloc_handles::ConnectionHandle;
use crate::odbc::implementation::connect::impl_connect;
use crate::odbc::utils::{get_from_wrapper, maybe_utf16_to_string};
use odbc_sys::{HandleType, SmallInt, SqlReturn, WChar};
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLConnectW(
    connection_handle: *mut c_void,
    server_name: *const WChar,
    server_name_length: SmallInt,
    user_name: *const WChar,
    user_name_length: SmallInt,
    authentication: *const WChar,
    authentication_length: SmallInt,
) -> SqlReturn {
    println!("SQLConnectW INFO");

    if connection_handle.is_null() {
        println!("SQLConnectW ERROR: connection_handle is null, can't set error details");
        return SqlReturn::INVALID_HANDLE;
    }

    let connection_handle: &mut ConnectionHandle =
        match get_from_wrapper(&HandleType::Dbc, connection_handle) {
            Ok(env) => env,
            Err(e) => {
                println!("SQLConnectW ERROR: {}", e);
                return SqlReturn::ERROR;
            }
        };

    // TODO: This can't be null (not sure if i meant only server_name or all of them)
    let server_name = match maybe_utf16_to_string(server_name, server_name_length) {
        Some(result) => result,
        None => {
            println!("SQLConnectW Error converting serverName");
            return SqlReturn::ERROR;
        }
    };
    let user_name = maybe_utf16_to_string(user_name, user_name_length);
    let authentication = maybe_utf16_to_string(authentication, authentication_length);

    impl_connect(connection_handle, server_name, user_name, authentication);

    SqlReturn::SUCCESS
}
