use std::{
    collections::BTreeMap,
    io::{self, Cursor, Seek},
    path::Path,
    ptr::{self},
};
use binrw::BinRead;
use crate::mapper::MappingFile;
use crate::error::UserDmpError;
use crate::data::{
    MINIDUMP_STREAM_TYPE::{self, *},
    *,
};

/// Represents the modules in a minidump file, mapped by their starting memory address.
pub type Modules<'a> = BTreeMap<u64, Module<'a>>;

/// Represents the threads in a minidump file, mapped by their thread IDs.
pub type Threads = BTreeMap<u32, Thread>;

/// Represents the handles in a minidump file, mapped by their handle values.
pub type Handles = BTreeMap<u64, Handle>;

/// Represents memory regions in a minidump file, mapped by their base addresses.
pub type Memorys<'a> = BTreeMap<u64, Memory<'a>>;

// Type of error
pub type Result<T> = std::result::Result<T, UserDmpError>;

/// Represents the processor architecture of the captured process.
#[derive(Copy, Debug, Clone, Default)]
pub enum Arch {
    // 64-bit architecture
    #[default]
    X64,

    // 32-bit architecture
    X86,
}

/// Trait to represent the parsing of generic streams in a minidump file.
pub trait MinidumpStream<'a> {
    /// Defines the type of output expected from the parser.
    type Output;

    /// Processes the stream and returns the corresponding output type.
    ///
    /// # Arguments
    ///
    /// * `cursor` - A mutable reference to a cursor pointing to the stream's binary data.
    ///
    /// # Returns
    ///
    /// * `Ok(Self::Output)` - The parsed output of the stream.
    /// * `Err(UserDmpError)` - An error indicating the failure of the parsing process.
    fn parse(cursor: &mut Cursor<&'a [u8]>) -> Result<Self::Output>;
}

/// Represents a parsed minidump file, containing metadata, modules, and threads.
#[derive(Debug)]
pub struct UserDump<'a> {
    /// Indicates that it is the ID of the thread directly related to the exception.
    pub exception_thread_id: Option<u32>,

    // System information on the dump
    pub system: System,

    /// The list of modules in the captured process.
    modules: Modules<'a>,

    /// The list of threads in the captured process.
    threads: Threads,

    /// The list of memorys in the captured process.
    memorys: Memorys<'a>,

    /// The list of handles in the captured process.
    handles: Handles,

    /// Mapped file information.
    pub mapped_file: MappingFile<'a>,
}

impl<'a> UserDump<'a> {
    /// Creates a new [`UserDump`] by parsing a minidump file from the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the minidump file.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the file is parsed successfully.
    /// * `Err(UserDmpError)` - If an error occurs during parsing.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use userdmp::{UserDump};
    ///
    /// match UserDump::new("example.dmp") {
    ///     Ok(dump) => println!("Successfully parsed minidump."),
    ///     Err(e) => eprintln!("Failed to parse minidump: {:?}", e),
    /// }
    /// ```
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        // Mapping the file in memory to the target environment (Windows or Linux).
        let mapped_file = MappingFile::new(path)?;
        Self::parse(mapped_file)
    }

    /// Returns a reference to the list of threads in the parsed minidump.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use userdmp::UserDump;
    ///
    /// let dump = UserDump::new("example.dmp").unwrap();
    /// for (thread_id, thread) in dump.threads() {
    ///     println!("Thread ID: {}, Priority: {}", thread_id, thread.priority);
    /// }
    /// ```
    pub fn threads(&self) -> &Threads {
        &self.threads
    }

    /// Returns a reference to the list of modules in the parsed minidump
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use userdmp::UserDump;
    ///
    /// let dump = UserDump::new("example.dmp").unwrap();
    /// for (base_address, module) in dump.modules() {
    ///     println!(
    ///         "Module: {}, Base Address: 0x{:x}, Size: {} bytes",
    ///         module.name().unwrap_or("Unknown"),
    ///         base_address,
    ///         module.len()
    ///     );
    /// }
    /// ```
    pub fn modules(&self) -> &Modules {
        &self.modules
    }

    /// Returns a reference to the list of memory in the parsed minidump
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use userdmp::UserDump;
    ///
    /// let dump = UserDump::new("example.dmp").unwrap();
    /// for (base_address, memory) in dump.memorys() {
    ///     println!(
    ///         "Memory Region: Base Address: 0x{:x}, Size: {} bytes",
    ///         base_address,
    ///         memory.len()
    ///     );
    /// }
    /// ```
    pub fn memorys(&self) -> &Memorys {
        &self.memorys
    }

    /// Returns a reference to the list of handles in the parsed minidump.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use userdmp::UserDump;
    ///
    /// let dump = UserDump::new("example.dmp").unwrap();
    /// for (handle_id, handle) in dump.handles() {
    ///     println!(
    ///         "Handle ID: 0x{}, Type: {:?}, Object Name: {:?}, Attributes: {}, Access: {}",
    ///         handle.handle(),
    ///         handle.type_name(),
    ///         handle.object_name(),
    ///         handle.attributes,
    ///         handle.granted_access
    ///     );
    /// }
    /// ```
    pub fn handles(&self) -> &Handles {
        &self.handles
    }

    /// Parses a specific stream type from a minidump file using the `MinidumpStream` trait.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The stream type to parse. Must implement the `MinidumpStream` trait.
    ///
    /// # Arguments
    ///
    /// * `cursor` - A mutable reference to a cursor positioned within the minidump file.
    ///   The cursor provides access to the binary data of the stream to be parsed.
    ///
    /// # Returns
    ///
    /// * `Ok(S::Output)` - The parsed result for the specific stream type.
    /// * `Err(UserDmpError)` - An error indicating that the parsing failed.
    fn parse_stream<S>(cursor: &mut Cursor<&'a [u8]>) -> Result<S::Output>
    where
        S: MinidumpStream<'a>,
    {
        S::parse(cursor)
    }

    /// Parses a minidump file into a [`UserDump`] structure.
    ///
    /// # Arguments
    ///
    /// * `mapped_file` - The memory-mapped minidump file.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the file is parsed successfully.
    /// * `Err(UserDmpError)` - If the file format is invalid or if parsing fails.
    fn parse(mapped_file: MappingFile<'a>) -> Result<Self> {
        // Creates a cursor to navigate the mapped file.
        let mut cursor = mapped_file.cursor();

        // Reads minidump header.
        let header = MINIDUMP_HEADER::read(&mut cursor)?;

        // Does the file provided have a minidump signature?
        if header.Signature != MINIDUMP_SIGNATURE {
            return Err(UserDmpError::InvalidSignature);
        }

        // Checks if at least one of the bits defined in DUMP_FLAGS (0x001f_ffff)
        // is present in the Flags field of the header. These bits correspond to:
        // - 0x00000001: Includes data sections of loaded modules (MiniDumpWithDataSegs).
        // - 0x00000002: Includes the full memory of the process (MiniDumpWithFullMemory).
        // - 0x00000004: Includes handle information (MiniDumpWithHandleData).
        // - 0x00000008: Filters out unused memory (MiniDumpFilterMemory).
        // - 0x00000010: Adds directly referenced memory (MiniDumpScanMemory).
        // - 0x00000020: Includes unloaded modules (MiniDumpWithUnloadedModules).
        // - 0x00000040: Includes indirectly referenced memory (MiniDumpWithIndirectlyReferencedMemory).
        // - 0x00000200: Adds private read/write memory regions (MiniDumpWithPrivateReadWriteMemory).
        // - 0x00000800: Adds detailed memory information (MiniDumpWithFullMemoryInfo).
        // - 0x00001000: Includes detailed thread information (MiniDumpWithThreadInfo).
        // - 0x00002000: Includes code segments from modules (MiniDumpWithCodeSegs).
        if (header.Flags & DUMP_FLAGS) != 0 {
            return Err(UserDmpError::InvalidFlags(header.Flags));
        }

        // Seeks to the stream directory.
        cursor.seek(io::SeekFrom::Start(header.StreamDirectoryRva.into()))?;

        // Collects all valid streams from the stream directory.
        let mut streams = (0..header.NumberOfStreams)
            .filter_map(|_| {
                let stream = MINIDUMP_DIRECTORY::read(&mut cursor).ok()?;
                (stream.StreamType != UnusedStream as u32).then_some(stream)
            })
            .collect::<Vec<MINIDUMP_DIRECTORY>>();

        // Sort streams by their StreamType in descending order to ensure
        // that higher priority or dependent streams are processed first.
        streams.sort_by_key(|stream| std::cmp::Reverse(stream.StreamType));

        let mut system = System::default();
        let mut modules = Modules::new();
        let mut threads = Threads::new();
        let mut memory_info = Memorys::new();
        let mut memory64 = Memorys::new();
        let mut handles = Handles::new();
        let mut exception_thread_id = None;

        // Processes each stream based on its type.
        for stream in &streams {
            // Seeks to the stream data.
            cursor.seek(io::SeekFrom::Start(stream.Location.RVA.into()))?;

            match MINIDUMP_STREAM_TYPE::try_from(stream.StreamType) {
                Ok(SystemInfoStream) => system = Self::parse_stream::<System>(&mut cursor)?,
                Ok(ModuleListStream) => modules = Self::parse_stream::<Module>(&mut cursor)?,
                Ok(HandleDataStream) => handles = Self::parse_stream::<Handle>(&mut cursor)?,
                Ok(ExceptionStream) => exception_thread_id = Some(Self::parser_exception(&mut cursor)?),
                Ok(ThreadListStream) => threads = Thread::parse(&mut cursor, &Some(system.processor_architecture))?,
                Ok(MemoryInfoListStream) => memory_info = Memory::parser_memory_info(&mut cursor)?,
                Ok(Memory64ListStream) => memory64 = Memory::parser_memory64_list(&mut cursor)?,
                _ => {}
            }
        }

        // Merges two maps of memory regions into a single map.
        let memorys = Memory::merge_memory(memory_info, memory64)?;

        // Returns the parsed UserDump.
        Ok(Self {
            exception_thread_id,
            system,
            modules,
            threads,
            memorys,
            handles,
            mapped_file,
        })
    }

    /// Parses the exception information from the `ExceptionStream`.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Cursor positioned at the exception stream.
    ///
    /// # Returns
    ///
    /// * `Ok(u32)` - The thread ID associated with the exception.
    /// * `Err(UserDmpError)` - If an error occurs during parsing.
    fn parser_exception(cursor: &mut Cursor<&'a [u8]>) -> Result<u32> {
        // Reads the exception stream.
        let exception = MINIDUMP_EXCEPTION_STREAM::read(cursor)?;

        // Returns the associated thread ID.
        Ok(exception.ThreadId)
    }

    /// Extracts raw data from a [`MINIDUMP_LOCATION_DESCRIPTOR`].
    ///
    /// # Arguments
    ///
    /// * `cursor` - Cursor to read data from.
    /// * `location` - The descriptor indicating where the data is located.
    ///
    /// # Returns
    ///
    /// * `Ok(&'a [u8])` - A slice containing the raw data.
    /// * `Err(io::Error)` - If the data extraction fails.
    fn extract_raw_data(cursor: &Cursor<&'a [u8]>, location: MINIDUMP_LOCATION_DESCRIPTOR) -> io::Result<&'a [u8]> {
        // Reads the RVA.
        let rva = location.RVA;

        // Reads the size of the data.
        let size = location.DataSize;

        // Splits the slice at the RVA.
        let slice = cursor.get_ref();
        let (_, tail) = slice.split_at(rva as usize);

        // Returns the extracted slice.
        Ok(&tail[..size as usize])
    }
}

// Represents the system information captured in the minidump.
/// The [`System`] struct contains details about the processor architecture,
/// operating system version, and other general system information useful
/// for analyzing the minidump.
#[derive(Copy, Debug, Clone, Default)]
pub struct System {
    /// The processor architecture captured in the minidump (e.g., x86 or x64).
    pub processor_architecture: Arch,

    /// The processor level.
    pub processor_level: u16,

    /// The processor revision.
    pub processor_revision: u16,

    /// The number of processors in the captured system.
    pub number_of_processors: u8,

    /// The product type of the operating system.
    pub product_type: u8,

    /// The major version of the operating system.
    pub major_version: u32,

    /// The minor version of the operating system.
    pub minor_version: u32,

    /// The build number of the operating system.
    pub build_number: u32,

    /// The platform identifier of the operating system.
    pub platform_id: u32,
}

impl MinidumpStream<'_> for System {
    type Output = System;

    /// Parses the system information from the `SystemInfoStream`.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Cursor positioned at the system info stream.
    ///
    /// # Returns
    ///
    /// * `Ok(Modules<'a>)` - If the system are parsed successfully.
    /// * `Err(UserDmpError)` - If an error occurs during parsing.
    fn parse(cursor: &mut Cursor<&'_ [u8]>) -> Result<Self::Output> {
        // Reads the system info stream.
        let system_info = MINIDUMP_SYSTEM_INFO::read(cursor)?;

        // Converts MINIDUMP_SYSTEM_INFO into System.
        Ok(System::from(system_info))
    }
}

impl From<MINIDUMP_SYSTEM_INFO> for System {
    /// Converts a `MINIDUMP_SYSTEM_INFO` structure into a `System` instance.
    ///
    /// # Parameters
    ///
    /// * `info` - A [`MINIDUMP_SYSTEM_INFO`] instance containing the raw data
    ///   extracted from the system information stream.
    ///
    /// # Returns
    ///
    /// * A new [`System`] instance populated with data from the [`MINIDUMP_SYSTEM_INFO`].
    fn from(info: MINIDUMP_SYSTEM_INFO) -> Self {
        Self {
            processor_architecture: match info.ProcessorArchitecture {
                ARCH_X64 => Arch::X64,
                ARCH_X86 => Arch::X86,
                _ => panic!("Unsupported architecture: {:x}", info.ProcessorArchitecture),
            },
            processor_level: info.ProcessorLevel,
            processor_revision: info.ProcessorRevision,
            number_of_processors: info.NumberOfProcessors,
            product_type: info.ProductType,
            major_version: info.MajorVersion,
            minor_version: info.MinorVersion,
            build_number: info.BuildNumber,
            platform_id: info.PlatformId,
        }
    }
}

/// Represents a module loaded in a process, including its memory range, checksum, path,
/// timestamp, and additional records like CodeView (CV) and miscellaneous (MISC) information.
#[derive(Debug, Clone)]
pub struct Module<'a> {
    /// The memory range of the module.
    pub range: std::ops::Range<u64>,

    /// The checksum of the module.
    pub checksum: u32,

    /// The path to the module file.
    pub path: std::path::PathBuf,

    /// The timestamp when the module was built, represented as a 32-bit UNIX time value.
    pub time_date_stamp: u32,

    /// The CodeView (CV) record for debugging information.
    pub cv_record: &'a [u8],

    /// The miscellaneous (MISC) record, often containing additional debug metadata.
    pub misc_record: &'a [u8],
}

impl<'a> Module<'a> {
    /// Creates a new `Module` instance from a `MINIDUMP_MODULE` and its name.
    ///
    /// # Arguments
    ///
    /// * `module` - A reference to a `MINIDUMP_MODULE` containing information about the module.
    /// * `name` - A `String` representing the module's name or path.
    ///
    /// # Returns
    ///
    /// * A new `Module` instance initialized with the provided data.
    ///
    /// # Panics
    ///
    /// * This function will panic if the memory range of the module is invalid (e.g., start >= end).
    pub fn new(module: &MINIDUMP_MODULE, name: String, cv_record: &'a [u8], misc_record: &'a [u8]) -> Self {
        let range = std::ops::Range {
            start: module.BaseOfImage,
            end: module.BaseOfImage + module.SizeOfImage as u64,
        };

        if range.is_empty() {
            panic!("Problem building the memory range")
        }

        Self {
            range,
            checksum: module.CheckSum,
            path: name.into(),
            time_date_stamp: module.TimeDateStamp,
            cv_record,
            misc_record,
        }
    }

    /// Returns the name of the module file, if available.
    ///
    /// # Returns
    ///
    /// * An `Option<&str>` containing the file name, or `None` if the path is invalid or
    ///   not UTF-8 encoded.
    pub fn name(&self) -> Option<&str> {
        self.path.file_name()?.to_str()
    }

    /// Returns the starting memory address of the module.
    ///
    /// # Returns
    ///
    /// * A `u64` representing the starting address of the module.
    pub fn start_addr(&self) -> u64 {
        self.range.start
    }

    /// Returns the ending memory address of the module (inclusive).
    ///
    /// # Returns
    ///
    /// * A `u64` representing the ending address of the module.
    pub fn end_addr(&self) -> u64 {
        self.range.end - 1
    }

    /// Returns the size of the module in bytes.
    ///
    /// # Returns
    ///
    /// * A `u64` representing the size of the module.
    pub fn len(&self) -> u64 {
        self.range.end - self.range.start
    }

    /// Returns true if the module has zero size.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> MinidumpStream<'a> for Module<'a> {
    type Output = Modules<'a>;

    /// Parses the list of modules from the `ModuleListStream`.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Cursor positioned at the module list stream.
    ///
    /// # Returns
    ///
    /// * `Ok(Modules<'a>)` - If the modules are parsed successfully.
    /// * `Err(UserDmpError)` - If an error occurs during parsing.
    fn parse(cursor: &mut Cursor<&'a [u8]>) -> Result<Modules<'a>> {
        // Reads the module list stream.
        let module_list = MINIDUMP_MODULE_LIST::read(cursor)?;

        // Parses each module entry in the list.
        let modules = module_list
            .Modules
            .iter()
            .map(|module| {
                // Seeks to the module name.
                cursor.seek(io::SeekFrom::Start(module.ModuleNameRva.into()))?;

                // reading the structure MINIDUMP_STRING
                let string = MINIDUMP_STRING::read(cursor)?;

                // Converts the name to UTF-8.
                let module_name = String::from_utf16_lossy(&string.Buffer)
                    .trim_end_matches('\0')
                    .to_string();

                // Creates a new Module.
                let module = Module::new(module, module_name, &[], &[]);
                Ok((module.range.start, module))
            })
            .collect::<Result<Modules>>()?;

        // Returns the parsed modules.
        Ok(modules)
    }
}

/// Represents the processor context of a thread captured in the minidump.
///
/// The `ThreadContext` enum encapsulates the architecture-specific context
/// data, such as register states, for threads in the captured process.
#[derive(Debug)]
pub enum ThreadContext {
    /// Represents the 64-bit processor context (`CONTEXT_X64`) for the thread.
    X64(Box<CONTEXT_X64>),

    /// Represents the 32-bit processor context (`CONTEXT_X86`) for the thread.
    X86(Box<CONTEXT_X86>),
}

/// Represents a thread in the process, as captured in the minidump file.
///
/// The `Thread` struct contains metadata about the thread, such as its ID,
/// priority, and execution context.
#[derive(Debug)]
pub struct Thread {
    /// The unique identifier (ID) of the thread.
    pub thread_id: u32,

    /// The number of times the thread has been suspended.
    pub suspend_count: u32,

    /// The priority class of the thread.
    pub priority_class: u32,

    /// The priority level of the thread within its priority class.
    pub priority: u32,

    /// The address of the Thread Environment Block (TEB), containing per-thread information.
    pub teb: u64,

    /// The execution context of the thread, including register states.
    context: ThreadContext,
}

impl Thread {
    /// Creates a new `Thread` instance from a `MINIDUMP_THREAD` structure and its context.
    ///
    /// # Arguments
    ///
    /// * `thread` - A reference to a `MINIDUMP_THREAD` containing metadata about the thread.
    /// * `context` - The architecture-specific execution context of the thread.
    ///
    /// # Returns
    ///
    /// * A new `Thread` instance initialized with the provided data.
    fn new(thread: &MINIDUMP_THREAD, context: ThreadContext) -> Self {
        Self {
            thread_id: thread.ThreadId,
            suspend_count: thread.SuspendCount,
            priority_class: thread.PriorityClass,
            priority: thread.Priority,
            teb: thread.Teb,
            context,
        }
    }

    /// Returns a reference to the execution context of the thread.
    pub fn context(&self) -> &ThreadContext {
        &self.context
    }

    /// Parses the list of threads from the `ThreadListStream`.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Cursor positioned at the thread list stream.
    /// * `arch` - An optional `Arch` parameter that specifies the architecture (e.g., `X64` or `X86`).
    ///            This is used to correctly parse the thread context based on the architecture.
    ///
    /// # Returns
    ///
    /// * `Ok(Threads)` - If the threads are parsed successfully.
    /// * `Err(UserDmpError)` - If an error occurs during parsing.
    fn parse(cursor: &mut Cursor<&[u8]>, arch: &Option<Arch>) -> Result<Threads> {
        // Reads the thread list stream.
        let thread_list = MINIDUMP_THREAD_LIST::read(cursor)?;

        // Parses each thread entry in the list.
        let threads = thread_list
            .Threads
            .iter()
            .map(|thread| {
                // Extracts the thread context.
                let context_slice = UserDump::extract_raw_data(cursor, thread.ThreadContext)?;
                let context = arch
                    .as_ref()
                    .map(|arch| match arch {
                        Arch::X64 => unsafe {
                            let ctx = ptr::read_unaligned(context_slice.as_ptr() as *const CONTEXT_X64);
                            ThreadContext::X64(Box::new(ctx))
                        },
                        Arch::X86 => unsafe {
                            let ctx = ptr::read_unaligned(context_slice.as_ptr() as *const CONTEXT_X86);
                            ThreadContext::X86(Box::new(ctx))
                        },
                    })
                    .ok_or(UserDmpError::InvalidContext)?;

                // Creates a new Thread.
                let thread = Thread::new(thread, context);
                Ok((thread.thread_id, thread))
            })
            .collect::<Result<Threads>>()?;

        Ok(threads)
    }
}

/// Represents a memory region in a minidump file, providing metadata about its state,
/// protection level, allocation base, and type.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Memory<'a> {
    /// The range of memory addresses for this region.
    pub range: std::ops::Range<u64>,

    /// The base address where this memory allocation begins.
    pub allocation_base: u64,

    /// The protection attributes applied at the time of memory allocation.
    pub allocation_protect: u32,

    /// The current state of the memory region, indicating if it's committed,
    /// reserved, or free (e.g., `MEM_COMMIT` or `MEM_FREE`).
    pub state: u32,

    /// The protection level of the memory region (e.g., `PAGE_READWRITE`).
    pub protect: u32,

    /// The type of memory region (e.g., private, mapped, or image).
    pub type_: u32,

    /// The raw bytes of the memory region, as extracted from the minidump file.
    /// This data represents the actual content of the memory in this region
    /// and can be used for further analysis or reconstruction.
    pub data: &'a [u8],
}

impl<'a> Memory<'a> {
    /// Creates a new `Memory` instance from a `MINIDUMP_MEMORY_INFO` structure.
    ///
    /// # Arguments
    ///
    /// * `memory` - A reference to a `MINIDUMP_MEMORY_INFO` containing details about the memory region.
    ///
    /// # Returns
    ///
    /// * A `Memory` instance initialized with the provided data.
    ///
    /// # Panics
    ///
    /// * This function will panic if the memory range is invalid (e.g., `start >= end`).
    fn new(memory: &MINIDUMP_MEMORY_INFO) -> Self {
        let range = std::ops::Range {
            start: memory.BaseAddress,
            end: memory.BaseAddress + memory.RegionSize,
        };

        if range.is_empty() {
            panic!("Problem building the memory range")
        }

        Self {
            range,
            allocation_base: memory.AllocationBase,
            allocation_protect: memory.AllocationProtect,
            state: memory.State,
            protect: memory.Protect,
            type_: memory.Type,
            ..Default::default()
        }
    }

    /// Returns a textual description of the current memory state.
    ///
    /// The possible states are:
    /// - `MEM_COMMIT` (0x1000): Memory is committed and backed by physical storage or the page file.
    /// - `MEM_RESERVE` (0x2000): Memory is reserved but not yet committed.
    /// - `MEM_FREE` (0x10000): Memory is free and available for allocation.
    /// - `MEM_RESET` (0x8000): Memory has been reset to a clean state.
    /// - `MEM_TOP_DOWN` (0x100000): Allocation was made top-down from high memory addresses.
    ///
    /// # Returns
    ///
    /// * A `&str` describing the state of the memory.
    pub fn state(&self) -> &str {
        if self.state == 0x10_000 {
            return "";
        }

        match self.state {
            0x1_000 => "MEM_COMMIT",
            0x2_000 => "MEM_RESERVE",
            0x10_000 => "MEM_FREE",
            0x8_000 => "MEM_RESET",
            0x100_000 => "MEM_TOP_DOWN",
            _ => "UNKNOWN",
        }
    }

    /// Returns a textual description of the memory type.
    ///
    /// The possible types are:
    /// - `MEM_PRIVATE` (0x20000): Memory is private to the process.
    /// - `MEM_MAPPED` (0x40000): Memory is mapped to a file.
    /// - `MEM_IMAGE` (0x1000000): Memory is associated with an executable image.
    ///
    /// # Returns
    ///
    /// * A `&str` describing the type of the memory region.
    pub fn type_memory(&self) -> &str {
        match self.type_ {
            0x20_000 => "MEM_PRIVATE",
            0x40_000 => "MEM_MAPPED",
            0x1_000_000 => "MEM_IMAGE",
            _ => "UNKNOWN",
        }
    }

    /// Returns the starting address of the memory region.
    ///
    /// # Returns
    ///
    /// * A `u64` representing the starting address of the memory region.
    pub fn start_addr(&self) -> u64 {
        self.range.start
    }

    /// Returns the ending address of the memory region (inclusive).
    ///
    /// # Returns
    ///
    /// * A `u64` representing the ending address of the memory region.
    pub fn end_addr(&self) -> u64 {
        self.range.end
    }

    /// Returns the size of the memory region in bytes.
    ///
    /// # Returns
    ///
    /// * A `u64` representing the size of the memory region.
    pub fn len(&self) -> u64 {
        self.range.end - self.range.start
    }

    /// Returns true if the memory region has zero size.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Merges two maps of memory regions into a single map.
    ///
    /// # Arguments
    ///
    /// * `memory_info` - Memory regions parsed from the `MemoryInfoListStream`.
    /// * `memory64` - Memory regions parsed from the `Memory64ListStream`.
    ///
    /// # Returns
    ///
    /// * `Ok(Memorys<'a>)` - The combined map of memory regions.
    /// * `Err(UserDmpError)` - If merging fails.
    fn merge_memory(mut memory_info: Memorys<'a>, memory64: Memorys<'a>) -> Result<Memorys<'a>> {
        // Insert memory64 regions into memory_info.
        for (address, memory) in memory64 {
            memory_info.insert(address, memory);
        }

        Ok(memory_info)
    }

    /// Parses memory information from the `MemoryInfoListStream`.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Cursor positioned at the memory info list stream.
    ///
    /// # Returns
    ///
    /// * `Ok(Memorys<'a>)` - A map of memory regions indexed by their base address.
    /// * `Err(UserDmpError)` - If an error occurs during parsing.
    fn parser_memory_info(cursor: &mut Cursor<&'a [u8]>) -> Result<Memorys<'a>> {
        // Reads the memory info list stream.
        let memory_info_list = MINIDUMP_MEMORY_INFO_LIST::read(cursor)?;

        // Parses each memory region in the list.
        let memorys = memory_info_list
            .Entries
            .iter()
            .map(|memory| {
                let memory_block = Memory::new(memory);

                Ok((memory.BaseAddress, memory_block))
            })
            .collect::<Result<Memorys>>()?;

        Ok(memorys)
    }

    /// Parses memory information from the `Memory64ListStream`.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Cursor positioned at the memory 64 list stream.
    ///
    /// # Returns
    ///
    /// * `Ok(Memorys<'a>)` - A map of memory regions indexed by their base address.
    /// * `Err(UserDmpError)` - If an error occurs during parsing.
    fn parser_memory64_list(cursor: &mut Cursor<&'a [u8]>) -> Result<Memorys<'a>> {
        // Reads the Memory64List stream.
        let memory64_list = MINIDUMP_MEMORY64_LIST::read(cursor)?;

        let mut memorys = Memorys::new();
        let mut current_rva = memory64_list.BaseRva;

        // Iterate over the memory descriptors in the list.
        for memory_descriptor in memory64_list.Ranges.iter() {
            let range = std::ops::Range {
                start: memory_descriptor.StartOfMemoryRange,
                end: memory_descriptor.StartOfMemoryRange + memory_descriptor.DataSize,
            };

            // Seek to the data for the current memory descriptor.
            cursor.seek(io::SeekFrom::Start(current_rva))?;

            // Read the memory data.
            let data = {
                let data_slice = &cursor.get_ref()[(current_rva as usize)..];
                &data_slice[..(memory_descriptor.DataSize as usize)]
            };

            // Create a Memory instance.
            let memory = Memory {
                range,
                allocation_base: 0,
                allocation_protect: 0,
                state: 0,
                protect: 0,
                type_: 0,
                data,
            };

            memorys.insert(memory_descriptor.StartOfMemoryRange, memory);

            // Update the current RVA for the next memory block.
            current_rva += memory_descriptor.DataSize;
        }

        Ok(memorys)
    }
}

/// Represents a handle in a minidump file, providing metadata about its type,
/// object name, attributes, and granted access rights.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Handle {
    /// The unique identifier (handle value) for this object.
    pub handle: u64,

    /// The type name of the object associated with the handle (e.g., `File`, `Event`).
    type_name: Option<String>,

    /// The object name associated with the handle, if available (e.g., file path).
    object_name: Option<String>,

    /// The attributes of the handle (e.g., inheritance flags).
    pub attributes: u32,

    /// The access rights granted to this handle.
    pub granted_access: u32,
}

impl Handle {
    /// Creates a new `Handle` instance from a `MINIDUMP_HANDLE_DESCRIPTOR`.
    ///
    /// # Arguments
    ///
    /// * `type_name` - An optional string representing the type of the handle (e.g., `File`).
    /// * `object_name` - An optional string representing the name of the object (e.g., file path).
    /// * `handle` - A reference to a `MINIDUMP_HANDLE_DESCRIPTOR` structure containing handle details.
    ///
    /// # Returns
    ///
    /// * A `Handle` instance initialized with the provided data.
    pub fn new(type_name: Option<String>, object_name: Option<String>, handle: &MINIDUMP_HANDLE_DESCRIPTOR) -> Self {
        Self {
            handle: handle.Handle,
            type_name,
            object_name,
            attributes: handle.Attributes,
            granted_access: handle.GrantedAccess,
        }
    }

    /// Returns the handle value as a hexadecimal string.
    ///
    /// # Returns
    ///
    /// * A `String` representing the handle in hexadecimal format.
    pub fn handle(&self) -> String {
        format!("0x{:x}", self.handle)
    }

    /// Returns the type name of the object associated with the handle.
    ///
    /// # Returns
    ///
    /// * An `Option<&str>` containing the type name, or `None` if unavailable.
    pub fn type_name(&self) -> Option<&str> {
        self.type_name.as_deref()
    }

    /// Returns the object name associated with the handle.
    ///
    /// # Returns
    ///
    /// * An `Option<&str>` containing the object name, or `None` if unavailable.
    pub fn object_name(&self) -> Option<&str> {
        self.object_name.as_deref()
    }
}

impl<'a> MinidumpStream<'a> for Handle {
    type Output = Handles;

    /// Parses the list of handles from the `HandleDataStream`.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Cursor positioned at the handle list stream.
    ///
    /// # Returns
    ///
    /// * `Ok(Handles)` - If the handles are parsed successfully.
    /// * `Err(UserDmpError)` - If an error occurs during parsing.
    fn parse(cursor: &mut Cursor<&'a [u8]>) -> Result<Self::Output> {
        // Reads the handle list stream.
        let handle_data = MINIDUMP_HANDLE_DATA_STREAM::read(cursor)?;

        // Parses each handle entry in the list.
        let handles = handle_data
            .Handles
            .iter()
            .map(|handle| {
                let type_name = if handle.TypeNameRva != 0 {
                    // Seeks to the type name.
                    cursor.seek(io::SeekFrom::Start(handle.TypeNameRva.into()))?;

                    // reading the structure MINIDUMP_STRING
                    let string = MINIDUMP_STRING::read(cursor)?;

                    // Converts the name to UTF-8.
                    let name = String::from_utf16_lossy(&string.Buffer)
                        .trim_end_matches('\0')
                        .to_string();

                    Some(name)
                } else {
                    None
                };

                let object_name = if handle.ObjectNameRva != 0 {
                    // Seeks to the object name.
                    cursor.seek(io::SeekFrom::Start(handle.ObjectNameRva.into()))?;

                    // reading the structure MINIDUMP_STRING
                    let string = MINIDUMP_STRING::read(cursor)?;

                    // Converts the name to UTF-8.
                    let name = String::from_utf16_lossy(&string.Buffer)
                        .trim_end_matches('\0')
                        .to_string();

                    Some(name)
                } else {
                    None
                };

                // Creates a new Handle.
                let handle = Handle::new(type_name, object_name, handle);
                Ok((handle.handle, handle))
            })
            .collect::<Result<Handles>>()?;

        Ok(handles)
    }
}
