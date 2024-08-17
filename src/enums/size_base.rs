use crate::{DWord, QWord};

/// Represents the possible sizes of certain fields in the PE (Portable Executable) format.
/// The PE format has two variants: PE32 and PE32+.
/// - In PE32, some fields are 32-bit (Double Word or DWord).
/// - In PE32+, those same fields might be 64-bit (Quad Word or QWord).
#[derive(Clone, Debug)]
pub enum SizeBase {
    /// DWord (Double Word) represents a 32-bit field.
    /// This is used in the PE32 format.
    DWord(DWord),

    /// QWord (Quad Word) represents a 64-bit field.
    /// This is used in the PE32+ format.
    QWord(QWord),
}
