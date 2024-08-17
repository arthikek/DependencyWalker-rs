use std::fs::File;
use std::io;
use std::io::Read;
use crate::dto::dos_header::DosHeader;

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
            dos_header_buffer[0..2].try_into().unwrap(),   // e_magic
            dos_header_buffer[2..4].try_into().unwrap(),   // e_cblp
            dos_header_buffer[4..6].try_into().unwrap(),   // e_cp
            dos_header_buffer[6..8].try_into().unwrap(),   // e_crlc
            dos_header_buffer[8..10].try_into().unwrap(),  // e_cparhdr
            dos_header_buffer[10..12].try_into().unwrap(), // e_minalloc
            dos_header_buffer[12..14].try_into().unwrap(), // e_maxalloc
            dos_header_buffer[14..16].try_into().unwrap(), // e_ss
            dos_header_buffer[16..18].try_into().unwrap(), // e_sp
            dos_header_buffer[18..20].try_into().unwrap(), // e_csum
            dos_header_buffer[20..22].try_into().unwrap(), // e_ip
            dos_header_buffer[22..24].try_into().unwrap(), // e_cs
            dos_header_buffer[24..26].try_into().unwrap(), // e_lfarlc
            dos_header_buffer[26..28].try_into().unwrap(), // e_ovno
            [
                dos_header_buffer[28..30].try_into().unwrap(), // e_res[0]
                dos_header_buffer[30..32].try_into().unwrap(), // e_res[1]
                dos_header_buffer[32..34].try_into().unwrap(), // e_res[2]
                dos_header_buffer[34..36].try_into().unwrap(), // e_res[3]
            ],
            dos_header_buffer[36..38].try_into().unwrap(), // e_oemid
            dos_header_buffer[38..40].try_into().unwrap(), // e_oeminfo
            [
                dos_header_buffer[40..42].try_into().unwrap(), // e_res2[0]
                dos_header_buffer[42..44].try_into().unwrap(), // e_res2[1]
                dos_header_buffer[44..46].try_into().unwrap(), // e_res2[2]
                dos_header_buffer[46..48].try_into().unwrap(), // e_res2[3]
                dos_header_buffer[48..50].try_into().unwrap(), // e_res2[4]
                dos_header_buffer[50..52].try_into().unwrap(), // e_res2[5]
                dos_header_buffer[52..54].try_into().unwrap(), // e_res2[6]
                dos_header_buffer[54..56].try_into().unwrap(), // e_res2[7]
                dos_header_buffer[56..58].try_into().unwrap(), // e_res2[8]
                dos_header_buffer[58..60].try_into().unwrap(), // e_res2[9]
            ],
            dos_header_buffer[60..64].try_into().unwrap(), // e_lfanew
        ))
    }

}

