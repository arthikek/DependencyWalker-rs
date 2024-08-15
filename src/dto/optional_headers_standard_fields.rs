use crate::enums::dword::DWord;
use crate::enums::word::Word;
use crate::enums::half_word::HalfWord;

pub struct OptionalHeadersStandardFields {
    magic: Word,
    major_linker_version: HalfWord,
    minor_linker_version: HalfWord,
    size_of_code: DWord,
    size_of_initialized_data: DWord,
    size_of_uninitialized_data: DWord,
    address_of_entry_point: DWord,
    base_of_code: Option<DWord>
}