use std::fs::File;
use std::io;
use std::io::Read;
use crate::dto::dos_header::DosHeader;
use crate::dto::pe::PeDto;
use crate::enums::dword::DWord;
use crate::enums::word::Word;
use crate::parser::pe_mapper::Mapper;

pub struct Parser {
 map : Mapper,
}

impl Parser {

    pub fn new() -> Self{
        let map = Mapper::new();
        Self{
            map
        }
    }
    pub fn parse_dos_header(file: &mut File) -> io::Result<DosHeader> {
        let mut dos_header_buffer = [0u8; 64];
        file.read_exact(&mut dos_header_buffer)?;

        Ok(DosHeader::new(
            Word::from_bytes(&dos_header_buffer[0..2]),
            Word::from_bytes(&dos_header_buffer[2..4]),
            Word::from_bytes(&dos_header_buffer[4..6]),
            Word::from_bytes(&dos_header_buffer[6..8]),
            Word::from_bytes(&dos_header_buffer[8..10]),
            Word::from_bytes(&dos_header_buffer[10..12]),
            Word::from_bytes(&dos_header_buffer[12..14]),
            Word::from_bytes(&dos_header_buffer[14..16]),
            Word::from_bytes(&dos_header_buffer[16..18]),
            Word::from_bytes(&dos_header_buffer[18..20]),
            Word::from_bytes(&dos_header_buffer[20..22]),
            Word::from_bytes(&dos_header_buffer[22..24]),
            Word::from_bytes(&dos_header_buffer[24..26]),
            Word::from_bytes(&dos_header_buffer[26..28]),
            [
                Word::from_bytes(&dos_header_buffer[28..30]),
                Word::from_bytes(&dos_header_buffer[30..32]),
                Word::from_bytes(&dos_header_buffer[32..34]),
                Word::from_bytes(&dos_header_buffer[34..36]),
            ],
            Word::from_bytes(&dos_header_buffer[36..38]),
            Word::from_bytes(&dos_header_buffer[38..40]),
            [
                Word::from_bytes(&dos_header_buffer[40..42]),
                Word::from_bytes(&dos_header_buffer[42..44]),
                Word::from_bytes(&dos_header_buffer[44..46]),
                Word::from_bytes(&dos_header_buffer[46..48]),
                Word::from_bytes(&dos_header_buffer[48..50]),
                Word::from_bytes(&dos_header_buffer[50..52]),
                Word::from_bytes(&dos_header_buffer[52..54]),
                Word::from_bytes(&dos_header_buffer[54..56]),
                Word::from_bytes(&dos_header_buffer[56..58]),
                Word::from_bytes(&dos_header_buffer[58..60]),
            ],
            DWord::from_bytes(&dos_header_buffer[60..64])
        ))
    }
}

