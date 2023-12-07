use crate::odbc::implementation::alloc_handles::{
    allocate_stmt_handle, impl_allocate_dbc_handle, impl_allocate_environment_handle,
    ConnectionHandle, EnvironmentHandle,
};
use crate::odbc::utils::{get_from_wrapper, wrap_and_set};
use odbc_sys::{HandleType, Pointer, SmallInt, SqlReturn};

/// SQLAllocHandle allocates an environment, connection, statement, or descriptor handle.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLAllocHandle(
    handle_type: SmallInt,
    input_handle: Pointer,
    output_handle: *mut Pointer,
) -> SqlReturn {
    println!(
        "SQLAllocHandle DEBUG: handle_type={:?}, input_handle={:?}, output_handle={:?}",
        handle_type, input_handle, output_handle
    );

    if output_handle.is_null() {
        println!("SQLAllocHandle ERROR: Output Handle is null");
        return SqlReturn::INVALID_HANDLE;
    }

    let handle_type = match HandleType::try_from(handle_type) {
        Ok(handle_type) => handle_type,
        Err(_) => {
            println!(
                "SQLAllocHandle ERROR: The provided handle_type is invalid, can't set error details - {}",
                handle_type
            );
            return SqlReturn::ERROR;
        }
    };

    // From the spec:
    // When allocating a handle other than an environment handle, if SQLAllocHandle returns SQL_ERROR, it sets OutputHandlePtr to SQL_NULL_HDBC, SQL_NULL_HSTMT, or SQL_NULL_HDESC, depending on the value of HandleType, unless the output argument is a null pointer.
    // The application can then obtain additional information from the diagnostic data structure associated with the handle in the InputHandle argument.
    let result = match handle_type {
        HandleType::Env => {
            // Spec: If HandleType is SQL_HANDLE_ENV, this is SQL_NULL_HANDLE.
            if !input_handle.is_null() {
                println!("SQLAllocHandle ERROR: handle_type is Env but input_handle is not null");
                return SqlReturn::ERROR;
            }

            // Call the implementation and convert the output properly
            let handle = impl_allocate_environment_handle();
            wrap_and_set(handle_type, handle, output_handle);

            println!("SQLAllocHandle INFO: Successfully allocated an environment handle");

            SqlReturn::SUCCESS
        }
        HandleType::Dbc => {
            // Spec: If HandleType is SQL_HANDLE_DBC, this must be an environment handle
            if input_handle.is_null() {
                println!("SQLAllocHandle ERROR: handle_type is Dbc but input_handle is null");
                unsafe { *output_handle = std::ptr::null_mut() }
                return SqlReturn::ERROR;
            }

            // Validate that the input handle really is an environment handle
            // I don't know a better way to do this as I can't use the question mark operator and also not any of the methods on Result.
            // Because we need to return from here.
            let env_handle: &mut EnvironmentHandle =
                match get_from_wrapper(&HandleType::Env, input_handle) {
                    Ok(env) => env,
                    Err(err) => {
                        println!("SQLAllocHandle ERROR: {}", err);
                        unsafe { *output_handle = std::ptr::null_mut() }
                        return SqlReturn::ERROR;
                    }
                };

            let handle = impl_allocate_dbc_handle(env_handle);
            wrap_and_set(handle_type, handle, output_handle);

            println!("SQLAllocHandle INFO: Successfully allocated a Dbc handle");

            SqlReturn::SUCCESS
        }
        HandleType::Stmt => {
            let connection_handle: &mut ConnectionHandle =
                match get_from_wrapper(&HandleType::Dbc, input_handle) {
                    Ok(env) => env,
                    Err(err) => {
                        println!("SQLAllocHandle ERROR: {}", err);
                        unsafe { *output_handle = std::ptr::null_mut() }
                        return SqlReturn::ERROR;
                    }
                };

            let handle = allocate_stmt_handle(connection_handle);
            wrap_and_set(handle_type, handle, output_handle);

            println!("SQLAllocHandle INFO: Successfully allocated a Stmt handle");

            SqlReturn::SUCCESS
        }
        HandleType::Desc => SqlReturn::SUCCESS,
        HandleType::DbcInfoToken => SqlReturn::SUCCESS,
    };

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlallochandle() {
        let mut output_handle: Pointer = std::ptr::null_mut();

        // Setup Env handle
        let ret = SQLAllocHandle(
            HandleType::Env as i16,
            std::ptr::null_mut(),
            &mut output_handle,
        );

        assert_eq!(ret, SqlReturn::SUCCESS);
        assert!(!output_handle.is_null());

        // Setup Dbc handle
        let input_handle = output_handle;
        let mut output_handle: Pointer = std::ptr::null_mut();
        let ret = SQLAllocHandle(HandleType::Dbc as i16, input_handle, &mut output_handle);

        assert_eq!(ret, SqlReturn::SUCCESS);
        assert!(!output_handle.is_null());
    }
}
