use crate::enums::dword::DWord;
use crate::enums::word::Word;

#[derive(Clone, Debug)]
pub struct CoffHeader{
    pe_signature : DWord,
    machine : Word,
    number_of_sections : Word,
    time_date_stamp : DWord,
    pointer_to_symbol_table: DWord,
    number_of_symbols: DWord,
    size_of_optional_header : Word,
    characteristics: DWord
}
