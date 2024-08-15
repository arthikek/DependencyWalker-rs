/// Represents a 8-bit half word (Halfword) in the PE32 format.
///
/// In the context of the PE (Portable Executable) 32-bit format, a "Halfword"
/// is conventionally defined as an 8-bit (1-byte) unsigned integer. This is a
/// standard size used throughout PE files for fields that require smaller numeric
/// values or addresses.
///
/// The `Halfword` enum encapsulates this 8-bit value as a `[u8; 1]` array,
/// ensuring that it conforms to the expected size and allows for easy
/// manipulation within PE structures.
#[derive(Clone, Debug)]
pub enum HalfWord {
    /// The raw 8-bit value stored as a 1-byte array.
    ///
    /// This variant holds the 8-bit value using a `[u8; 1]` array, which
    /// directly corresponds to the "Halfword" size in a PE32 structure.
    Raw([u8; 1]),
}
