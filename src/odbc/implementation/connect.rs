use crate::odbc::implementation::alloc_handles::ConnectionHandle;
use crate::odbc::utils::get_private_profile_string;
use rusqlite::{Connection, OpenFlags};
use tracing::{error, info};

/// Connect via DSN name — looks up Database from ODBC configuration.
/// Used by SQLConnectW.
pub(crate) fn impl_connect(
    connection_handle: &mut ConnectionHandle,
    server_name: String,
    _user_name: Option<String>,
    _authentication: Option<String>,
) {
    let database = match get_private_profile_string(&server_name, "Database", "odbc.ini", 1024) {
        Ok(Some(dsn)) => dsn,
        Ok(None) => {
            error!("Error: Database setting not found");
            "TODO".to_string()
        }
        Err(e) => {
            error!("Error: Database setting not found: {}", e);
            "TODO".to_string()
        }
    };
    info!("Opening [{}] for DSN [{}]", database, server_name);
    impl_connect_to_database(connection_handle, database);
}

/// Connect directly to a database file path.
/// Used by SQLDriverConnectW after resolving the database path.
pub(crate) fn impl_connect_to_database(
    connection_handle: &mut ConnectionHandle,
    database_path: String,
) {
    let conn = match Connection::open_with_flags(
        &database_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    ) {
        Ok(conn) => conn,
        Err(e) => {
            error!("Connection failed: {}", e);
            return;
        }
    };

    connection_handle.sqlite_connection = Some(conn);
}
