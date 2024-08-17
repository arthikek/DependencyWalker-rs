use std::collections::HashMap;
use crate::dto::fieldsoffset::FieldsMapper;

pub struct Mapper {
    m_coff_fields_fields : HashMap<String,FieldsMapper>,
    m_pe32: HashMap<String, FieldsMapper>,
    m_pe32_plus: HashMap<String, FieldsMapper>,
}

impl Mapper {
    pub fn new() -> Self {
        let mut coff_fields = HashMap::new();
        let mut pe32 = HashMap::new();
        let mut pe32plus = HashMap::new();


        coff_fields.insert("Machine".to_string(), FieldsMapper { offset: 0, size: 2 });
        coff_fields.insert("NumberOfSections".to_string(), FieldsMapper { offset: 2, size: 2 });
        coff_fields.insert("TimeDateStamp".to_string(), FieldsMapper { offset: 4, size: 4 });
        coff_fields.insert("PointerToSymbolTable".to_string(), FieldsMapper { offset: 8, size: 4 });
        coff_fields.insert("NumberOfSymbols".to_string(), FieldsMapper { offset: 12, size: 4 });
        coff_fields.insert("SizeOfOptionalHeader".to_string(), FieldsMapper { offset: 16, size: 2 });
        coff_fields.insert("Characteristics".to_string(), FieldsMapper { offset: 18, size: 2 });

        pe32.insert("Magic".to_string(), FieldsMapper { offset: 0, size: 2 });
        pe32.insert("MajorLinkerVersion".to_string(), FieldsMapper { offset: 2, size: 1 });
        pe32.insert("MinorLinkerVersion".to_string(), FieldsMapper { offset: 3, size: 1 });
        pe32.insert("SizeOfCode".to_string(), FieldsMapper { offset: 4, size: 4 });
        pe32.insert("SizeOfInitializedData".to_string(), FieldsMapper { offset: 8, size: 4 });
        pe32.insert("SizeOfUninitializedData".to_string(), FieldsMapper { offset: 12, size: 4 });
        pe32.insert("AddressOfEntryPoint".to_string(), FieldsMapper { offset: 16, size: 4 });
        pe32.insert("BaseOfCode".to_string(), FieldsMapper { offset: 20, size: 4 });
        pe32.insert("BaseOfData".to_string(), FieldsMapper { offset: 24, size: 4 });
        pe32.insert("ImageBase".to_string(), FieldsMapper { offset: 28, size: 4 });
        pe32.insert("SectionAlignment".to_string(), FieldsMapper { offset: 32, size: 4 });
        pe32.insert("FileAlignment".to_string(), FieldsMapper { offset: 36, size: 4 });
        pe32.insert("MajorOperatingSystemVersion".to_string(), FieldsMapper { offset: 40, size: 2 });
        pe32.insert("MinorOperatingSystemVersion".to_string(), FieldsMapper { offset: 42, size: 2 });
        pe32.insert("MajorImageVersion".to_string(), FieldsMapper { offset: 44, size: 2 });
        pe32.insert("MinorImageVersion".to_string(), FieldsMapper { offset: 46, size: 2 });
        pe32.insert("MajorSubsystemVersion".to_string(), FieldsMapper { offset: 48, size: 2 });
        pe32.insert("MinorSubsystemVersion".to_string(), FieldsMapper { offset: 50, size: 2 });
        pe32.insert("Win32VersionValue".to_string(), FieldsMapper { offset: 52, size: 4 });
        pe32.insert("SizeOfImage".to_string(), FieldsMapper { offset: 56, size: 4 });
        pe32.insert("SizeOfHeaders".to_string(), FieldsMapper { offset: 60, size: 4 });
        pe32.insert("CheckSum".to_string(), FieldsMapper { offset: 64, size: 4 });
        pe32.insert("Subsystem".to_string(), FieldsMapper { offset: 68, size: 2 });
        pe32.insert("DllCharacteristics".to_string(), FieldsMapper { offset: 70, size: 2 });
        pe32.insert("SizeOfStackReserve".to_string(), FieldsMapper { offset: 72, size: 4 });
        pe32.insert("SizeOfStackCommit".to_string(), FieldsMapper { offset: 76, size: 4 });
        pe32.insert("SizeOfHeapReserve".to_string(), FieldsMapper { offset: 80, size: 4 });
        pe32.insert("SizeOfHeapCommit".to_string(), FieldsMapper { offset: 84, size: 4 });
        pe32.insert("LoaderFlags".to_string(), FieldsMapper { offset: 88, size: 4 });
        pe32.insert("NumberOfRvaAndSizes".to_string(), FieldsMapper { offset: 92, size: 4 });

        pe32plus.insert("Magic".to_string(), FieldsMapper { offset: 0, size: 2 });
        pe32plus.insert("MajorLinkerVersion".to_string(), FieldsMapper { offset: 2, size: 1 });
        pe32plus.insert("MinorLinkerVersion".to_string(), FieldsMapper { offset: 3, size: 1 });
        pe32plus.insert("SizeOfCode".to_string(), FieldsMapper { offset: 4, size: 4 });
        pe32plus.insert("SizeOfInitializedData".to_string(), FieldsMapper { offset: 8, size: 4 });
        pe32plus.insert("SizeOfUninitializedData".to_string(), FieldsMapper { offset: 12, size: 4 });
        pe32plus.insert("AddressOfEntryPoint".to_string(), FieldsMapper { offset: 16, size: 4 });
        pe32plus.insert("BaseOfCode".to_string(), FieldsMapper { offset: 20, size: 4 });
        pe32plus.insert("ImageBase".to_string(), FieldsMapper { offset: 24, size: 8 });
        pe32plus.insert("SectionAlignment".to_string(), FieldsMapper { offset: 32, size: 4 });
        pe32plus.insert("FileAlignment".to_string(), FieldsMapper { offset: 36, size: 4 });
        pe32plus.insert("MajorOperatingSystemVersion".to_string(), FieldsMapper { offset: 40, size: 2 });
        pe32plus.insert("MinorOperatingSystemVersion".to_string(), FieldsMapper { offset: 42, size: 2 });
        pe32plus.insert("MajorImageVersion".to_string(), FieldsMapper { offset: 44, size: 2 });
        pe32plus.insert("MinorImageVersion".to_string(), FieldsMapper { offset: 46, size: 2 });
        pe32plus.insert("MajorSubsystemVersion".to_string(), FieldsMapper { offset: 48, size: 2 });
        pe32plus.insert("MinorSubsystemVersion".to_string(), FieldsMapper { offset: 50, size: 2 });
        pe32plus.insert("Win32VersionValue".to_string(), FieldsMapper { offset: 52, size: 4 });
        pe32plus.insert("SizeOfImage".to_string(), FieldsMapper { offset: 56, size: 4 });
        pe32plus.insert("SizeOfHeaders".to_string(), FieldsMapper { offset: 60, size: 4 });
        pe32plus.insert("CheckSum".to_string(), FieldsMapper { offset: 64, size: 4 });
        pe32plus.insert("Subsystem".to_string(), FieldsMapper { offset: 68, size: 2 });
        pe32plus.insert("DllCharacteristics".to_string(), FieldsMapper { offset: 70, size: 2 });
        pe32plus.insert("SizeOfStackReserve".to_string(), FieldsMapper { offset: 72, size: 8 });
        pe32plus.insert("SizeOfStackCommit".to_string(), FieldsMapper { offset: 80, size: 8 });
        pe32plus.insert("SizeOfHeapReserve".to_string(), FieldsMapper { offset: 88, size: 8 });
        pe32plus.insert("SizeOfHeapCommit".to_string(), FieldsMapper { offset: 96, size: 8 });
        pe32plus.insert("LoaderFlags".to_string(), FieldsMapper { offset: 104, size: 4 });
        pe32plus.insert("NumberOfRvaAndSizes".to_string(), FieldsMapper { offset: 108, size: 4 });

        Mapper { m_coff_fields_fields:coff_fields, m_pe32:pe32, m_pe32_plus:pe32plus }
    }

    pub fn get_coff_map(&self) -> &HashMap<String,FieldsMapper>{
        &self.m_coff_fields_fields
    }

    pub fn get_pe32_map(&self) -> &HashMap<String, FieldsMapper> {
        &self.m_pe32
    }

    pub fn get_pe32_plus_map(&self) -> &HashMap<String, FieldsMapper> {
        &self.m_pe32_plus
    }


}
