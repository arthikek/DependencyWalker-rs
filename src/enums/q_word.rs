/// Represents a 64-bit quad word (Qword) in the PE32+ format.
///
/// In the context of the PE (Portable Executable) 32-bit format, a "Qword"
/// is conventionally defined as a 64-bit (8-byte) unsigned integer. This is a
/// standard size used throughout PE32+ files for fields that require larger numeric
/// values or addresses.
///
/// The `Qword` enum encapsulates this 64-bit value as a `[u8; 8]` array,
/// ensuring that it conforms to the expected size and allows for easy
/// manipulation within PE structures.
#[derive(Clone, Debug)]
pub enum QWord {
    /// The raw 64-bit value stored as an 8-byte array.
    ///
    /// This variant holds the 64-bit value using a `[u8; 8]` array, which
    /// directly corresponds to the "Qword" size in a PE32+ structure.
    Raw([u8; 8]),
}
