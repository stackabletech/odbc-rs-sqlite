use crate::odbc::implementation::alloc_handles::ConnectionHandle;
use crate::odbc::utils::get_private_profile_string;
use rusqlite::{Connection, OpenFlags};

pub(crate) fn impl_connect(
    connection_handle: &mut ConnectionHandle,
    server_name: String,
    _user_name: Option<String>,
    _authentication: Option<String>,
) {
    let database = match get_private_profile_string(&server_name, "Database", "odbc.ini", 1024) {
        Ok(Some(dsn)) => dsn,
        Ok(None) => {
            println!("Error: Database setting not found");
            "TODO".to_string()
        }
        Err(e) => {
            println!("Error: Database setting not found: {}", e);
            "TODO".to_string()
        }
    };
    println!("Opening [{}] for DSN [{}]", database, server_name);
    let conn = match Connection::open_with_flags(
        database,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    ) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Connection failed: {}", e);
            return;
        }
    };

    connection_handle.sqlite_connection = Some(conn);
}
