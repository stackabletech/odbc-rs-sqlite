use crate::odbc::implementation::alloc_handles::EnvironmentHandle;
use crate::odbc::implementation::env_attrs::set_odbc_version;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{AttrOdbcVersion, EnvironmentAttribute, HandleType, Integer, Pointer, SqlReturn};

///  SQLSetEnvAttr sets attributes that govern aspects of environments.
#[allow(non_snake_case)]
#[no_mangle]
pub fn SQLSetEnvAttr(
    environment_handle: Pointer,
    attribute: Integer,
    value_ptr: Pointer,
    _string_length: i32, // There are currently no attributes that take a String so this is actually unused
) -> SqlReturn {
    println!("SQLSetEnvAttr DEBUG: attribute={}", attribute);

    if environment_handle.is_null() {
        println!("SQLSetEnvAttr ERROR: Environment handle is null");
        return SqlReturn::INVALID_HANDLE;
    }

    if value_ptr.is_null() {
        println!("SQLSetEnvAttr ERROR: value_ptr is null");
        // TODO
        return SqlReturn::ERROR;
    }

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

    let env: &mut EnvironmentHandle = match get_from_wrapper(&HandleType::Env, environment_handle) {
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
            };
        }
    }

    SqlReturn::SUCCESS
}
