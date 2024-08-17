use crate::{DWord, HalfWord, Word};

#[derive(Clone, Debug)]
pub struct OptionalHeadersStandardFields {
    magic: Word,
    major_linker_version: HalfWord,
    minor_linker_version: HalfWord,
    size_of_code: DWord,
    size_of_initialized_data: DWord,
    size_of_uninitialized_data: DWord,
    address_of_entry_point: DWord,
    base_of_code: Option<DWord>,
}

impl OptionalHeadersStandardFields {
    pub fn new(
        magic: Word,
        major_linker_version: HalfWord,
        minor_linker_version: HalfWord,
        size_of_code: DWord,
        size_of_initialized_data: DWord,
        size_of_uninitialized_data: DWord,
        address_of_entry_point: DWord,
        base_of_code: Option<DWord>,
    ) -> Self {
        Self {
            magic,
            major_linker_version,
            minor_linker_version,
            size_of_code,
            size_of_initialized_data,
            size_of_uninitialized_data,
            address_of_entry_point,
            base_of_code,
        }
    }
}
