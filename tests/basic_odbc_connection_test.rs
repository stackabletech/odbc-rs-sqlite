#![allow(dead_code)]
use odbc_api::buffers::TextRowSet;
use odbc_api::{
    ColumnDescription, ConnectionOptions, Cursor, DataType, Environment, Nullability,
    ResultSetMetadata,
};
use std::num::NonZeroUsize;
use std::process::Command;

/// Basic ODBC connection test using odbc-api crate
///
/// This test validates that we can establish a basic ODBC connection
/// to our driver through the standard ODBC client stack.

const CONNECTION_STRING: &str = "DSN=test_connection";
//const CONNECTION_STRING: &str = "Driver=/home/andrew/gitrepos/odbc-rs-sqlite/target/debug/libodbc_driver_rs.so;Database=/home/andrew/gitrepos/odbc-rs-sqlite/test_odbc.sqlite";

/// Set up test environment by building driver and configuring ODBC
fn setup_test_environment() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Ensure driver is built and ODBC is configured
    let output = Command::new("./scripts/build-and-setup.sh")
        .arg("debug")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()?;

    if !output.status.success() {
        panic!(
            "Failed to setup test environment: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

#[test]
fn test_odbc_environment_creation() {
    setup_test_environment().expect("Test environment setup failed");

    // Test basic ODBC environment creation
    let _env = Environment::new().expect("Failed to create ODBC environment");
    println!("✅ ODBC environment created successfully");
}

#[test]
fn test_odbc_connection_establishment() {
    //setup_test_environment().expect("Test environment setup failed");

    // Test basic ODBC connection
    let env = Environment::new().expect("Failed to create ODBC environment");

    // The .connect method uses the SQLConnectW method to connect via a DSN provided by the Driver Manager
    //
    // The env.connect_with_connection_string method can be used to connect to a database that is not pre-configured.
    // This calls SQLDriverConnectW instead.

    let connection = env
        .connect("test_connection", "", "", ConnectionOptions::default())
        .expect("Failed to connect to database");

    println!("✅ ODBC connection established successfully");
    drop(connection);
    println!("✅ ODBC connection closed successfully");
}

#[test]
fn test_list_tables() {
    //setup_test_environment().expect("Test environment setup failed");

    // Test basic ODBC connection
    let env = Environment::new().expect("Failed to create ODBC environment");

    // The .connect method uses the SQLConnectW method to connect via a DSN provided by the Driver Manager
    //
    // The env.connect_with_connection_string method can be used to connect to a database that is not pre-configured.
    // This calls SQLDriverConnectW instead.

    let connection = env
        .connect("test_connection", "", "", ConnectionOptions::default())
        .expect("Failed to connect to database");

    {
        // Set all filters to an empty string, to really print all tables
        let mut cursor = connection.tables("", "", "", "").unwrap();

        println!("listing tables");

        let batch_size = 100;
        let mut buffer = TextRowSet::for_cursor(batch_size, &mut cursor, Some(4096)).unwrap();
        let mut row_set_cursor = cursor.bind_buffer(&mut buffer).unwrap();

        while let Some(row_set) = row_set_cursor.fetch().unwrap() {
            for row_index in 0..row_set.num_rows() {
                if row_index != 0 {
                    print!("\n");
                }
                for col_index in 0..row_set.num_cols() {
                    if col_index != 0 {
                        print!(",");
                    }
                    let value = row_set
                        .at_as_str(col_index, row_index)
                        .unwrap()
                        .unwrap_or("NULL");
                    print!("{}", value);
                }
            }
        }

        println!("table listing done");
    }

    println!("✅ ODBC connection established successfully");
    drop(connection);
    println!("✅ ODBC connection closed successfully");
}

#[test]
fn test_multiple_connections() {
    setup_test_environment().expect("Test environment setup failed");

    let env = Environment::new().expect("Failed to create ODBC environment");

    // Test multiple simultaneous connections
    let conn1 = env
        .connect_with_connection_string(CONNECTION_STRING, ConnectionOptions::default())
        .expect("Failed to create first connection");

    let conn2 = env
        .connect_with_connection_string(CONNECTION_STRING, ConnectionOptions::default())
        .expect("Failed to create second connection");

    println!("✅ Multiple connections established successfully");

    // Both connections should work independently
    drop(conn1);
    println!("✅ First connection closed without affecting second");

    drop(conn2);
    println!("✅ Second connection closed successfully");
}

#[test]
fn test_describe_columns() {
    let env = Environment::new().expect("Failed to create ODBC environment");

    let connection = env
        .connect("test_connection", "", "", ConnectionOptions::default())
        .expect("Failed to connect to database");

    // Execute a query so we have a result set with known columns
    let mut cursor = connection
        .execute(
            "SELECT name FROM sqlite_master WHERE type='table'",
            (),
            None,
        )
        .expect("Failed to execute query")
        .expect("Expected a cursor for SELECT statement");

    // Describe column 1 (the "name" column)
    let mut col_desc = ColumnDescription::default();
    cursor
        .describe_col(1, &mut col_desc)
        .expect("Failed to describe column 1");

    let col_name = col_desc
        .name_to_string()
        .expect("Failed to decode column name");
    println!(
        "Column 1 name: '{}', data_type: {:?}, nullable: {:?}",
        col_name, col_desc.data_type, col_desc.nullability
    );

    assert_eq!(col_name, "name");
    assert_eq!(
        col_desc.data_type,
        DataType::Varchar {
            length: NonZeroUsize::new(255)
        }
    );
    assert_eq!(col_desc.nullability, Nullability::Nullable);

    println!("describe_columns test passed");
}

#[test]
fn test_invalid_connection_string() {
    setup_test_environment().expect("Test environment setup failed");

    let env = Environment::new().expect("Failed to create ODBC environment");

    // Test invalid connection string - this should fail gracefully
    let result =
        env.connect_with_connection_string("DSN=nonexistent_dsn", ConnectionOptions::default());

    match result {
        Ok(_) => {
            panic!("Expected error for nonexistent DSN, but connection succeeded");
        }
        Err(e) => {
            println!("✅ Connection error handling working: {:?}", e);
            // Should get a proper ODBC error, not a crash
        }
    }
}
