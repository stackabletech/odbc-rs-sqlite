use crate::connection::ConnectionClass;
use crate::odbc::implementation::implementation::get_info;
use odbc_sys::{InfoType, InfoTypeType, InfoTypeTypeInformation, Pointer, SmallInt, SqlReturn};
use std::ffi::CString;

const STRING_LENGTH_FOR_USMALLINT: i16 = std::mem::size_of::<u16>() as i16;
const STRING_LENGTH_FOR_UINTEGER: i16 = std::mem::size_of::<u32>() as i16;

/// Returns general information about the driver and data source associated with a connection
///
/// https://learn.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetinfo-function
///
/// # Returns
/// `SUCCESS`, `SUCCESS_WITH_INFO`, `ERROR`, or `INVALID_HANDLE`
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLGetInfo(
    connection_handle: *mut ConnectionClass,
    info_type: u16,
    info_value_ptr: Pointer,
    buffer_length: SmallInt,
    string_length_ptr: *mut SmallInt,
) -> SqlReturn {
    println!(
        "SQLGetInfo INFO: info_type={:?}, info_value_ptr={:?}, buffer_length={}, string_length_ptr={:?}",
        info_type, info_value_ptr, buffer_length, string_length_ptr
    );

    if connection_handle.is_null() {
        println!("SQLGetInfo ERROR: connection_handle is null, can't set error details");
        return SqlReturn::INVALID_HANDLE;
    }

    if string_length_ptr.is_null() {
        println!("SQLGetInfo ERROR: string_length_ptr is null");
        // TODO: Set error in connection_handle
        return SqlReturn::ERROR;
    }

    let info_type = match InfoType::try_from(info_type) {
        Ok(info_type) => info_type,
        Err(_) => {
            println!(
                "SQLGetInfo ERROR: The provided info_type is invalid - {}",
                info_type
            );
            // TODO: Set error in connection_handle
            return SqlReturn::ERROR;
        }
    };

    // This checks for proper alignment of `info_value_ptr` which depends on the info_type
    // Strings are byte-aligned (i.e. alignment of 1) as far as I know, so no need to check
    match info_type.return_type() {
        InfoTypeTypeInformation::SqlUSmallInt => {
            if (info_value_ptr as usize) % std::mem::align_of::<u16>() != 0 {
                println!("SQLGetInfo ERROR: Alignment of info_value_ptr is wrong (u16)");
                return SqlReturn::ERROR;
            }
        }
        InfoTypeTypeInformation::SqlUInteger => {
            if (info_value_ptr as usize) % std::mem::align_of::<u32>() != 0 {
                println!("SQLGetInfo ERROR: Alignment of info_value_ptr is wrong (u32)");
                return SqlReturn::ERROR;
            }
        }
        _ => {}
    }

    // TODO: Check buffer_length if info_value_ptr is not null and info_type type is a character string
    // TODO: Unicode variant needs to check if buffer_length is even number, if not -> HY0900

    let result =
        get_info(info_type).map_or(info_type.return_type().not_supported_value(), |value| value);

    // buffer_length is ignored for anything that's not a string as per the specification
    match result {
        InfoTypeType::String(result) => {
            // TODO: Make sure to handle encoding properly here, not sure how
            let c_string = match CString::new(result) {
                Ok(string) => string,
                Err(_e) => {
                    println!("SQLGetInfo ERROR: Converting String to CString failed");
                    return SqlReturn::ERROR;
                    // TODO: Set error in connection_handle
                }
            };
            let c_string_bytes = c_string.as_bytes();
            let c_string_len = c_string_bytes.len();

            if info_value_ptr.is_null() {
                unsafe {
                    *string_length_ptr = c_string_len as i16;
                }
            } else {
                let c_string_bytes_with_nul = c_string.as_bytes_with_nul();
                let c_string_len = c_string_bytes_with_nul.len() - 1; // Exclude the null terminator to get the actual length of the string.

                // Calculate the final string length to be used in the operation.
                let final_string_length = std::cmp::min(c_string_len, buffer_length as usize - 1); // Leave space for the null byte

                unsafe {
                    // Copy the appropriate slice of the string based on the calculated length.
                    std::ptr::copy_nonoverlapping(
                        c_string_bytes_with_nul.as_ptr(),
                        info_value_ptr as *mut u8,
                        final_string_length,
                    );

                    // Cast the `c_void` pointer to a `u8` pointer before dereferencing.
                    // This is safe because we know the original data structure is a u8 buffer.
                    let null_terminator_ptr = info_value_ptr.cast::<u8>().add(final_string_length);

                    // Add the null terminator at the right position.
                    *null_terminator_ptr = 0;

                    // Set the string length excluding the null terminator.
                    *string_length_ptr = final_string_length as i16;
                }
            }
        }
        InfoTypeType::SqlUSmallInt(result) => {
            if !info_value_ptr.is_null() {
                unsafe {
                    *(info_value_ptr as *mut u16) = result;
                }
            }

            unsafe {
                *string_length_ptr = STRING_LENGTH_FOR_USMALLINT;
            }
        }
        InfoTypeType::SqlUInteger(result) => {
            if !info_value_ptr.is_null() {
                unsafe {
                    *(info_value_ptr as *mut u32) = result;
                }
            }

            unsafe { *string_length_ptr = STRING_LENGTH_FOR_UINTEGER }
        }
    }

    SqlReturn::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::os::raw::{c_char, c_void};

    #[test]
    fn test_null_connection_handle() {
        let result = SQLGetInfo(
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
        );
        assert_eq!(result, SqlReturn::INVALID_HANDLE);
    }

    #[test]
    fn test_nullstring_length_ptr() {
        let mut connection = ConnectionClass {};
        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            InfoType::MaxDriverConnections as u16,
            std::ptr::null_mut(),
            2,
            std::ptr::null_mut(),
        );
        assert_eq!(result, SqlReturn::ERROR);
        // TODO: Once implemented check that the correct error type is returned
    }

    #[test]
    fn test_invalid_info_type() {
        let mut connection = ConnectionClass {};
        let string_length_ptr = &mut 0;
        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            9999,
            std::ptr::null_mut(),
            2,
            string_length_ptr,
        );
        assert_eq!(result, SqlReturn::ERROR);
        // TODO: Once implemented check that the correct error type is returned
    }

    #[test]
    fn test_valid_info_type_usmallint() {
        let mut connection = ConnectionClass {};
        let mut value: u16 = 0;
        let mut string_length: i16 = 0;
        let buffer_length = std::mem::size_of_val(&value) as i16;

        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            InfoType::ActiveEnvironments as u16,
            &mut value as *mut u16 as *mut c_void,
            buffer_length,
            &mut string_length as *mut i16,
        );

        assert_eq!(result, SqlReturn::SUCCESS);
        assert_eq!(value, 0); // TODO: Find another info_type that's more interesting
        assert_eq!(string_length, STRING_LENGTH_FOR_USMALLINT);
    }

    #[test]
    fn test_valid_info_type_string() {
        let mut connection = ConnectionClass {};
        let mut buffer = vec![0u8; 256];
        let mut string_length: i16 = 0;

        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            InfoType::UserName as u16,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as i16,
            &mut string_length as *mut i16,
        );

        assert_eq!(result, SqlReturn::SUCCESS);
        assert_eq!(string_length, 3);

        let result_str = String::from_utf8(buffer[..string_length as usize].to_vec())
            .expect("Found invalid UTF-8");
        assert_eq!(result_str, "foo");
    }

    #[test]
    fn test_string_truncation() {
        let mut connection = ConnectionClass {};
        let mut buffer = vec![0u8; 3];
        let mut string_length: i16 = 0;

        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            InfoType::UserName as u16,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as i16,
            &mut string_length as *mut i16,
        );

        assert_eq!(result, SqlReturn::SUCCESS); // TODO: I believe it should return a different thing in this case
        assert_eq!(string_length, buffer.len() as i16 - 1); // should be equal to buffer length if it was truncated

        let result_str = String::from_utf8(buffer[..string_length as usize].to_vec())
            .expect("Found invalid UTF-8");
        assert_eq!(result_str, "fo");
    }

    #[test]
    fn test_insufficient_buffer_for_integer() {
        let mut connection = ConnectionClass {};
        let mut value: u16 = 0; // Not enough space for a u32
        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            InfoType::MaxConcurrentActivities as u16, // Assuming this info_type should return a u32
            &mut value as *mut u16 as *mut c_void,
            std::mem::size_of::<u16>() as i16, // too small!
            std::ptr::null_mut(),
        );
        assert_eq!(result, SqlReturn::ERROR); // or whatever error code you return for insufficient space
    }

    #[test]
    fn test_bad_alignment() {
        let mut connection = ConnectionClass {};
        let mut buffer = vec![0u8; 256];
        let result = unsafe {
            SQLGetInfo(
                &mut connection as *mut ConnectionClass,
                InfoType::ActiveEnvironments as u16,
                (buffer.as_mut_ptr().offset(1)) as *mut c_void, // misaligned
                buffer.len() as i16,
                std::ptr::null_mut(),
            )
        };
        assert_eq!(result, SqlReturn::ERROR);
        // TODO: Once implemented check that the correct error type is returned
    }

    #[test]
    fn test_null_info_value_ptr() {
        let mut connection = ConnectionClass {};
        let mut string_length: i16 = 0;
        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            InfoType::ActiveEnvironments as u16,
            std::ptr::null_mut(),
            0,
            &mut string_length as *mut i16,
        );
        assert_eq!(result, SqlReturn::SUCCESS); // or specific success code indicating no actual data was copied
        assert_eq!(string_length, STRING_LENGTH_FOR_USMALLINT);
    }

    #[test]
    fn test_sqluinteger() {
        let mut connection = ConnectionClass {};
        let info_type = InfoType::ActiveEnvironments;
        let buffer_length = std::mem::size_of::<u32>() as i16;
        let mut string_length: i16 = 0;
        let mut buffer: [c_char; 4] = [0; 4];
        let info_value_ptr: *mut c_void = buffer.as_mut_ptr() as *mut c_void;

        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            info_type as u16,
            info_value_ptr,
            buffer_length,
            &mut string_length as *mut i16,
        );

        assert_eq!(result, SqlReturn::SUCCESS);
        assert_eq!(string_length, 4); // Size of u32
    }

    #[test]
    fn test_sql_get_info() {
        // Test case: SQLUSMALLINT case
        let mut connection = ConnectionClass {};
        let info_type = InfoType::ActiveEnvironments;
        let buffer_length = 2;
        let mut string_length: i16 = 0;
        let mut buffer: [c_char; 2] = [0; 2];
        let info_value_ptr: *mut c_void = buffer.as_mut_ptr() as *mut c_void;

        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            info_type as u16,
            info_value_ptr,
            buffer_length,
            &mut string_length as *mut i16,
        );

        assert_eq!(result, SqlReturn::SUCCESS);
        assert_eq!(string_length, 2); // Size of u16/SQLUSMALLINT)
        let info_value_u16_ptr = info_value_ptr as *const u16;
        unsafe {
            assert_eq!(*info_value_u16_ptr as u16, 0);
        }

        // Test case: String case for an InfoType that is not implemented
        let mut connection = ConnectionClass {};
        let info_type = InfoType::DescribeParameter;
        let buffer_length = 15;
        let mut string_length: i16 = 0;
        let mut buffer: [c_char; 15] = [0; 15];
        let info_value_ptr: *mut c_void = buffer.as_mut_ptr() as *mut c_void;

        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            info_type as u16,
            info_value_ptr,
            buffer_length,
            &mut string_length as *mut i16,
        );

        assert_eq!(result, SqlReturn::SUCCESS);
        assert_eq!(string_length, 1); // Truncated string length (excluding null terminator)

        let c_string = unsafe { CStr::from_ptr(info_value_ptr as *const c_char) };
        let rust_string = c_string.to_string_lossy();
        assert_eq!(rust_string, "N");

        // Test case: String case for a string that is too long
        let mut connection = ConnectionClass {};
        let info_type = InfoType::DescribeParameter;
        let buffer_length = 15;
        let mut string_length: i16 = 0;
        let mut buffer: [c_char; 15] = [0; 15];
        let info_value_ptr: *mut c_void = buffer.as_mut_ptr() as *mut c_void;

        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            info_type as u16,
            info_value_ptr,
            buffer_length,
            &mut string_length as *mut i16,
        );

        assert_eq!(result, SqlReturn::SUCCESS);
        assert_eq!(string_length, 1); // Truncated string length (excluding null terminator)

        let c_string = unsafe { CStr::from_ptr(info_value_ptr as *const c_char) };
        let rust_string = c_string.to_string_lossy();
        assert_eq!(rust_string, "N");

        println!("INVALID");
        let invalid_info_type: u16 = 12345;
        let mut connection = ConnectionClass {};
        let buffer_length = 15;
        //let mut string_length: i16 = 0;
        let mut buffer: [c_char; 15] = [0; 15];

        unsafe {
            let (_prefix, content, _suffix) = buffer.align_to::<u32>();
            assert!(!content.is_empty(), "Buffer is not properly aligned");

            let invalid_bar = std::mem::transmute::<u16, InfoType>(invalid_info_type);
            let info_value_ptr: *mut c_void = buffer.as_mut_ptr() as *mut c_void;

            let _result = SQLGetInfo(
                &mut connection as *mut ConnectionClass,
                invalid_bar as u16,
                info_value_ptr,
                buffer_length,
                //&mut string_length as *mut i16,
                0 as *mut i16,
            );
        }

        /*
        // Test case 2: String case with insufficient buffer length
        let buffer_length = 5;
        let mut string_length: i16 = 0;
        let mut buffer: [c_char; 5] = [0; 5];
        let info_value_ptr: *mut c_void = buffer.as_mut_ptr() as *mut c_void;

        let result = SQLGetInfo(
            &mut connection as *mut ConnectionClass,
            info_type,
            info_value_ptr,
            buffer_length,
            &mut string_length as *mut i16,
        );

        assert_eq!(result, SqlReturn::ERROR); // Buffer too small


         */
    }
}
