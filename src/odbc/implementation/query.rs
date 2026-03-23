use crate::odbc::handles::{ActiveStatement, DbConnection, DbConnectionFactory};
use odbc_sys::{CDataType, InfoType, InfoTypeType};
use rusqlite::OpenFlags;
use rusqlite::types::Value;
use std::sync::{Arc, Mutex};
use tracing::error;

// ─── Factory ─────────────────────────────────────────────────────────────────

pub(crate) struct SqliteDbConnectionFactory;

impl DbConnectionFactory for SqliteDbConnectionFactory {
    fn create(&self, database: &str) -> Result<Arc<dyn DbConnection>, String> {
        let conn = rusqlite::Connection::open_with_flags(
            database,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| e.to_string())?;
        Ok(Arc::new(SqliteDbConnection::new(conn)))
    }
}

// ─── Connection ──────────────────────────────────────────────────────────────

pub(crate) struct SqliteDbConnection {
    connection: Arc<Mutex<rusqlite::Connection>>,
}

impl SqliteDbConnection {
    pub(crate) fn new(connection: rusqlite::Connection) -> Self {
        SqliteDbConnection {
            connection: Arc::new(Mutex::new(connection)),
        }
    }
}

impl DbConnection for SqliteDbConnection {
    fn prepare_statement(&self, sql: &str) -> Result<Box<dyn ActiveStatement>, String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;

        let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
        let column_count = column_names.len();

        // Capture readonly flag before query() consumes the statement.
        let is_readonly = stmt.readonly();

        let mut rows_data: Vec<Vec<Value>> = Vec::new();
        let mut raw_rows = stmt.query([]).map_err(|e| e.to_string())?;
        while let Some(row) = raw_rows.next().map_err(|e| e.to_string())? {
            let row_data: Vec<Value> = (0..column_count)
                .map(|i| row.get::<_, Value>(i).unwrap_or(Value::Null))
                .collect();
            rows_data.push(row_data);
        }

        // For read-only queries (SELECT), changes() reflects prior DML — not this statement.
        let changes = if is_readonly { 0 } else { conn.changes() };

        Ok(Box::new(SqliteStatement {
            column_names,
            rows: rows_data,
            current_row: None,
            changes,
        }))
    }

    fn get_tables(&self) -> Result<Box<dyn ActiveStatement>, String> {
        self.prepare_statement("SELECT name FROM sqlite_master WHERE type='table'")
    }

    fn get_columns(&self, table_name: &str) -> Result<Box<dyn ActiveStatement>, String> {
        // Validate table name before interpolating into SQL.
        // pragma_table_info does not accept bound parameters.
        if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!("Invalid table name: '{table_name}'"));
        }

        // Reshape pragma_table_info output into the ODBC 18-column SQLColumns result set.
        // ODBC SQL type codes: -7=BIT, 2=NUMERIC, 4=INTEGER, 8=DOUBLE, -4=LONGVARBINARY, 12=VARCHAR
        let sql = format!(
            r#"SELECT
                NULL                                                AS TABLE_CAT,
                NULL                                                AS TABLE_SCHEM,
                '{table_name}'                                      AS TABLE_NAME,
                "name"                                              AS COLUMN_NAME,
                CASE
                    WHEN upper("type") LIKE '%INT%'                 THEN 4
                    WHEN upper("type") LIKE '%REAL%'
                      OR upper("type") LIKE '%FLOAT%'
                      OR upper("type") LIKE '%DOUBLE%'              THEN 8
                    WHEN upper("type") LIKE '%BLOB%'                THEN -4
                    WHEN upper("type") LIKE '%BOOL%'                THEN -7
                    WHEN upper("type") LIKE '%NUMERIC%'
                      OR upper("type") LIKE '%DECIMAL%'             THEN 2
                    ELSE 12
                END                                                 AS DATA_TYPE,
                CASE WHEN "type" = '' THEN 'TEXT' ELSE "type" END   AS TYPE_NAME,
                NULL                                                AS COLUMN_SIZE,
                NULL                                                AS BUFFER_LENGTH,
                NULL                                                AS DECIMAL_DIGITS,
                NULL                                                AS NUM_PREC_RADIX,
                CASE WHEN "notnull" = 1 THEN 0 ELSE 1 END           AS NULLABLE,
                NULL                                                AS REMARKS,
                "dflt_value"                                        AS COLUMN_DEF,
                CASE
                    WHEN upper("type") LIKE '%INT%'                 THEN 4
                    WHEN upper("type") LIKE '%REAL%'
                      OR upper("type") LIKE '%FLOAT%'
                      OR upper("type") LIKE '%DOUBLE%'              THEN 8
                    WHEN upper("type") LIKE '%BLOB%'                THEN -4
                    WHEN upper("type") LIKE '%BOOL%'                THEN -7
                    WHEN upper("type") LIKE '%NUMERIC%'
                      OR upper("type") LIKE '%DECIMAL%'             THEN 2
                    ELSE 12
                END                                                 AS SQL_DATA_TYPE,
                NULL                                                AS SQL_DATETIME_SUB,
                NULL                                                AS CHAR_OCTET_LENGTH,
                "cid" + 1                                           AS ORDINAL_POSITION,
                CASE WHEN "notnull" = 1 THEN 'NO' ELSE 'YES' END    AS IS_NULLABLE
            FROM pragma_table_info('{table_name}')"#
        );

        self.prepare_statement(&sql).map_err(|e| {
            error!("Failed to prepare column query for '{}': {}", table_name, e);
            e
        })
    }

    fn get_info(&self, info_type: InfoType) -> Option<InfoTypeType> {
        match info_type {
            InfoType::ActiveEnvironments => Some(InfoTypeType::SqlUSmallInt(0)),
            InfoType::UserName => Some(InfoTypeType::String("foo".to_string())),
            InfoType::MaxConcurrentActivities => Some(InfoTypeType::SqlUSmallInt(1)),
            InfoType::ScrollOptions => Some(InfoTypeType::SqlUInteger(1)),
            _ => None,
        }
    }
}

// ─── Statement ───────────────────────────────────────────────────────────────

pub(crate) struct SqliteStatement {
    column_names: Vec<String>,
    rows: Vec<Vec<Value>>,
    current_row: Option<usize>,
    changes: u64,
}

impl ActiveStatement for SqliteStatement {
    fn column_count(&self) -> usize {
        self.column_names.len()
    }

    fn column_name(&self, index: usize) -> Result<String, String> {
        self.column_names
            .get(index)
            .cloned()
            .ok_or_else(|| format!("Column index {index} out of range"))
    }

    fn execute(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn fetch_next_row(&mut self) -> Result<bool, String> {
        let next = match self.current_row {
            None => 0,
            Some(i) => i + 1,
        };
        if next < self.rows.len() {
            self.current_row = Some(next);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_data(&self, col_index: usize, target_type: CDataType) -> Result<String, String> {
        let row_index = self
            .current_row
            .ok_or("No current row; call SQLFetch first")?;
        let row = self.rows.get(row_index).ok_or("Row index out of range")?;
        let value = row
            .get(col_index)
            .ok_or_else(|| format!("Column index {col_index} out of range"))?;

        let result = match target_type {
            CDataType::Char => value_to_string(value),
            CDataType::SLong => value_to_slong(value),
            CDataType::Double => value_to_double(value),
            _ => value_to_string(value),
        };
        Ok(result)
    }

    fn row_changes(&self) -> u64 {
        self.changes
    }
}

// ─── Value converters ────────────────────────────────────────────────────────

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Null => "NULL".to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Real(f) => f.to_string(),
        Value::Text(s) => s.clone(),
        Value::Blob(b) => b.iter().map(|byte| format!("{byte:02x}")).collect(),
    }
}

fn value_to_slong(value: &Value) -> String {
    match value {
        Value::Integer(i) => i.to_string(),
        Value::Real(f) => (*f as i64).to_string(),
        Value::Text(s) => s
            .parse::<i64>()
            .map(|i| i.to_string())
            .unwrap_or_else(|_| "0".to_string()),
        Value::Null => "0".to_string(),
        Value::Blob(_) => "0".to_string(),
    }
}

fn value_to_double(value: &Value) -> String {
    match value {
        Value::Real(f) => f.to_string(),
        Value::Integer(i) => (*i as f64).to_string(),
        Value::Text(s) => s
            .parse::<f64>()
            .map(|f| f.to_string())
            .unwrap_or_else(|_| "0.0".to_string()),
        Value::Null => "0.0".to_string(),
        Value::Blob(_) => "0.0".to_string(),
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::odbc::handles::DbConnection;
    use odbc_sys::CDataType;
    use rusqlite::Connection;

    fn make_test_db() -> SqliteDbConnection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE users (
                id    INTEGER PRIMARY KEY,
                name  TEXT    NOT NULL,
                score REAL
            );
            INSERT INTO users VALUES (1, 'Alice', 9.5);
            INSERT INTO users VALUES (2, 'Bob',   7.0);",
        )
        .unwrap();
        SqliteDbConnection::new(conn)
    }

    fn make_widgets_db() -> SqliteDbConnection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE widgets (
                widget_id INTEGER PRIMARY KEY,
                name      VARCHAR(50) NOT NULL,
                weight    REAL,
                notes     TEXT
            );",
        )
        .unwrap();
        SqliteDbConnection::new(conn)
    }

    // ── prepare_statement ────────────────────────────────────────────────────

    #[test]
    fn prepare_returns_correct_column_count() {
        let db = make_test_db();
        let stmt = db
            .prepare_statement("SELECT id, name, score FROM users")
            .unwrap();
        assert_eq!(stmt.column_count(), 3);
    }

    #[test]
    fn prepare_returns_correct_column_names() {
        let db = make_test_db();
        let stmt = db
            .prepare_statement("SELECT id, name, score FROM users")
            .unwrap();
        assert_eq!(stmt.column_name(0).unwrap(), "id");
        assert_eq!(stmt.column_name(1).unwrap(), "name");
        assert_eq!(stmt.column_name(2).unwrap(), "score");
    }

    #[test]
    fn column_name_out_of_range_returns_error() {
        let db = make_test_db();
        let stmt = db.prepare_statement("SELECT id FROM users").unwrap();
        assert!(stmt.column_name(99).is_err());
    }

    #[test]
    fn fetch_next_row_iterates_through_all_rows() {
        let db = make_test_db();
        let mut stmt = db.prepare_statement("SELECT id FROM users").unwrap();
        assert_eq!(stmt.fetch_next_row().unwrap(), true);
        assert_eq!(stmt.fetch_next_row().unwrap(), true);
        assert_eq!(stmt.fetch_next_row().unwrap(), false);
    }

    #[test]
    fn fetch_next_row_returns_false_immediately_on_empty_result() {
        let db = make_test_db();
        let mut stmt = db
            .prepare_statement("SELECT id FROM users WHERE id = 999")
            .unwrap();
        assert_eq!(stmt.fetch_next_row().unwrap(), false);
    }

    #[test]
    fn get_data_returns_correct_string_values() {
        let db = make_test_db();
        let mut stmt = db
            .prepare_statement("SELECT id, name, score FROM users ORDER BY id")
            .unwrap();
        stmt.fetch_next_row().unwrap();
        assert_eq!(stmt.get_data(0, CDataType::Char).unwrap(), "1");
        assert_eq!(stmt.get_data(1, CDataType::Char).unwrap(), "Alice");
    }

    #[test]
    fn get_data_slong_returns_integer_string() {
        let db = make_test_db();
        let mut stmt = db
            .prepare_statement("SELECT id FROM users ORDER BY id")
            .unwrap();
        stmt.fetch_next_row().unwrap();
        assert_eq!(stmt.get_data(0, CDataType::SLong).unwrap(), "1");
    }

    #[test]
    fn get_data_double_returns_float_string() {
        let db = make_test_db();
        let mut stmt = db
            .prepare_statement("SELECT score FROM users ORDER BY id")
            .unwrap();
        stmt.fetch_next_row().unwrap();
        assert_eq!(stmt.get_data(0, CDataType::Double).unwrap(), "9.5");
    }

    #[test]
    fn execute_is_a_noop_and_rows_remain_iterable() {
        let db = make_test_db();
        let mut stmt = db.prepare_statement("SELECT id FROM users").unwrap();
        stmt.execute().unwrap();
        assert_eq!(stmt.fetch_next_row().unwrap(), true);
    }

    #[test]
    fn non_select_has_zero_rows_and_nonzero_changes() {
        let db = make_test_db();
        let stmt = db
            .prepare_statement("INSERT INTO users VALUES (3, 'Carol', 8.0)")
            .unwrap();
        assert_eq!(stmt.column_count(), 0);
        assert_eq!(stmt.row_changes(), 1);
    }

    #[test]
    fn select_has_zero_changes() {
        let db = make_test_db();
        let stmt = db.prepare_statement("SELECT id FROM users").unwrap();
        assert_eq!(stmt.row_changes(), 0);
    }

    // ── get_tables ───────────────────────────────────────────────────────────

    #[test]
    fn get_tables_result_set_is_populated() {
        let db = make_test_db();
        assert!(db.get_tables().is_ok());
    }

    #[test]
    fn get_tables_returns_at_least_one_row_per_table() {
        let db = make_test_db();
        let mut stmt = db.get_tables().unwrap();
        let mut count = 0;
        while stmt.fetch_next_row().unwrap() {
            count += 1;
        }
        assert!(count >= 1);
    }

    #[test]
    fn get_tables_row_contains_nonempty_name() {
        let db = make_test_db();
        let mut stmt = db.get_tables().unwrap();
        stmt.fetch_next_row().unwrap();
        let name = stmt.get_data(0, CDataType::Char).unwrap();
        assert!(!name.is_empty());
    }

    // ── get_columns ──────────────────────────────────────────────────────────

    #[test]
    fn get_columns_result_set_has_18_columns() {
        let db = make_widgets_db();
        let stmt = db.get_columns("widgets").unwrap();
        assert_eq!(stmt.column_count(), 18);
    }

    #[test]
    fn get_columns_has_one_row_per_table_column() {
        let db = make_widgets_db();
        let mut stmt = db.get_columns("widgets").unwrap();
        let mut count = 0;
        while stmt.fetch_next_row().unwrap() {
            count += 1;
        }
        assert_eq!(count, 4);
    }

    #[test]
    fn get_columns_row_contains_table_and_column_name() {
        let db = make_widgets_db();
        let mut stmt = db.get_columns("widgets").unwrap();
        stmt.fetch_next_row().unwrap();
        assert_eq!(stmt.get_data(2, CDataType::Char).unwrap(), "widgets");
        assert_eq!(stmt.get_data(3, CDataType::Char).unwrap(), "widget_id");
    }

    #[test]
    fn get_columns_ordinal_position_is_one_based() {
        let db = make_widgets_db();
        let mut stmt = db.get_columns("widgets").unwrap();
        stmt.fetch_next_row().unwrap();
        assert_eq!(stmt.get_data(16, CDataType::SLong).unwrap(), "1");
    }

    #[test]
    fn get_columns_not_null_has_nullable_zero() {
        let db = make_widgets_db();
        let mut stmt = db.get_columns("widgets").unwrap();
        stmt.fetch_next_row().unwrap(); // widget_id
        stmt.fetch_next_row().unwrap(); // name NOT NULL
        assert_eq!(stmt.get_data(10, CDataType::SLong).unwrap(), "0");
    }

    #[test]
    fn get_columns_nullable_has_nullable_one() {
        let db = make_widgets_db();
        let mut stmt = db.get_columns("widgets").unwrap();
        stmt.fetch_next_row().unwrap(); // widget_id
        stmt.fetch_next_row().unwrap(); // name
        stmt.fetch_next_row().unwrap(); // weight REAL (nullable)
        assert_eq!(stmt.get_data(10, CDataType::SLong).unwrap(), "1");
    }

    #[test]
    fn get_columns_rejects_invalid_table_name() {
        let db = make_widgets_db();
        assert!(db.get_columns("widgets; DROP TABLE widgets").is_err());
    }
}
