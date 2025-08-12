use crate::odbc::implementation::alloc_handles::{
    ConnectionHandle, EnvironmentHandle, StatementHandle,
};
use crate::odbc::utils::{HandleWrapper, tag_for_handle};
use odbc_sys::{HandleType, SqlReturn};
use std::ffi::c_void;
use tracing::{debug, error, info, warn};

/// SQLFreeHandle frees resources associated with a specific environment, connection, statement, or descriptor handle.
///
/// This function properly deallocates the memory used by ODBC handles that were created with SQLAllocHandle.
/// It's critical for preventing memory leaks in ODBC applications.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLFreeHandle(handle_type: i16, handle: *mut c_void) -> SqlReturn {
    info!("Freeing handle of type {}", handle_type);

    // Validate handle is not null
    if handle.is_null() {
        error!("Handle is null");
        return SqlReturn::INVALID_HANDLE;
    }

    // Convert handle_type to HandleType enum
    let handle_type_enum = match HandleType::try_from(handle_type) {
        Ok(ht) => ht,
        Err(_) => {
            error!("Invalid handle type: {}", handle_type);
            return SqlReturn::INVALID_HANDLE;
        }
    };

    debug!("Processing {:?} handle", handle_type_enum);

    // Get the expected tag for this handle type
    let expected_tag = tag_for_handle(&handle_type_enum);

    // Cast the handle to a HandleWrapper and validate the tag
    let wrapper: &HandleWrapper = unsafe { &*(handle as *const HandleWrapper) };

    if wrapper.tag != expected_tag {
        error!(
            "Invalid handle tag. Expected {}, got {}",
            expected_tag, wrapper.tag
        );
        return SqlReturn::INVALID_HANDLE;
    }

    debug!("Handle validation successful");

    // Free the handle based on its type
    unsafe {
        match handle_type_enum {
            HandleType::Env => {
                debug!("Freeing Environment handle");
                // First free the inner EnvironmentHandle
                let _env_handle = Box::from_raw(wrapper.handle as *mut EnvironmentHandle);
                // Then free the wrapper
                let _wrapper_box = Box::from_raw(handle as *mut HandleWrapper);
                debug!("Environment handle freed successfully");
            }
            HandleType::Dbc => {
                debug!("Freeing Connection handle");
                // First free the inner ConnectionHandle
                let _conn_handle = Box::from_raw(wrapper.handle as *mut ConnectionHandle);
                // Then free the wrapper
                let _wrapper_box = Box::from_raw(handle as *mut HandleWrapper);
                debug!("Connection handle freed successfully");
            }
            HandleType::Stmt => {
                debug!("Freeing Statement handle");
                // First free the inner StatementHandle
                let _stmt_handle = Box::from_raw(wrapper.handle as *mut StatementHandle);
                // Then free the wrapper
                let _wrapper_box = Box::from_raw(handle as *mut HandleWrapper);
                debug!("Statement handle freed successfully");
            }
            HandleType::Desc => {
                warn!("Descriptor handles not fully implemented yet");
                // For now, just free the wrapper - descriptor handles are not fully implemented
                let _wrapper_box = Box::from_raw(handle as *mut HandleWrapper);
            }
            HandleType::DbcInfoToken => {
                warn!("DbcInfoToken handles not fully implemented yet");
                // For now, just free the wrapper - DbcInfoToken handles are not fully implemented
                let _wrapper_box = Box::from_raw(handle as *mut HandleWrapper);
            }
        }
    }

    info!("Handle freed successfully");
    SqlReturn::SUCCESS
}
