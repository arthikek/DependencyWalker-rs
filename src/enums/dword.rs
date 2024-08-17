/// Represents a 32-bit double word (DWORD) in the PE32 format.
///
/// In the context of the PE (Portable Executable) 32-bit format, a "DWORD"
/// (short for "double word") is conventionally defined as a 32-bit (4-byte)
/// unsigned integer. This is a standard size used throughout PE files for
/// fields that require larger numeric values or addresses.
///
/// The `Dword` enum encapsulates this 32-bit value as a `[u8; 4]` array,
/// ensuring that it conforms to the expected size and allows for easy
/// manipulation within PE structures.
#[derive(Clone, Debug,PartialEq)]
pub enum DWord {
    /// The raw 32-bit value stored as a 4-byte array.
    ///
    /// This variant holds the 32-bit value using a `[u8; 4]` array, which
    /// directly corresponds to the "DWORD" size in a PE32 structure.
    Raw([u8; 4]),
}

impl DWord{
    pub fn from_bytes(bytes: &[u8]) -> DWord{
        let mut array:[u8;4] = [0u8;4];
        if (bytes.len() == 4) {
            array.copy_from_slice(bytes);
        };
        DWord::Raw(array)
    }
}
