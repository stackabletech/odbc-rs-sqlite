use crate::odbc::handles::StatementHandle;
use crate::odbc::utils::get_from_wrapper;
use odbc_sys::{CDataType, HandleType, SqlReturn};
use std::ffi::{CString, c_void};
use tracing::{debug, error, info, warn};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SQLGetData(
    statement_handle: *mut c_void,
    col_or_param_num: u16,
    target_type: i16,
    target_value_ptr: *mut c_void,
    buffer_length: isize,
    str_len_or_ind_ptr: *mut isize,
) -> SqlReturn {
    info!("Getting data");

    let statement_handle: &mut StatementHandle =
        match get_from_wrapper(&HandleType::Stmt, statement_handle) {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to get statement handle: {}", err);
                return SqlReturn::INVALID_HANDLE;
            }
        };

    if col_or_param_num == 0 {
        warn!("Bookmarks not supported yet");
        return SqlReturn::ERROR;
    }

    let target_type = match CDataType::try_from(target_type) {
        Ok(t) => t,
        Err(e) => {
            error!(
                "Could not convert {} to valid target type: {}",
                target_type, e
            );
            return SqlReturn::ERROR;
        }
    };

    debug!(
        "Requested target_type: {:?}, col_or_param: {}",
        target_type, col_or_param_num
    );

    let stmt = match &statement_handle.active_statement {
        Some(stmt) => stmt,
        None => {
            error!("No active statement; call SQLFetch first");
            return SqlReturn::ERROR;
        }
    };

    let col_index = (col_or_param_num - 1) as usize;
    let result = match stmt.get_data(col_index, target_type) {
        Ok(value) => value,
        Err(err) => {
            error!("get_data failed: {}", err);
            return SqlReturn::ERROR;
        }
    };

    let c_string = match CString::new(result) {
        Ok(s) => s,
        Err(_) => {
            error!("Converting String to CString failed");
            return SqlReturn::ERROR;
        }
    };

    let c_string_bytes_with_nul = c_string.as_bytes_with_nul();
    let c_string_len = c_string_bytes_with_nul.len() - 1;

    let final_string_length = std::cmp::min(c_string_len, buffer_length as usize - 1);

    unsafe {
        std::ptr::copy_nonoverlapping(
            c_string_bytes_with_nul.as_ptr(),
            target_value_ptr as *mut u8,
            final_string_length,
        );

        let null_terminator_ptr = target_value_ptr.cast::<u8>().add(final_string_length);
        *null_terminator_ptr = 0;

        *str_len_or_ind_ptr = final_string_length as isize;
    }

    SqlReturn::SUCCESS
}
