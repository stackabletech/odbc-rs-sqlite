use odbc_sys::SqlReturn;
use std::ffi::c_void;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQLColAttributeW(
    _statement_handle: *mut c_void,
    _column_number: u16,
    _field_identifier: u16,
    _character_attribute_ptr: *mut c_void,
    _buffer_length: i16,
    _string_length_ptr: *mut i16,
    _numeric_attribute_ptr: *mut isize,
) -> SqlReturn {
    println!("SQLColAttributeW INFO");
    SqlReturn::SUCCESS
}

/*

   println!(
       "SQLColAttributeW(column_number={}, field_identifier={:?}, buffer_length={})",
       column_number, field_identifier, buffer_length
   );

   match field_identifier {
       Desc::Count => {}
       Desc::Type => {}
       Desc::Length => {}
       Desc::OctetLengthPtr => {}
       Desc::Precision => {}
       Desc::Scale => {}
       Desc::DatetimeIntervalCode => {}
       Desc::Nullable => {}
       Desc::IndicatorPtr => {}
       Desc::DataPtr => {}
       Desc::Name => {}
       Desc::Unnamed => {}
       Desc::OctetLength => {}
       Desc::AllocType => {}
       Desc::ArraySize => {}
       Desc::ArrayStatusPtr => {}
       Desc::AutoUniqueValue => {}
       Desc::BaseColumnName => {}
       Desc::BaseTableName => {}
       Desc::BindOffsetPtr => {}
       Desc::BindType => {}
       Desc::CaseSensitive => {}
       Desc::CatalogName => {}
       Desc::ConciseType => {}
       Desc::DatetimeIntervalPrecision => {}
       Desc::DisplaySize => unsafe {
           *numeric_attribute_ptr = 20;
       },
       Desc::FixedPrecScale => {}
       Desc::Label => {
           if !character_attribute_ptr.is_null() {
               let os_string =
                   U16CString::from_str("foobar").expect("U16CString::from_str failed");

               // Make sure 'buffer_length' is the maximum length you can handle in wide characters
               if buffer_length >= os_string.len() as i16 {
                   // Set string_length_ptr to the length of the wide string
                   if !string_length_ptr.is_null() {
                       unsafe {
                           *string_length_ptr = os_string.len() as i16;
                       }
                   }

                   // Copy the wide string into the memory pointed to by character_attribute_ptr
                   unsafe {
                       ptr::copy_nonoverlapping(
                           os_string.as_ptr() as *const c_void,
                           character_attribute_ptr,
                           os_string.len() * 2, // Each character is 2 bytes in UTF-16
                       );
                   }
               } else {
                   // Handle buffer too small error
               }
           }
       }
       Desc::LiteralPrefix => {}
       Desc::LiteralSuffix => {}
       Desc::LocalTypeName => {}
       Desc::MaximumScale => {}
       Desc::MinimumScale => {}
       Desc::NumPrecRadix => {}
       Desc::ParameterType => {}
       Desc::RowsProcessedPtr => {}
       Desc::RowVer => {}
       Desc::SchemaName => {}
       Desc::Searchable => {}
       Desc::TypeName => {}
       Desc::TableName => {}
       Desc::Unsigned => {}
       Desc::Updatable => {}
   }

   SqlReturn::SUCCESS
*/
