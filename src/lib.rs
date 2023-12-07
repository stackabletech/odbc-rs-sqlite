use odbc_sys::{Integer, WChar};

mod connection;
mod odbc;

#[cfg_attr(windows, link(name = "odbc32"))]
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
extern "C" {
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
