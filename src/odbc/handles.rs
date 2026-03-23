use odbc_sys::{AttrOdbcVersion, CDataType, InfoType, InfoTypeType};
use std::sync::{Arc, OnceLock};

// ─── Traits ──────────────────────────────────────────────────────────────────

/// Abstracts a database connection. All DB-specific logic lives behind this trait.
pub trait DbConnection: Send + Sync {
    /// Prepare and eagerly execute a SQL query, returning the full result set.
    fn prepare_statement(&self, sql: &str) -> Result<Box<dyn ActiveStatement>, String>;

    /// Return an ODBC-spec 1-column result set of table names.
    fn get_tables(&self) -> Result<Box<dyn ActiveStatement>, String>;

    /// Return an ODBC-spec 18-column result set of column metadata for a table.
    fn get_columns(&self, table_name: &str) -> Result<Box<dyn ActiveStatement>, String>;

    /// Return driver/data-source information for `SQLGetInfo`.
    fn get_info(&self, info_type: InfoType) -> Option<InfoTypeType>;
}

/// A fully-executed statement whose results are ready to iterate.
pub trait ActiveStatement: Send + Sync {
    fn column_count(&self) -> usize;
    fn column_name(&self, index: usize) -> Result<String, String>;
    /// No-op: execution happened eagerly at prepare time.
    fn execute(&mut self) -> Result<(), String>;
    fn fetch_next_row(&mut self) -> Result<bool, String>;
    fn get_data(&self, col_index: usize, target_type: CDataType) -> Result<String, String>;
    fn row_changes(&self) -> u64;
}

/// Creates `DbConnection` instances from a database path.
/// Register one implementation at startup via `register_factory`.
pub trait DbConnectionFactory: Send + Sync {
    fn create(&self, database: &str) -> Result<Arc<dyn DbConnection>, String>;
}

// ─── Global factory registry ─────────────────────────────────────────────────

static FACTORY: OnceLock<Box<dyn DbConnectionFactory>> = OnceLock::new();

/// Register the database-specific factory. Must be called before any
/// `SQLConnectW` / `SQLDriverConnectW` call. Idempotent (subsequent calls are ignored).
pub fn register_factory(factory: Box<dyn DbConnectionFactory>) {
    let _ = FACTORY.set(factory);
}

/// Retrieve the registered factory. Panics if `register_factory` was never called.
pub(crate) fn factory() -> &'static dyn DbConnectionFactory {
    FACTORY
        .get()
        .expect("no DbConnectionFactory registered; call register_factory first")
        .as_ref()
}

// ─── Handle types ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct EnvironmentHandle {
    pub odbc_version: AttrOdbcVersion,
    pub _output_nts: bool,
}

impl Default for EnvironmentHandle {
    fn default() -> Self {
        EnvironmentHandle {
            odbc_version: AttrOdbcVersion::Odbc3,
            _output_nts: true,
        }
    }
}

impl EnvironmentHandle {
    pub fn odbc_version(&self) -> AttrOdbcVersion {
        self.odbc_version
    }

    pub fn set_odbc_version(&mut self, version: AttrOdbcVersion) {
        self.odbc_version = version;
    }
}

pub struct ConnectionHandle {
    pub connection: Option<Arc<dyn DbConnection>>,
}

impl ConnectionHandle {
    pub fn allocate_stmt_handle(&self) -> Option<StatementHandle> {
        let connection = self.connection.as_ref()?.clone();
        Some(StatementHandle {
            connection,
            active_statement: None,
        })
    }
}

pub struct StatementHandle {
    pub connection: Arc<dyn DbConnection>,
    pub active_statement: Option<Box<dyn ActiveStatement>>,
}
