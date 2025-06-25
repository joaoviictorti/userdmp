use binrw::Error as BinrwError;
use thiserror::Error;

/// Represents errors that may occur during the processing of a minidump file.
#[derive(Debug, Error)]
pub enum UserDmpError {
    /// Raised when the application fails to open a file.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The underlying `std::io::Error` providing details about the failure.
    #[error("Failed to open file: {0}")]
    FileOpenError(#[from] std::io::Error),

    /// Raised when the minidump contains an invalid signature.
    #[error("Invalid minidump signature.")]
    InvalidSignature,

    /// Raised when the minidump contains invalid or unsupported flags.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The invalid or unsupported flags as a hexadecimal value.
    #[error("The minidump contains invalid or unsupported flags: {0:#x}")]
    InvalidFlags(u64),

    /// Raised when the minidump specifies an unsupported architecture.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The architecture identifier as a 16-bit integer.
    #[error("Unsupported architecture: {0}")]
    UnsupportedArchitecture(u16),

    /// Raised when the application fails to parse the system information in the minidump.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The underlying `std::io::Error` encountered during parsing.
    #[error("Failed to parse system info: {0}")]
    ParseSystemInfoError(std::io::Error),

    /// Raised when the application fails to parse the module list in the minidump.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The underlying `std::io::Error` encountered during parsing.
    #[error("Failed to parse module list: {0}")]
    ParseModuleListError(std::io::Error),

    /// Raised when the minidump contains a module with an invalid memory range.
    #[error("Invalid memory range in module.")]
    InvalidMemoryRange,

    /// Raised when the application fails to create a file mapping for the minidump.
    #[error("Failed to create file mapping.")]
    CreateFileMappingError,

    /// Raised when the application fails to map a view of the minidump file (Windows).
    #[error("Failed to map view of file.")]
    MapViewOfFileError,

    /// Raised when the application fails to map a view of the minidump file (Unix).
    #[error("Failed to map view of file.")]
    MmapError,

    /// Raised when a parsing error occurs in the `binrw` library.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The error produced by the `binrw` library.
    #[error("Parsing error: {0}")]
    BinrwError(#[from] BinrwError),

    /// Raised when an address cannot be found in the `Memory64ListStream`.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The address that could not be found, represented as a hexadecimal value.
    #[error("Address {0:#x?} was not found in Memory64ListStream")]
    AddressNotFound(u64),

    /// Raised when the context is invalid.
    ///
    /// # Arguments
    ///
    /// * `{0}` - The size of the context that was invalid.
    #[error("Invalid context")]
    InvalidContext,
}
