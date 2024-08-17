use crate::enums::dword::DWord;

pub struct DataDirectoryBase {
    virtual_address: DWord,
    size: DWord,
}
