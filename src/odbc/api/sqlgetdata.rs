use crate::odbc::implementation::alloc_handles::StatementHandle;
use crate::odbc::implementation::getdata::impl_getdata;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{CDataType, HandleType, SqlReturn};
use std::ffi::{c_void, CString};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLGetData(
    statement_handle: *mut c_void,
    col_or_param_num: u16,
    target_type: i16,
    target_value_ptr: *mut c_void,
    buffer_length: isize,
    str_len_or_ind_ptr: *mut isize,
) -> SqlReturn {
    println!("SQLGetData INFO");

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                println!("SQLGetData ERROR: {}", err);
                return SqlReturn::ERROR;
            }
        };

    if col_or_param_num == 0 {
        // TODO
        println!("SQLGetData WARN: Bookmarks not supported yet");
        return SqlReturn::ERROR;
    }

    // TODO: Have a generic way to check if the requested column is even in range

    let target_type = match CDataType::try_from(target_type) {
        Ok(target_type) => target_type,
        Err(e) => {
            println!(
                "SQLGetData ERROR: Could not convert {} to valid target type: {}",
                target_type, e
            );
            return SqlReturn::ERROR;
        }
    };

    println!(
        "SQLGetData DEBUG: Requested target_type: {:?}, col_or_param: {}",
        target_type, col_or_param_num
    );

    let result = impl_getdata(statement_handle, &target_type, col_or_param_num);

    // TODO: Make sure to handle encoding properly here, not sure how
    let c_string = match CString::new(result) {
        Ok(string) => string,
        Err(_e) => {
            println!("SQLGetInfo ERROR: Converting String to CString failed");
            return SqlReturn::ERROR;
            // TODO: Set error in connection_handle
        }
    };

    let c_string_bytes_with_nul = c_string.as_bytes_with_nul();
    let c_string_len = c_string_bytes_with_nul.len() - 1; // Exclude the null terminator to get the actual length of the string.

    // Calculate the final string length to be used in the operation.
    let final_string_length = std::cmp::min(c_string_len, buffer_length as usize - 1); // Leave space for the null byte

    unsafe {
        // Copy the appropriate slice of the string based on the calculated length.
        std::ptr::copy_nonoverlapping(
            c_string_bytes_with_nul.as_ptr(),
            target_value_ptr as *mut u8,
            final_string_length,
        );

        // Cast the `c_void` pointer to a `u8` pointer before dereferencing.
        // This is safe because we know the original data structure is a u8 buffer.
        let null_terminator_ptr = target_value_ptr.cast::<u8>().add(final_string_length);

        // Add the null terminator at the right position.
        *null_terminator_ptr = 0;

        // Set the string length excluding the null terminator.
        *str_len_or_ind_ptr = final_string_length as isize;
    }

    SqlReturn::SUCCESS
}
