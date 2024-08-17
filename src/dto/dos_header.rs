use crate::{DWord, Word};

#[derive(Debug, Clone)]
pub struct DosHeader {
    e_magic: [u8; 2],      // Magic number
    e_cblp: [u8; 2],       // Bytes on last page of file
    e_cp: [u8; 2],         // Pages in file
    e_crlc: [u8; 2],       // Relocations
    e_cparhdr: [u8; 2],    // Size of header in paragraphs
    e_minalloc: [u8; 2],   // Minimum extra paragraphs needed
    e_maxalloc: [u8; 2],   // Maximum extra paragraphs needed
    e_ss: [u8; 2],         // Initial (relative) SS value
    e_sp: [u8; 2],         // Initial SP value
    e_csum: [u8; 2],       // Checksum
    e_ip: [u8; 2],         // Initial IP value
    e_cs: [u8; 2],         // Initial (relative) CS value
    e_lfarlc: [u8; 2],     // File address of relocation table
    e_ovno: [u8; 2],       // Overlay number
    e_res: [[u8; 2]; 4],   // Reserved words
    e_oemid: [u8; 2],      // OEM identifier (for e_oeminfo)
    e_oeminfo: [u8; 2],    // OEM information; e_oemid specific
    e_res2: [[u8; 2]; 10], // Reserved words
    e_lfanew: [u8; 4],     // File address of new exe header
}

impl DosHeader {
    pub fn new(
        e_magic: [u8; 2],
        e_cblp: [u8; 2],
        e_cp: [u8; 2],
        e_crlc: [u8; 2],
        e_cparhdr: [u8; 2],
        e_minalloc: [u8; 2],
        e_maxalloc: [u8; 2],
        e_ss: [u8; 2],
        e_sp: [u8; 2],
        e_csum: [u8; 2],
        e_ip: [u8; 2],
        e_cs: [u8; 2],
        e_lfarlc: [u8; 2],
        e_ovno: [u8; 2],
        e_res: [[u8; 2]; 4],
        e_oemid: [u8; 2],
        e_oeminfo: [u8; 2],
        e_res2: [[u8; 2]; 10],
        e_lfanew: [u8; 4],
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

    pub fn e_magic(&self) -> [u8; 2] {
        self.e_magic
    }

    pub fn e_cblp(&self) -> [u8; 2] {
        self.e_cblp
    }

    pub fn e_cp(&self) -> [u8; 2] {
        self.e_cp
    }

    pub fn e_crlc(&self) -> [u8; 2] {
        self.e_crlc
    }

    pub fn e_cparhdr(&self) -> [u8; 2] {
        self.e_cparhdr
    }

    pub fn e_minalloc(&self) -> [u8; 2] {
        self.e_minalloc
    }

    pub fn e_maxalloc(&self) -> [u8; 2] {
        self.e_maxalloc
    }

    pub fn e_ss(&self) -> [u8; 2] {
        self.e_ss
    }

    pub fn e_sp(&self) -> [u8; 2] {
        self.e_sp
    }

    pub fn e_csum(&self) -> [u8; 2] {
        self.e_csum
    }

    pub fn e_ip(&self) -> [u8; 2] {
        self.e_ip
    }

    pub fn e_cs(&self) -> [u8; 2] {
        self.e_cs
    }

    pub fn e_lfarlc(&self) -> [u8; 2] {
        self.e_lfarlc
    }

    pub fn e_ovno(&self) -> [u8; 2] {
        self.e_ovno
    }

    pub fn e_res(&self) -> [[u8; 2]; 4] {
        self.e_res
    }

    pub fn e_oemid(&self) -> [u8; 2] {
        self.e_oemid
    }

    pub fn e_oeminfo(&self) -> [u8; 2] {
        self.e_oeminfo
    }

    pub fn e_res2(&self) -> [[u8; 2]; 10] {
        self.e_res2
    }

    pub fn e_lfanew(&self) -> [u8; 4] {
        self.e_lfanew
    }
}
