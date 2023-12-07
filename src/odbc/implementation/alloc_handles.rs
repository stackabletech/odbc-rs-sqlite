use odbc_sys::AttrOdbcVersion;
use rusqlite::{Connection, Row, Rows, Statement};

#[derive(Debug)]
pub struct EnvironmentHandle {
    pub odbc_version: AttrOdbcVersion,
    pub output_nts: bool,
}

impl Default for EnvironmentHandle {
    fn default() -> Self {
        EnvironmentHandle {
            odbc_version: AttrOdbcVersion::Odbc3,
            output_nts: true,
        }
    }
}

#[derive(Debug)]
pub struct ConnectionHandle {
    pub sqlite_connection: Option<Connection>,
}

pub struct StatementHandle<'a> {
    pub sqlite_connection: &'a Connection, // TODO: Make it a reference to the ConnectionHandle instead
    pub statement: Option<Statement<'a>>,
    pub rows: Option<Rows<'a>>,
    pub row: Option<&'a Row<'a>>,
}

pub(crate) fn impl_allocate_environment_handle() -> EnvironmentHandle {
    EnvironmentHandle::default()
}

pub(crate) fn impl_allocate_dbc_handle(_env_handle: &mut EnvironmentHandle) -> ConnectionHandle {
    ConnectionHandle {
        sqlite_connection: None,
    }
}

pub(crate) fn allocate_stmt_handle(connection_handle: &mut ConnectionHandle) -> StatementHandle {
    let connection_ref = connection_handle.sqlite_connection.as_ref().unwrap();
    StatementHandle {
        sqlite_connection: connection_ref,
        statement: None,
        rows: None,
        row: None,
    }
}
