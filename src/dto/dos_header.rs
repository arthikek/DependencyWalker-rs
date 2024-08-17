use crate::enums::dword::DWord;
use crate::enums::word::Word;

#[derive(Debug, Clone)]
pub struct DosHeader {
    e_magic: Word,      // Magic number
    e_cblp: Word,       // Bytes on last page of file
    e_cp: Word,         // Pages in file
    e_crlc: Word,       // Relocations
    e_cparhdr: Word,    // Size of header in paragraphs
    e_minalloc: Word,   // Minimum extra paragraphs needed
    e_maxalloc: Word,   // Maximum extra paragraphs needed
    e_ss: Word,         // Initial (relative) SS value
    e_sp: Word,         // Initial SP value
    e_csum: Word,       // Checksum
    e_ip: Word,         // Initial IP value
    e_cs: Word,         // Initial (relative) CS value
    e_lfarlc: Word,     // File address of relocation table
    e_ovno: Word,       // Overlay number
    e_res: [Word; 4],   // Reserved words
    e_oemid: Word,      // OEM identifier (for e_oeminfo)
    e_oeminfo: Word,    // OEM information; e_oemid specific
    e_res2: [Word; 10], // Reserved words
    e_lfanew: DWord,    // File address of new exe header
}

impl DosHeader {
    pub fn new(
        e_magic: Word,
        e_cblp: Word,
        e_cp: Word,
        e_crlc: Word,
        e_cparhdr: Word,
        e_minalloc: Word,
        e_maxalloc: Word,
        e_ss: Word,
        e_sp: Word,
        e_csum: Word,
        e_ip: Word,
        e_cs: Word,
        e_lfarlc: Word,
        e_ovno: Word,
        e_res: [Word; 4],
        e_oemid: Word,
        e_oeminfo: Word,
        e_res2: [Word; 10],
        e_lfanew: DWord,
    ) -> Self {
        Self {
            e_magic,
            e_cblp,
            e_cp,
            e_crlc,
            e_cparhdr,
            e_minalloc,
            e_maxalloc,
            e_ss,
            e_sp,
            e_csum,
            e_ip,
            e_cs,
            e_lfarlc,
            e_ovno,
            e_res,
            e_oemid,
            e_oeminfo,
            e_res2,
            e_lfanew,
        }
    }

    pub fn e_magic(&self) -> &Word {
        &self.e_magic
    }

    pub fn e_cblp(&self) -> &Word {
        &self.e_cblp
    }

    pub fn e_cp(&self) -> &Word {
        &self.e_cp
    }

    pub fn e_crlc(&self) -> &Word {
        &self.e_crlc
    }

    pub fn e_cparhdr(&self) -> &Word {
        &self.e_cparhdr
    }

    pub fn e_minalloc(&self) -> &Word {
        &self.e_minalloc
    }

    pub fn e_maxalloc(&self) -> &Word {
        &self.e_maxalloc
    }

    pub fn e_ss(&self) -> &Word {
        &self.e_ss
    }

    pub fn e_sp(&self) -> &Word {
        &self.e_sp
    }

    pub fn e_csum(&self) -> &Word {
        &self.e_csum
    }

    pub fn e_ip(&self) -> &Word {
        &self.e_ip
    }

    pub fn e_cs(&self) -> &Word {
        &self.e_cs
    }

    pub fn e_lfarlc(&self) -> &Word {
        &self.e_lfarlc
    }

    pub fn e_ovno(&self) -> &Word {
        &self.e_ovno
    }

    pub fn e_res(&self) -> &[Word; 4] {
        &self.e_res
    }

    pub fn e_oemid(&self) -> &Word {
        &self.e_oemid
    }

    pub fn e_oeminfo(&self) -> &Word {
        &self.e_oeminfo
    }

    pub fn e_res2(&self) -> &[Word; 10] {
        &self.e_res2
    }

    pub fn e_lfanew(&self) -> &DWord {
        &self.e_lfanew
    }
}
