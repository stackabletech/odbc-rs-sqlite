use odbc_sys::{SqlReturn, USmallInt};
use std::ffi::c_void;
use std::slice;

const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: usize = 250;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLGetFunctions(
    _connection_handle: *mut c_void,
    function_id: u16,
    supported_ptr: &mut USmallInt,
) -> SqlReturn {
    println!("SQLGetFunctions INFO: function_id: {}", function_id);

    if function_id == 999 {
        let supported_array =
            unsafe { slice::from_raw_parts_mut(supported_ptr, SQL_API_ODBC3_ALL_FUNCTIONS_SIZE) };

        for i in 0..1500 {
            sql_func_eset(supported_array, i);
        }
    }

    /*
    match function_id {
        1 => {
            let mut data: [u16; 100] = [0; 100]; // Initialize all elements to 0
            for supported_function in supported_functions {
                let function_id = supported_function as u16;
                if function_id < 100 {
                    data[function_id as usize] = 1
                }
            }
            unsafe {
                // TODO *supported_ptr = data.as_mut_ptr();
            }
        }
        GetFunctionsArgument::Odbc3AllFunctions => {}
        GetFunctionsArgument::Function(_) => {}
    }

     */

    /*
    let supported_functions = get_supported_functions();


     */

    SqlReturn::SUCCESS
}

fn sql_func_eset(pf_exists: &mut [u16], uw_api: usize) {
    pf_exists[uw_api >> 4] |= 1 << (uw_api & 0x000F);
}

fn sql_func_exists(pf_exists: &[u16], uw_api: usize) -> bool {
    (pf_exists[uw_api >> 4] & (1 << (uw_api & 0x000F))) != 0
}
