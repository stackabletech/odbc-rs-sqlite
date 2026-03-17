use crate::odbc::implementation::alloc_handles::StatementHandle;
use tracing::error;

pub(crate) fn impl_get_columns<'a>(
    statement_handle: &mut StatementHandle<'a>,
    table_name: &str,
) -> Result<(), String> {
    // Validate table name before interpolating into SQL.
    // pragma_table_info does not accept bound parameters, so we must
    // ensure the name contains only safe characters.
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(format!("Invalid table name: '{table_name}'"));
    }

    // pragma_table_info('<name>') is a SQLite table-valued function that returns
    // one row per column: cid, name, type, notnull, dflt_value, pk.
    //
    // We reshape the output into the 18-column result set required by the ODBC
    // spec for SQLColumns. Columns that SQLite cannot provide are returned as NULL.
    //
    // ODBC SQL type codes used here:
    //   -7 = SQL_BIT, 2 = SQL_NUMERIC, 4 = SQL_INTEGER,
    //    8 = SQL_DOUBLE, -4 = SQL_LONGVARBINARY, 12 = SQL_VARCHAR (default)
    // Column names from pragma_table_info that are SQLite reserved words must be
    // double-quoted: "notnull" and "type". Others ("name", "cid", "dflt_value") are
    // quoted for consistency.
    let sql = format!(
        r#"SELECT
            NULL                                                AS TABLE_CAT,
            NULL                                                AS TABLE_SCHEM,
            '{table_name}'                                               AS TABLE_NAME,
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

    let stmt = statement_handle
        .sqlite_connection
        .prepare(&sql)
        .map_err(|e| {
            error!("Failed to prepare column query for '{}': {}", table_name, e);
            e.to_string()
        })?;

    statement_handle.statement = Some(stmt);

    let rows = statement_handle
        .statement
        .as_mut()
        .unwrap()
        .query([])
        .map_err(|e| {
            error!("Failed to execute column query for '{}': {}", table_name, e);
            e.to_string()
        })?;

    // Safety: `rows` borrows from `statement_handle.statement` which has lifetime `'a`.
    // The borrow checker infers a shorter lifetime for the reference, but the data lives
    // for `'a`. We transmute here to express that — the same pattern the rest of this
    // codebase relies on implicitly via `get_from_wrapper`.
    let rows: rusqlite::Rows<'a> = unsafe { std::mem::transmute(rows) };
    statement_handle.rows = Some(rows);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::odbc::implementation::alloc_handles::StatementHandle;
    use rusqlite::Connection;

    fn make_test_db() -> Connection {
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
        conn
    }

    #[test]
    fn result_set_has_18_columns() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        impl_get_columns(&mut handle, "widgets").unwrap();

        let col_count = handle.statement.as_ref().unwrap().column_count();
        assert_eq!(col_count, 18);
    }

    #[test]
    fn result_set_has_one_row_per_table_column() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        impl_get_columns(&mut handle, "widgets").unwrap();

        let mut count = 0;
        let rows = handle.rows.as_mut().unwrap();
        while rows.next().unwrap().is_some() {
            count += 1;
        }
        assert_eq!(count, 4); // widgets has 4 columns
    }

    #[test]
    fn rows_contain_correct_table_and_column_names() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        impl_get_columns(&mut handle, "widgets").unwrap();

        let rows = handle.rows.as_mut().unwrap();
        let row = rows.next().unwrap().unwrap();

        // Column 3 (index 2) = TABLE_NAME, column 4 (index 3) = COLUMN_NAME
        let table_name: String = row.get(2).unwrap();
        let column_name: String = row.get(3).unwrap();

        assert_eq!(table_name, "widgets");
        assert_eq!(column_name, "widget_id");
    }

    #[test]
    fn ordinal_position_is_one_based() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        impl_get_columns(&mut handle, "widgets").unwrap();

        let rows = handle.rows.as_mut().unwrap();
        let row = rows.next().unwrap().unwrap();

        // Column 17 (index 16) = ORDINAL_POSITION
        let ordinal: i64 = row.get(16).unwrap();
        assert_eq!(ordinal, 1);
    }

    #[test]
    fn not_null_column_has_nullable_zero() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        impl_get_columns(&mut handle, "widgets").unwrap();

        let rows = handle.rows.as_mut().unwrap();

        // widget_id is PRIMARY KEY (implicitly NOT NULL); skip to name (NOT NULL)
        rows.next().unwrap(); // widget_id
        let row = rows.next().unwrap().unwrap(); // name VARCHAR(50) NOT NULL

        // Column 11 (index 10) = NULLABLE: 0 = not nullable
        let nullable: i64 = row.get(10).unwrap();
        assert_eq!(nullable, 0);
    }

    #[test]
    fn nullable_column_has_nullable_one() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        impl_get_columns(&mut handle, "widgets").unwrap();

        let rows = handle.rows.as_mut().unwrap();
        rows.next().unwrap(); // widget_id
        rows.next().unwrap(); // name
        let row = rows.next().unwrap().unwrap(); // weight REAL (nullable)

        // Column 11 (index 10) = NULLABLE: 1 = nullable
        let nullable: i64 = row.get(10).unwrap();
        assert_eq!(nullable, 1);
    }

    #[test]
    fn rejects_table_name_with_special_characters() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        let result = impl_get_columns(&mut handle, "widgets; DROP TABLE widgets");
        assert!(result.is_err());
    }
}
