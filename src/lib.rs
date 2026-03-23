use odbc_sys::{Integer, WChar};

mod logging;
mod odbc;

/// Registers the SQLite implementation with the driver's handle factory.
/// Called once at driver startup (from SQLAllocHandle for environment handles).
pub(crate) fn init_driver() {
    odbc::handles::register_factory(Box::new(
        odbc::implementation::query::SqliteDbConnectionFactory,
    ));
}

// SQLGetPrivateProfileStringW is an ODBC installer/configuration function, not a Driver Manager
// function. It lives in different libraries depending on platform:
//   Windows:           odbccp32.dll  (ODBC installer library, NOT odbc32.dll)
//   Unix + unixODBC:   libodbcinst   (dynamic or static)
//   Unix + iODBC:      libiodbcinst  (dynamic or static)
//
// This function is declared here because it is not in the standard odbc-sys crate.
// An upstream contribution was rejected: https://github.com/pacman82/odbc-sys/pull/44
#[cfg_attr(windows, link(name = "odbccp32"))]
#[cfg_attr(
    all(not(windows), not(feature = "static"), not(feature = "iodbc")),
    link(name = "odbcinst")
)]
#[cfg_attr(
    all(not(windows), feature = "static", not(feature = "iodbc")),
    link(name = "odbcinst", kind = "static")
)]
#[cfg_attr(
    all(not(windows), not(feature = "static"), feature = "iodbc"),
    link(name = "iodbcinst")
)]
#[cfg_attr(
    all(not(windows), feature = "static", feature = "iodbc"),
    link(name = "iodbcinst", kind = "static")
)]
unsafe extern "system" {
    ///  Gets a list of names of values or data corresponding to a value of the system information.
    ///
    /// # Returns
    ///
    /// The amount of characters returned (negative indicates an error)
    pub fn SQLGetPrivateProfileStringW(
        section: *const WChar,
        entry: *const WChar,
        default: *const WChar,
        ret_buffer: *mut WChar,
        ret_buffer_size: Integer,
        filename: *const WChar,
    ) -> i32;
}
