use odbc_sys::{Integer, WChar};

mod connection;
mod logging;
mod odbc;

// Cross-platform ODBC library linking configuration
// These attributes handle the complexity of linking against different ODBC implementations
// across Windows, Linux, and macOS with support for both static and dynamic linking.

// Windows: Use the built-in Windows ODBC library
#[cfg_attr(windows, link(name = "odbc32"))]
// Unix + Dynamic + unixODBC (default case for Linux)
#[cfg_attr(
    all(not(windows), not(feature = "static"), not(feature = "iodbc")),
    link(name = "odbcinst")
)]
// Unix + Static + unixODBC (for self-contained binaries)
#[cfg_attr(
    all(not(windows), feature = "static", not(feature = "iodbc")),
    link(name = "odbcinst", kind = "static")
)]
// Unix + Dynamic + iODBC (common on macOS)
#[cfg_attr(
    all(not(windows), not(feature = "static"), feature = "iodbc"),
    link(name = "iodbcinst")
)]
// Unix + Static + iODBC (self-contained binaries with iODBC)
#[cfg_attr(
    all(not(windows), feature = "static", feature = "iodbc"),
    link(name = "iodbcinst", kind = "static")
)]

// This function is here because it's needed for ODBC configuration file parsing
// but doesn't exist in the standard odbc-sys crate.
// It was added here directly temporarily, eventually this might move to a separate file or library.
// We tried contributing it upstream in odbc-sys but it got rejected for good reason:
// https://github.com/pacman82/odbc-sys/pull/44
unsafe extern "C" {
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
