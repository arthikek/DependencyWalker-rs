/// Represents a 16-bit word in the PE32 format.
///
/// In the context of the PE (Portable Executable) 32-bit format, a "word"
/// is conventionally defined as a 16-bit (2-byte) unsigned integer. This
/// is a historical standard that is consistently used across PE files.
///
/// The `Word` enum encapsulates this 16-bit value as a `[u8; 2]` array,
/// allowing for easy handling of these values while ensuring they conform
/// to the expected size.
#[derive(Clone, Debug,PartialEq)]
pub enum Word {
    /// The raw 16-bit value stored as a 2-byte array.
    ///
    /// This variant holds the 16-bit value using a `[u8; 2]` array, which
    /// directly corresponds to the "word" size in a PE32 structure.
    Raw([u8; 2]),
}
impl Word {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut array = [0u8; 2];
        if bytes.len() == 2 {
            array.copy_from_slice(bytes);
        }
        Word::Raw(array)
    }
}
