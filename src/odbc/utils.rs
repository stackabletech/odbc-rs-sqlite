use crate::odbc::utils::Error::Utf16Error;
use crate::SQLGetPrivateProfileStringW;
use odbc_sys::{HandleType, Pointer, NTS};
use snafu::prelude::*;
use std::ffi::c_void;
use std::string::FromUtf16Error;

const DEFAULT_MARKER: &str = "___DEFAULT___";

const ENV_TAG: i32 = 1488005347;
const DBC_TAG: i32 = 190273262;
const STMT_TAG: i32 = 292201727;
const DESC_TAG: i32 = 108498290;
const DBC_INFO_TOKEN_TAG: i32 = 983280932;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("SQLGetPrivateProfileStringW returned a negative value: {ret}"))]
    UnknownError { ret: i32 },

    #[snafu(context(false))]
    Utf16Error { source: FromUtf16Error },

    #[snafu(display(
        "Tag for wrapper was expected to be {expected_tag} but was actually {received_tag}"
    ))]
    InvalidTag {
        expected_tag: i32,
        received_tag: i32,
    },

    #[snafu(display("Pointer for {handle_type:?} was null"))]
    NullPointer { handle_type: HandleType },
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct HandleWrapper {
    pub tag: i32,
    pub handle: *mut c_void,
}

pub(crate) fn tag_for_handle(handle_type: &HandleType) -> i32 {
    match handle_type {
        HandleType::Env => ENV_TAG,
        HandleType::Dbc => DBC_TAG,
        HandleType::Stmt => STMT_TAG,
        HandleType::Desc => DESC_TAG,
        HandleType::DbcInfoToken => DBC_INFO_TOKEN_TAG,
    }
}

// I know this is all kinds of unsafe.
// The lifetime of the output is tied to the input raw pointer which is not something that can be expressed in Rust.
// And even if it could be expressed it'd be meaningless because Rust doesn't control the raw pointer either.
// In short: The caller needs to make sure that this is called in a safe way and that the object the pointer points to is actually of handle_type.
pub fn get_from_wrapper<'a, T>(
    handle_type: &HandleType,
    wrapper_pointer: Pointer,
) -> Result<&'a mut T, Error> {
    ensure!(
        !wrapper_pointer.is_null(),
        NullPointerSnafu {
            handle_type: handle_type.clone()
        }
    );

    let in_wrapper: &HandleWrapper = unsafe { &*(wrapper_pointer as *const HandleWrapper) };

    let tag = tag_for_handle(&handle_type);
    ensure!(
        in_wrapper.tag == tag,
        InvalidTagSnafu {
            expected_tag: tag,
            received_tag: in_wrapper.tag
        }
    );

    Ok(unsafe { &mut *(in_wrapper.handle as *mut T) })
}

/// This will create a wrapper around the `object` on move it to the heap by Boxing it, will then create a wrapper of `handle_type`
/// around it and lets the pointer point at it.
pub(crate) fn wrap_and_set<T>(handle_type: HandleType, object: T, pointer: *mut Pointer) {
    let tag = tag_for_handle(&handle_type);

    // The object will usually be allocated on the stack and we'll need to move it to the heap before passing it on
    // I have no idea how I can codify the fact that `output_object` should not be a Box already
    let raw_ptr = Box::into_raw(Box::new(object)) as *mut c_void;

    let wrapper = HandleWrapper {
        tag,
        handle: raw_ptr,
    };

    let boxed_output = Box::new(wrapper);
    let raw_ptr: *mut HandleWrapper = Box::into_raw(boxed_output);
    unsafe { *pointer = raw_ptr as *mut c_void }
}

pub fn get_private_profile_string(
    section: &str,
    entry: &str,
    filename: &str,
    buffer_size: usize,
) -> Result<Option<String>, Error> {
    let section_wstring: Vec<u16> = section.encode_utf16().chain(Some(0)).collect();
    let entry_wstring: Vec<u16> = entry.encode_utf16().chain(Some(0)).collect();
    let default_wstring: Vec<u16> = DEFAULT_MARKER.encode_utf16().chain(Some(0)).collect();
    let filename_wstring: Vec<u16> = filename.encode_utf16().chain(Some(0)).collect();

    let mut ret_buffer: Vec<u16> = vec![0; buffer_size];

    let chars_read = unsafe {
        SQLGetPrivateProfileStringW(
            section_wstring.as_ptr(),
            entry_wstring.as_ptr(),
            default_wstring.as_ptr(),
            ret_buffer.as_mut_ptr(),
            ret_buffer.len() as i32,
            filename_wstring.as_ptr(),
        )
    };

    ensure!(chars_read >= 0, UnknownSnafu { ret: chars_read });

    // TODO: Test what happens when buffer is too small
    if chars_read as usize >= ret_buffer.len() {
        // Handle potential buffer overflow or increase buffer size
    }

    // TODO Not sure if this is correct as this should return e.g. 26 bytes for 13 characters
    // unixODBC seems to return single byte characters even for the "W" version
    // We probably need
    // unixODBC also does not seem to add a null termination char so we need to make sure to initialize the array with zeroes but even that is weird should it ever return real UTF-16 where every secon dbyte will be 0
    ret_buffer.truncate(chars_read as usize);

    match String::from_utf16(&ret_buffer) {
        Ok(string) if string == DEFAULT_MARKER => Ok(None),
        Ok(string) => Ok(Some(string)),
        Err(e) => Err(Utf16Error { source: e }), // TODO
    }
}

/// Maybe converts a pointer and a length to a String.
/// This automatically takes care of handling the `NTS` flag.
pub fn maybe_utf16_to_string(pointer: *const u16, length: i16) -> Option<String> {
    match pointer.is_null() {
        true => None,
        false => Some(utf16_to_string(pointer, length)),
    }
}

pub fn utf16_to_string(pointer: *const u16, length: i16) -> String {
    // Ensure the pointer is not null
    assert!(!pointer.is_null());

    let slice = if length == NTS as i16 {
        // Handle as null-terminated string
        let mut len = 0;
        let mut curr_ptr = pointer;
        unsafe {
            // Count characters until null terminator
            while *curr_ptr != 0 {
                len += 1;
                curr_ptr = curr_ptr.add(1);
            }
            // Create a slice from the raw pointer and length
            std::slice::from_raw_parts(pointer, len as usize)
        }
    } else {
        // Create a slice from the raw pointer and length
        unsafe { std::slice::from_raw_parts(pointer, length as usize) }
    };

    String::from_utf16_lossy(slice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profilestuff() {
        let result = get_private_profile_string("test_connection", "Driver", "", 1024);

        println!("{:?}", result);
    }
}
