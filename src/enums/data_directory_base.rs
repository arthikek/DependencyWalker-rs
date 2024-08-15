use crate::enums::dword::DWord;

pub struct ImageDataDirectoryBase {
    virtual_address: DWord,
    size: DWord,
}