use crate::enums::dword::DWord;
use crate::enums::size_base::SizeBase;
use crate::enums::word::Word;

pub struct OptionalHeadersWindowsSpecificFields {
    image_base: SizeBase,
    section_alignment: DWord,
    file_alignment: DWord,
    major_os_version: Word,
    minor_os_version: Word,
    major_image_version: Word,
    major_subsystem_version: Word,
    minor_subsystem_version: Word,
    win32_version_value: DWord,
    size_of_image: DWord,
    size_of_headers: DWord,
    checksum: DWord,
    subsystem: Word,
    dll_characteristics: Word,
    size_of_stack_reserve: SizeBase,
    size_of_stack_commit: SizeBase,
    size_of_heap_reserve: SizeBase,
    size_of_heap_commit: SizeBase,
    loader_flags: DWord,
    number_of_rva_and_sizes: DWord,
}