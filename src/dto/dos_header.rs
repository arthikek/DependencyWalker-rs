use crate::enums::dword::DWord;
use crate::enums::word::Word;

#[derive(Debug,Clone)]
pub struct DosHeader {
    e_magic: Word,       // Magic number
    e_cblp: Word,        // Bytes on last page of file
    e_cp: Word,          // Pages in file
    e_crlc: Word,        // Relocations
    e_cparhdr: Word,     // Size of header in paragraphs
    e_minalloc: Word,    // Minimum extra paragraphs needed
    e_maxalloc: Word,    // Maximum extra paragraphs needed
    e_ss: Word,          // Initial (relative) SS value
    e_sp: Word,          // Initial SP value
    e_csum: Word,        // Checksum
    e_ip: Word,          // Initial IP value
    e_cs: Word,          // Initial (relative) CS value
    e_lfarlc: Word,      // File address of relocation table
    e_ovno: Word,        // Overlay number
    e_res: [Word; 4],    // Reserved words
    e_oemid: Word,       // OEM identifier (for e_oeminfo)
    e_oeminfo: Word,     // OEM information; e_oemid specific
    e_res2: [Word; 10],  // Reserved words
    e_lfanew: DWord,     // File address of new exe header
}