use odbc_sys::{Integer, Pointer, SqlReturn};
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLSetConnectAttr(
    connection_handle: Pointer,
    attribute: Integer,
    value_ptr: *mut c_void, // TODO maybe char?
    _str_length: Integer,
) -> SqlReturn {
    println!("SQLSetConnectAttr DEBUG: attribute={}", attribute);

    if connection_handle.is_null() {
        println!("SQLSetConnectAttr ERROR: Connection handle is null");
        return SqlReturn::INVALID_HANDLE;
    }

    if value_ptr.is_null() {
        println!("SQLSetConnectAttr INFO: value_ptr is null");
        // TODO not everytime is an error, e.g. attribute 115

        /*
                /*	ODBC Driver Manager sets this connection attribute to a unicode driver
            (which supports SQLConnectW) when the application is an ANSI application
            (which calls SQLConnect, SQLDriverConnect, or SQLBrowseConnect).
            This is SetConnectAttr only and application does not set this attribute
            This attribute was introduced because some unicode driver's some APIs may
            need to behave differently on ANSI or Unicode applications. A unicode
            driver, which  has same behavior for both ANSI or Unicode applications,
            should return SQL_ERROR when the driver manager sets this connection
            attribute. When a unicode driver returns SQL_SUCCESS on this attribute,
            the driver manager treates ANSI and Unicode connections differently in
            connection pooling.
        */
        #define SQL_ATTR_ANSI_APP			115
                 */
        //return SqlReturn::ERROR;
    }
    /*
    let attribute = match EnvironmentAttribute::try_from(attribute) {
        Ok(attribute) => attribute,
        Err(_) => {
            println!(
                "SQLSetEnvAttr ERROR: The provided attribute is invalid: {}",
                attribute
            );
            // TODO set errors
            return SqlReturn::ERROR;
        }
    };

    let env: &mut Environment = match get_from_wrapper(&HandleType::Env, environment_handle) {
        Ok(env) => env,
        Err(err) => {
            println!("SQLSetEnvAttr ERROR: {}", err);
            return SqlReturn::ERROR;
        }
    };


    // TODO: Lock Environment

    match attribute {
        EnvironmentAttribute::OdbcVersion => {
            let value = value_ptr as i32;

            let odbc_version = match AttrOdbcVersion::try_from(value) {
                Ok(odbc_version) => odbc_version,
                Err(_) => {
                    println!(
                                    "SQLSetEnvAttr ERROR: The provided ODBC version is invalid or not supported: {}",
                                    value
                                );
                    // TODO
                    return SqlReturn::ERROR;
                }
            };

            set_odbc_version(env, odbc_version);
        }
        EnvironmentAttribute::ConnectionPooling => {
            // TODO: This is implemented in the driver manager, not the driver
            return SqlReturn::ERROR;
        }
        EnvironmentAttribute::CpMatch => {
            // TODO: This is implemented in the driver manager, not the driver
            return SqlReturn::ERROR;
        }
        EnvironmentAttribute::OutputNts => {
            // A 32-bit integer that determines how the driver returns string data. If SQL_TRUE, the driver returns string data null-terminated. If SQL_FALSE, the driver does not return string data null-terminated.
            //
            // This attribute defaults to SQL_TRUE. A call to SQLSetEnvAttr to set it to SQL_TRUE returns SQL_SUCCESS. A call to SQLSetEnvAttr to set it to SQL_FALSE returns SQL_ERROR and SQLSTATE HYC00 (Optional feature not implemented).
            let value = value_ptr as i32;
            return if value == 1 {
                SqlReturn::SUCCESS
            } else {
                // TODO
                SqlReturn::ERROR
            }
        }
    }

     */

    SqlReturn::SUCCESS
}
