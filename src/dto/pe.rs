use crate::dto::coff_header::CoffHeader;
use crate::dto::dos_header::DosHeader;

#[derive(Debug, Clone)]
pub struct PeDto {
    pub dos_header: DosHeader,
    pub coff_header: CoffHeader,
    pub optional_header: OptionalHeader,
}


