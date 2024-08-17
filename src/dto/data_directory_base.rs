use crate::DWord;

pub struct DataDirectoryBase {
    virtual_address: DWord,
    size: DWord,
}
