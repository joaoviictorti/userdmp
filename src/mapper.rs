use std::{
    fs::File,
    path::Path,
    io,
    ffi::c_void
};
use super::error::UserDmpError;

/// Represents a memory-mapped file.
/// This struct provides an abstraction for mapping a file into memory
/// and accessing it as a slice of bytes.
#[derive(Debug)]
pub struct MappingFile<'a> {
    /// A slice representing the memory-mapped contents of the file.
    pub buffer: &'a [u8],

    /// The base address of the memory-mapped file in the process's address space.
    pub address: *mut c_void,
}

impl<'a> MappingFile<'a> {
    /// Creates a new `MappingFile` instance by mapping the contents of a file into memory.
    ///
    /// # Arguments
    ///
    /// * `path` - A reference to the path of the file to be mapped.
    ///
    /// # Returns
    ///
    /// A `MappingFile` instance containing the memory-mapped contents of the file.
    /// 
    /// # Example
    ///
    /// ```rust,ignore
    /// use std::path::Path;
    /// use userdmp::{MappingFile, UserDmpError};
    ///
    /// fn main() -> Result<(), UserDmpError> {
    ///     let path = Path::new("example.dmp");
    ///
    ///     // Create a memory-mapped file.
    ///     let mapped_file = MappingFile::new(path)?;
    ///     println!("Memory-mapped file created. Size: {}", mapped_file.buffer.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(path: &Path) -> Result<Self, UserDmpError> {
        let file = File::open(path)?;
        let (buffer, address) = mapper::map_file(file)?;
        Ok(Self { buffer, address })
    }

    /// Creates a cursor for the memory-mapped file buffer.
    ///
    /// # Returns
    ///
    /// An `io::Cursor` that wraps the memory-mapped file's buffer,
    /// allowing for efficient reading.
    /// 
    /// # Example
    ///
    /// ```rust,ignore
    /// use std::path::Path;
    /// use userdmp::{MappingFile, UserDmpError};
    ///
    /// fn main() -> Result<(), UserDmpError> {
    ///     let path = Path::new("example.dmp");
    ///
    ///     // Create a memory-mapped file and a cursor for it.
    ///     let mapped_file = MappingFile::new(path)?;
    ///     let mut cursor = mapped_file.cursor();
    ///
    ///     // Read the first 4 bytes of the mapped file as a u32.
    ///     let mut buffer = [0u8; 4];
    ///     cursor.read_exact(&mut buffer)?;
    ///     let value = u32::from_le_bytes(buffer);
    ///
    ///     println!("First value in file: {}", value);
    ///     Ok(())
    /// }
    /// ```
    pub fn cursor(&self) -> io::Cursor<&'a [u8]> {
        io::Cursor::new(self.buffer)
    }
}

impl Drop for MappingFile<'_> {
    fn drop(&mut self) {
        if !self.address.is_null() {
            #[cfg(windows)] {
                use windows_sys::Win32::System::Memory::{UnmapViewOfFile, MEMORY_MAPPED_VIEW_ADDRESS};

                // Create a MEMORY_MAPPED_VIEW_ADDRESS struct with the correct value.
                let address = MEMORY_MAPPED_VIEW_ADDRESS {
                    Value: self.address,
                };

                // SAFETY: UnmapViewOfFile is called with a valid mapped address.
                unsafe { UnmapViewOfFile(address); }
            }

            #[cfg(unix)] {
                // SAFETY: munmap is called with a valid mapped address and size.
                unsafe { libc::munmap(self.address, self.buffer.len()); }
            }
        }
    }
}

mod mapper {
    use super::{File, UserDmpError};
    use std::{ptr, slice, ffi::c_void};

    /// Maps a file into memory and retrieves its memory buffer and base address (Windows).
    ///
    /// # Arguments
    ///
    /// * `file` - A `File` instance representing the file to be mapped.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// * A slice of the memory-mapped file contents.
    /// * The base address of the memory-mapped file in the process's address space.
    /// 
    /// # Example
    ///
    /// ```rust,ignore
    /// use std::fs::File;
    /// use userdmp::mapper;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let file = File::open("example.dmp")?;
    ///
    ///     // Directly map the file on Windows.
    ///     let (buffer, address) = mapper::map_file(file)?;
    ///     println!("Mapped {} bytes at address {:?}", buffer.len(), address);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cfg(windows)]
    pub fn map_file(file: File) -> Result<(&'static [u8], *mut c_void), UserDmpError> {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::{
            Foundation::CloseHandle,
            System::Memory::{
                CreateFileMappingA, MapViewOfFile, FILE_MAP_READ, PAGE_READONLY,
            },
        };

        // Get the raw file handle.
        let h_file = file.as_raw_handle();

        // Create a memory mapping for the file.
        let h_mapping = unsafe {
            CreateFileMappingA(
                h_file,
                ptr::null_mut(),
                PAGE_READONLY,
                0,
                0,
                ptr::null_mut(),
            )
        };

        // Return an error if the file mapping creation failed.
        if h_mapping.is_null() {
            return Err(UserDmpError::CreateFileMappingError);
        }

        // Get the file size and map the view of the file.
        let size = file.metadata()?.len() as usize;
        let base_address = unsafe { MapViewOfFile(h_mapping, FILE_MAP_READ, 0, 0, size) };
        
        // Return an error if mapping the view failed.
        if base_address.Value.is_null() {
            unsafe { CloseHandle(h_mapping) };
            return Err(UserDmpError::MapViewOfFileError);
        }

        // Close the file mapping handle; the view remains valid.
        unsafe { CloseHandle(h_mapping) };

        // Return the memory-mapped buffer and its base address.
        unsafe {
            Ok((
                slice::from_raw_parts(base_address.Value as *const u8, size),
                base_address.Value,
            ))
        }
    }

    /// Maps a file into memory and retrieves its memory buffer and base address (Unix).
    ///
    /// # Arguments
    ///
    /// * `file` - A `File` instance representing the file to be mapped.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// * A slice of the memory-mapped file contents.
    /// * The base address of the memory-mapped file in the process's address space.
    /// 
    /// # Example
    ///
    /// ```rust,ignore
    /// use std::fs::File;
    /// use userdmp::mapper;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let file = File::open("example.dmp")?;
    ///
    ///     // Directly map the file on Unix.
    ///     let (buffer, address) = mapper::map_file(file)?;
    ///     println!("Mapped {} bytes at address {:?}", buffer.len(), address);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cfg(unix)]
    pub fn map_file(file: File) -> Result<(&'static [u8], *mut c_void), UserDmpError> {
        use libc::{mmap, MAP_FAILED, MAP_SHARED, PROT_READ};
        use std::os::unix::io::AsRawFd;

        // Get the raw file descriptor.
        let fd = file.as_raw_fd();

        // Get the file size.
        let size = file.metadata()?.len() as usize;

        // Create a memory mapping for the file.
        let base_address = unsafe {
            mmap(
                ptr::null_mut(),
                size,
                PROT_READ,
                MAP_SHARED,
                fd,
                0,
            )
        };

         // Return an error if the mapping failed.
        if base_address == MAP_FAILED {
            return Err(UserDmpError::MmapError);
        }

        // Return the memory-mapped buffer and its base address.
        unsafe {
            Ok((
                slice::from_raw_parts(base_address as *const u8, size),
                base_address,
            ))
        }
    }
}