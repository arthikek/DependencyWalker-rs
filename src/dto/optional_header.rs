use crate::dto::optional_headers_standard_fields::OptionalHeadersStandardFields;
use crate::dto::optional_headers_windowsspecific_fields::OptionalHeadersWindowsSpecificFields;
use crate::enums::word::Word;

pub struct OptionalHeader {
    magic_number: Word,
    optional_header_standard_fields : OptionalHeadersStandardFields,
    optional_headers_windows_specific_fields: OptionalHeadersWindowsSpecificFields
}