use crate::{DWord, QWord, Word};

#[derive(Debug, Clone)]
pub struct SectionHeader {
    name: QWord,
    virtual_size: DWord,
    virtual_adress: DWord,
    size_of_raw_data: QWord,
    pointer_to_raw_data: QWord,
    pointer_to_relocations: QWord,
    pointer_to_line_numbers: QWord,
    number_of_relocations: Word,
    number_of_line_numbers: Word,
    characteristics: Word,
}

impl SectionHeader {
    pub fn new(
        name: QWord,
        virtual_size: DWord,
        virtual_adress: DWord,
        size_of_raw_data: QWord,
        pointer_to_raw_data: QWord,
        pointer_to_relocations: QWord,
        pointer_to_line_numbers: QWord,
        number_of_relocations: Word,
        number_of_line_numbers: Word,
        characteristics: Word,
    ) -> Self {
        Self {
            name,
            virtual_size,
            virtual_adress,
            size_of_raw_data,
            pointer_to_raw_data,
            pointer_to_relocations,
            pointer_to_line_numbers,
            number_of_relocations,
            number_of_line_numbers,
            characteristics,
        }
    }
}
