use crate::dto::coff_header::CoffHeader;
use crate::dto::dos_header::DosHeader;
use crate::dto::optional_header::OptionalHeader;
use crate::dto::section_header::SectionHeader;

#[derive(Debug, Clone)]
pub struct PeDto {
    m_dos_header: DosHeader, // The DOS header is 64 bytes for the standard PE format, which includes the DOS stub and a pointer to the PE header.
    m_coff_header: CoffHeader, // The COFF header (also known as the File header) is 20 bytes in size in the standard PE format.
    m_optional_header: OptionalHeader, // The size of the Optional Header is specified by the `SizeOfOptionalHeader` field in the COFF header. It varies but is typically 224 bytes for PE32 and 240 bytes for PE32+.
    m_section_table: Vec<SectionHeader>, // The size of the Section Table is determined by the `NumberOfSections` field in the COFF header, with each `SectionHeader` being 40 bytes. Therefore, total size = NumberOfSections * 40 bytes.
}

impl PeDto {
    pub fn new(
        m_dos_header: DosHeader,
        m_coff_header: CoffHeader,
        m_optional_header: OptionalHeader,
        m_section_table: Vec<SectionHeader>,
    ) -> Self {
        Self {
            m_dos_header,
            m_coff_header,
            m_optional_header,
            m_section_table,
        }
    }
}
