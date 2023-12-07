use crate::odbc::implementation::alloc_handles::EnvironmentHandle;
use crate::odbc::implementation::env_attrs::get_odbc_version;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{EnvironmentAttribute, HandleType, Integer, Pointer, SqlReturn};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLGetEnvAttr(
    environment_handle: Pointer,
    attribute: Integer,
    value_ptr: Pointer,
    _buffer_length: Integer, // There are no attributes which require returning a string
    _string_length_ptr: *mut Integer, // There are no attributes which require returning a string
) -> SqlReturn {
    println!("SQLGetEnvAttr DEBUG: attribute={}", attribute);

    if environment_handle.is_null() {
        println!("SQLGetEnvAttr ERROR: Environment handle is null");
        return SqlReturn::INVALID_HANDLE;
    }

    if value_ptr.is_null() {
        println!("SQLGetEnvAttr ERROR: value_ptr is null");
        // TODO
        return SqlReturn::ERROR;
    }

    let attribute = match EnvironmentAttribute::try_from(attribute) {
        Ok(attribute) => attribute,
        Err(_) => {
            println!(
                "SQLGetEnvAttr ERROR: The provided attribute is invalid: {}",
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

    match attribute {
        EnvironmentAttribute::OdbcVersion => {
            let odbc_version = get_odbc_version(env);
            unsafe { *(value_ptr as *mut i32) = odbc_version as i32 }
        }
        EnvironmentAttribute::ConnectionPooling => {
            // TODO: This is implemented in the driver manager, not the driver
            return SqlReturn::ERROR;
        }
        EnvironmentAttribute::CpMatch => {
            // TODO: This is implemented in the driver manager, not the driver
            return SqlReturn::ERROR;
        }
        EnvironmentAttribute::OutputNts => unsafe { *(value_ptr as *mut i32) = 1 },
    }

    SqlReturn::SUCCESS
}
