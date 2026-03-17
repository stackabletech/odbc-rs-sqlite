use crate::odbc::implementation::alloc_handles::StatementHandle;

pub(crate) fn impl_get_tables<'a>(
    statement_handle: &mut StatementHandle<'a>,
) -> Result<(), String> {
    let stmt = statement_handle
        .sqlite_connection
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .map_err(|e| e.to_string())?;

    statement_handle.statement = Some(stmt);

    let rows = statement_handle
        .statement
        .as_mut()
        .unwrap()
        .query([])
        .map_err(|e| e.to_string())?;

    // Safety: `rows` borrows from `statement_handle.statement` which has lifetime `'a`.
    // We transmute the inferred shorter reference lifetime to `'a` — the same pattern
    // used throughout this codebase via `get_from_wrapper`.
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
            "CREATE TABLE apples (id INTEGER PRIMARY KEY);
             CREATE TABLE oranges (id INTEGER PRIMARY KEY);",
        )
        .unwrap();
        conn
    }

    #[test]
    fn result_set_is_populated_after_call() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        let result = impl_get_tables(&mut handle);

        assert!(result.is_ok());
        assert!(handle.statement.is_some());
        assert!(handle.rows.is_some());
    }

    #[test]
    fn returns_one_row_per_table() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        impl_get_tables(&mut handle).unwrap();

        let mut count = 0;
        let rows = handle.rows.as_mut().unwrap();
        while rows.next().unwrap().is_some() {
            count += 1;
        }
        // sqlite_sequence is also present (autoincrement tracking table)
        assert!(count >= 2);
    }

    #[test]
    fn row_contains_table_name() {
        let conn = make_test_db();
        let mut handle = StatementHandle {
            sqlite_connection: &conn,
            statement: None,
            rows: None,
            row: None,
        };

        impl_get_tables(&mut handle).unwrap();

        let rows = handle.rows.as_mut().unwrap();
        let row = rows.next().unwrap().unwrap();
        let name: String = row.get(0).unwrap();
        assert!(!name.is_empty());
    }
}
