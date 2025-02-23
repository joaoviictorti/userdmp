#![allow(non_snake_case, non_camel_case_types)]

/// Maximum number of parameters associated with an exception.
pub const EXCEPTION_MAXIMUM_PARAMETERS: usize = 15;

/// Signature to identify Minidump files ("MDMP" in ASCII).
pub const MINIDUMP_SIGNATURE: u32 = 0x504D_444D;

/// Default flags for configuring dumps.
pub const DUMP_FLAGS: u64 = 0x001F_FFFF;

/// Architecture code for 64-bit systems (x86_64).
pub const ARCH_X64: u16 = 9;

/// Architecture code for 32-bit systems (x86).
pub const ARCH_X86: u16 = 0;

/// Contains header information for the minidump file.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_header).
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_HEADER {
    /// The signature.
    pub Signature: u32,
    
    /// The version of the minidump format.
    pub Version: u32,
    
    /// The number of streams in the minidump directory.
    pub NumberOfStreams: u32,
    
    /// The base RVA of the minidump directory.
    pub StreamDirectoryRva: u32,
    
    /// The checksum for the minidump file.
    pub CheckSum: u32,
    
    /// This member is reserved.
    pub Reserved: u32,

    // Time and date, in time_t format.
    pub TimeDateStamp: u32,
    
    /// One or more values from the MINIDUMP_TYPE enumeration type.
    pub Flags: u64,
}

/// Contains the information needed to access a specific data stream in a minidump file.
///
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_directory).
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
#[derive(Debug)]
pub struct MINIDUMP_DIRECTORY {
    /// The type of data stream.
    pub StreamType: u32,

    /// A [`MINIDUMP_LOCATION_DESCRIPTOR`] structure that specifies the location of the data stream.
    pub Location: MINIDUMP_LOCATION_DESCRIPTOR
}

/// Represents an exception information stream.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_exception_stream).
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_EXCEPTION_STREAM {
    /// The identifier of the thread that caused the exception.
    pub ThreadId: u32,

    /// A variable for alignment.
    pub alignment: u32,
    
    /// A MINIDUMP_EXCEPTION structure.
    pub ExceptionRecord: MINIDUMP_EXCEPTION,
    
    /// A MINIDUMP_LOCATION_DESCRIPTOR structure.
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR 
}

/// Represents an exception information stream.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_exception_stream).
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_EXCEPTION {
    /// The reason the exception occurred.
    pub ExceptionCode: u32,

    ///This member can be either zero, indicating a continuable exception, or EXCEPTION_NONCONTINUABLE, indicating a noncontinuable exception.
    pub ExceptionFlags: u32,
    
    /// A pointer to an associated MINIDUMP_EXCEPTION structure.
    pub ExceptionRecord: u64,
    
    /// The address where the exception occurred.
    pub ExceptionAddress: u64,
    
    /// The number of parameters associated with the exception. 
    pub NumberParameters: u32,
    
    /// Reserved for cross-platform structure member alignment. Do not set
    pub unusedAlignment: u32, 
    
    /// An array of additional arguments that describe the exception.
    pub ExceptionInformation: [u64; EXCEPTION_MAXIMUM_PARAMETERS],
}

/// Contains a list of memory ranges.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_memory_info_list).
#[derive(Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_MEMORY_INFO_LIST {
    /// Size of the header for this structure.
    pub SizeOfHeader: u32,
    
    /// Size of each entry in the memory info list.
    pub SizeOfEntry: u32,
    
    /// Number of entries in the memory info list.
    pub NumberOfEntries: u64,
    
    /// The list of memory info entries.
    #[br(count = NumberOfEntries)]
    pub Entries: Vec<MINIDUMP_MEMORY_INFO>,
}

/// Describes a region of memory.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_memory_info). 
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_MEMORY_INFO {
    /// The base address of the memory region.
    pub BaseAddress: u64,

    /// The base address of the allocation containing the memory region.
    pub AllocationBase: u64,

    /// The memory protection applied at the time of allocation.
    pub AllocationProtect: u32,

    /// Alignment padding (unused).
    pub alignment1: u32,

    /// The size of the memory region in bytes.
    pub RegionSize: u64,

    /// The state of the memory region (e.g., committed, free, reserved).
    pub State: u32,

    /// The protection level of the memory region.
    pub Protect: u32,

    /// The type of memory region (e.g., private, mapped, image).
    pub Type: u32,
    
    /// Alignment padding (unused).
    pub alignment2: u32,
}

/// Contains a list of memory ranges.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_memory64_list). 
#[derive(Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_MEMORY64_LIST {
    /// The number of structures in the MemoryRanges array.
    pub NumberOfMemoryRanges: u64,
    
    /// An array of MINIDUMP_MEMORY_DESCRIPTOR structures.
    pub BaseRva: u64,

    /// Memory descriptors.
    #[br(count = NumberOfMemoryRanges)]
    pub Ranges: Vec<MINIDUMP_MEMORY_DESCRIPTOR64>,
}

/// Describes a range of memory.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_memory_descriptor). 
#[derive(Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_MEMORY_DESCRIPTOR64 {
    /// The starting address of the memory range.
    pub StartOfMemoryRange: u64,

    /// A MINIDUMP_LOCATION_DESCRIPTOR structure.
    pub DataSize: u64
} 

/// Contains processor and operating system information.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_system_info)
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_SYSTEM_INFO {
    /// The system's processor architecture. 
    pub ProcessorArchitecture: u16,
    
    /// The system's architecture-dependent processor level.
    pub ProcessorLevel: u16,
    
    /// The architecture-dependent processor revision.
    pub ProcessorRevision: u16,
    
    /// The number of processors in the system.
    pub NumberOfProcessors: u8,
    
    /// Any additional information about the system.
    pub ProductType: u8,
    
    /// The major version number of the operating system.
    pub MajorVersion: u32,
    
    /// The minor version number of the operating system.
    pub MinorVersion: u32,
    
    /// The build number of the operating system.
    pub BuildNumber: u32,
    
    /// The operating system platform.
    pub PlatformId: u32,
    
    /// An RVA (from the beginning of the dump) to a MINIDUMP_STRING that describes the latest Service Pack installed on the system.
    pub CSDVersionRva: u32,
    
    /// The bit flags that identify the product suites available on the system.
    pub SuiteMask: u16,
    
    /// This member is reserved for future use.
    pub Reserved2: u16,
}

/// Contains a list of modules.
/// 
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_module_list)
#[derive(Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_MODULE_LIST {
    /// The number of structures in the Modules array.
    pub NumberOfModules: u32,

    /// An array of MINIDUMP_MODULE structures.
    #[br(count = NumberOfModules)]
    pub Modules: Vec<MINIDUMP_MODULE>,
} 

/// Contains information for a specific module.
/// 
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_module)
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)] 
pub struct MINIDUMP_MODULE {
    /// The base address of the module executable image in memory.
    pub BaseOfImage: u64,

    /// The size of the module executable image in memory, in bytes.
    pub SizeOfImage: u32,

    /// The checksum value of the module executable image.
    pub CheckSum: u32,

    /// The timestamp value of the module executable image, in time_t format.
    pub TimeDateStamp: u32,

    /// An RVA to a MINIDUMP_STRING structure that specifies the name of the module.
    pub ModuleNameRva: u32,

    /// A VS_FIXEDFILEINFO structure that specifies the version of the module.
    pub VersionInfo: VS_FIXEDFILEINFO,

    /// A MINIDUMP_LOCATION_DESCRIPTOR structure that specifies the CodeView record of the module.
    pub CvRecord: MINIDUMP_LOCATION_DESCRIPTOR,

    /// A MINIDUMP_LOCATION_DESCRIPTOR structure that specifies the miscellaneous record of the module.
    pub MiscRecord: MINIDUMP_LOCATION_DESCRIPTOR,
    
    /// Reserved for future use.
    pub Reserved0: u64,

    /// Reserved for future use.
    pub Reserved1: u64,
}

/// Contains a bitmask that specifies the Boolean attributes of the file.
#[repr(transparent)]
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct VS_FIXEDFILEINFO_FILE_FLAGS(pub u32);

/// The operating system for which this file was designed.
#[repr(transparent)]
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct VS_FIXEDFILEINFO_FILE_OS(pub u32);

/// Contains version information for a file.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/verrsrc/ns-verrsrc-vs_fixedfileinfo)
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct VS_FIXEDFILEINFO {
    /// Contains the value 0xFEEF04BD.
    pub dwSignature: u32,

    /// The binary version number of this structure.
    pub dwStrucVersion: u32,

    /// The most significant 32 bits of the file's binary version number.
    pub dwFileVersionMS: u32,

    /// The least significant 32 bits of the file's binary version number.
    pub dwFileVersionLS: u32,

    /// The most significant 32 bits of the binary version number of the product with which this file was distributed.
    pub dwProductVersionMS: u32,

    /// The least significant 32 bits of the binary version number of the product with which this file was distributed.
    pub dwProductVersionLS: u32,

    /// Contains a bitmask that specifies the valid bits in dwFileFlags.
    pub dwFileFlagsMask: u32,

    /// Contains a bitmask that specifies the Boolean attributes of the file.
    pub dwFileFlags: VS_FIXEDFILEINFO_FILE_FLAGS,

    /// The operating system for which this file was designed.
    pub dwFileOS: VS_FIXEDFILEINFO_FILE_OS,
    
    /// The general type of file.
    pub dwFileType: u32,

    /// The function of the file. 
    pub dwFileSubtype: u32,

    /// The most significant 32 bits of the file's 64-bit binary creation date and time stamp.
    pub dwFileDateMS: u32,

    /// The least significant 32 bits of the file's 64-bit binary creation date and time stamp.
    pub dwFileDateLS: u32,
}

/// Contains a list of threads.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_thread_list)
#[derive(Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_THREAD_LIST {
    /// The number of structures in the Threads array.
    pub NumberOfThreads: u32,

    /// An array of MINIDUMP_THREAD structures.
    #[br(count = NumberOfThreads)]
    pub Threads: Vec<MINIDUMP_THREAD>,
}

/// Contains information for a specific thread.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_thread)
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_THREAD {
    /// The identifier of the thread.
    pub ThreadId: u32,
    
    /// The suspend count for the thread. If the suspend count is greater than zero, the thread is suspended; otherwise, the thread is not suspended.
    pub SuspendCount: u32,

    /// The priority class of the thread. See Scheduling Priorities.
    pub PriorityClass: u32,

    /// The priority level of the thread.
    pub Priority: u32,

    /// The thread environment block.
    pub Teb: u64,

    /// A MINIDUMP_MEMORY_DESCRIPTOR structure.
    pub Stack: MINIDUMP_MEMORY_DESCRIPTOR,

    /// A MINIDUMP_LOCATION_DESCRIPTOR structure.
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR
}

/// Describes a range of memory.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_memory_descriptor)
#[derive(Copy, Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_MEMORY_DESCRIPTOR {
    /// The starting address of the memory range.
    pub StartOfMemoryRange: u64,

    /// A MINIDUMP_LOCATION_DESCRIPTOR structure.
    pub Memory: MINIDUMP_LOCATION_DESCRIPTOR
}

/// Contains information describing the location of a data stream within a minidump file.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_location_descriptor)
#[derive(Copy, Clone, Debug)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_LOCATION_DESCRIPTOR {
    /// The size of the data stream, in bytes.
    pub DataSize: u32,

    /// The relative virtual address (RVA) of the data.
    pub RVA: u32,
}

#[derive(Debug, Clone, Copy, binrw::NamedArgs)]
pub struct HandleArgs {
    /// The size of the descriptor.
    pub size_of_descriptor: u32,
}

/// Represents the header for a handle data stream.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_handle_data_stream)
#[derive(Clone)]
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_HANDLE_DATA_STREAM {
    /// The size of the header information for the stream, in bytes.
    pub SizeOfHeader: u32,

    /// The size of a descriptor in the stream, in bytes.
    pub SizeOfDescriptor: u32,

    /// The number of descriptors in the stream.
    pub NumberOfDescriptors: u32,

    /// Reserved for future use; must be zero.
    pub Reserved: u32,

    /// List of handle descriptors.
    #[br(
        count = NumberOfDescriptors,
        args { inner: (SizeOfDescriptor,) }
    )]
    pub Handles: Vec<MINIDUMP_HANDLE_DESCRIPTOR>,
}

/// Contains the state of an individual system handle at the time the minidump was written.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_handle_descriptor)
#[derive(Clone)]
#[binrw::binrw]
#[brw(little, import(size_of_descriptor: u32))]
pub struct MINIDUMP_HANDLE_DESCRIPTOR {
    /// The operating system handle value.
    pub Handle: u64,

    /// An RVA to a MINIDUMP_STRING structure that specifies the object type of the handle.
    pub TypeNameRva: u32,
    
    /// An RVA to a MINIDUMP_STRING structure that specifies the object name of the handle.
    pub ObjectNameRva: u32,

    /// The meaning of this member depends on the handle type and the operating system.
    pub Attributes: u32,

    /// The meaning of this member depends on the handle type and the operating system.
    pub GrantedAccess: u32,

    /// The meaning of this member depends on the handle type and the operating system.
    pub HandleCount: u32,

    /// The meaning of this member depends on the handle type and the operating system.
    pub PointerCount: u32,

    /// Extra space to adjust the size of the descriptor.
    #[br(pad_after = (size_of_descriptor - size_of::<Self>() as u32) as usize)]
    _padding: (),
}

/// Describes a string.
/// 
/// For more details, see the official [Microsoft documentation](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_string)
#[binrw::binrw]
#[brw(little)]
pub struct MINIDUMP_STRING { 
    /// The size of the string in the Buffer member, in bytes.
    pub Length: u32,

    // The null-terminated string.
    #[br(count = Length / 2)]
    pub Buffer: Vec<u16>,
}

/// Represents the type of a minidump data stream.
/// 
/// <https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ne-minidumpapiset-minidump_stream_type>
#[allow(dead_code)]
#[repr(u32)]
pub enum MINIDUMP_STREAM_TYPE {
    UnusedStream = 0,
    ReservedStream0 = 1,
    ReservedStream1 = 2,
    ThreadListStream = 3,
    ModuleListStream = 4,
    MemoryListStream = 5,
    ExceptionStream = 6,
    SystemInfoStream = 7,
    ThreadExListStream = 8,
    Memory64ListStream = 9,
    CommentStreamA = 10,
    CommentStreamW = 11,
    HandleDataStream = 12,
    FunctionTableStream = 13,
    UnloadedModuleListStream = 14,
    MiscInfoStream = 15,
    MemoryInfoListStream = 16,
    ThreadInfoListStream = 17,
    HandleOperationListStream = 18,
    TokenStream = 19,
    JavaScriptDataStream = 20,
    SystemMemoryInfoStream = 21,
    ProcessVmCountersStream = 22,
    IptTraceStream = 23,
    ThreadNamesStream = 24,
    ceStreamNull = 0x8000,
    ceStreamSystemInfo = 0x8001,
    ceStreamException = 0x8002,
    ceStreamModuleList = 0x8003,
    ceStreamProcessList = 0x8004,
    ceStreamThreadList = 0x8005,
    ceStreamThreadContextList = 0x8006,
    ceStreamThreadCallStackList = 0x8007,
    ceStreamMemoryVirtualList = 0x8008,
    ceStreamMemoryPhysicalList = 0x8009,
    ceStreamBucketParameters = 0x800A,
    ceStreamProcessModuleMap = 0x800B,
    ceStreamDiagnosisList = 0x800C,
    LastReservedStream = 0xffff
}

impl TryFrom<u32> for MINIDUMP_STREAM_TYPE {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MINIDUMP_STREAM_TYPE::UnusedStream),
            1 => Ok(MINIDUMP_STREAM_TYPE::ReservedStream0),
            2 => Ok(MINIDUMP_STREAM_TYPE::ReservedStream1),
            3 => Ok(MINIDUMP_STREAM_TYPE::ThreadListStream),
            4 => Ok(MINIDUMP_STREAM_TYPE::ModuleListStream),
            5 => Ok(MINIDUMP_STREAM_TYPE::MemoryListStream),
            6 => Ok(MINIDUMP_STREAM_TYPE::ExceptionStream),
            7 => Ok(MINIDUMP_STREAM_TYPE::SystemInfoStream),
            8 => Ok(MINIDUMP_STREAM_TYPE::ThreadExListStream),
            9 => Ok(MINIDUMP_STREAM_TYPE::Memory64ListStream),
            10 => Ok(MINIDUMP_STREAM_TYPE::CommentStreamA),
            11 => Ok(MINIDUMP_STREAM_TYPE::CommentStreamW),
            12 => Ok(MINIDUMP_STREAM_TYPE::HandleDataStream),
            13 => Ok(MINIDUMP_STREAM_TYPE::FunctionTableStream),
            14 => Ok(MINIDUMP_STREAM_TYPE::UnloadedModuleListStream),
            15 => Ok(MINIDUMP_STREAM_TYPE::MiscInfoStream),
            16 => Ok(MINIDUMP_STREAM_TYPE::MemoryInfoListStream),
            17 => Ok(MINIDUMP_STREAM_TYPE::ThreadInfoListStream),
            18 => Ok(MINIDUMP_STREAM_TYPE::HandleOperationListStream),
            19 => Ok(MINIDUMP_STREAM_TYPE::TokenStream),
            20 => Ok(MINIDUMP_STREAM_TYPE::JavaScriptDataStream),
            21 => Ok(MINIDUMP_STREAM_TYPE::SystemMemoryInfoStream),
            22 => Ok(MINIDUMP_STREAM_TYPE::ProcessVmCountersStream),
            23 => Ok(MINIDUMP_STREAM_TYPE::IptTraceStream),
            24 => Ok(MINIDUMP_STREAM_TYPE::ThreadNamesStream),
            0x8000 => Ok(MINIDUMP_STREAM_TYPE::ceStreamNull),
            0x8001 => Ok(MINIDUMP_STREAM_TYPE::ceStreamSystemInfo),
            0x8002 => Ok(MINIDUMP_STREAM_TYPE::ceStreamException),
            0x8003 => Ok(MINIDUMP_STREAM_TYPE::ceStreamModuleList),
            0x8004 => Ok(MINIDUMP_STREAM_TYPE::ceStreamProcessList),
            0x8005 => Ok(MINIDUMP_STREAM_TYPE::ceStreamThreadList),
            0x8006 => Ok(MINIDUMP_STREAM_TYPE::ceStreamThreadContextList),
            0x8007 => Ok(MINIDUMP_STREAM_TYPE::ceStreamThreadCallStackList),
            0x8008 => Ok(MINIDUMP_STREAM_TYPE::ceStreamMemoryVirtualList),
            0x8009 => Ok(MINIDUMP_STREAM_TYPE::ceStreamMemoryPhysicalList),
            0x800A => Ok(MINIDUMP_STREAM_TYPE::ceStreamBucketParameters),
            0x800B => Ok(MINIDUMP_STREAM_TYPE::ceStreamProcessModuleMap),
            0x800C => Ok(MINIDUMP_STREAM_TYPE::ceStreamDiagnosisList),
            0xffff => Ok(MINIDUMP_STREAM_TYPE::LastReservedStream),
            _ => Err("Invalid value for MINIDUMP_STREAM_TYPE"),
        }
    }
}

/// CONTEXT structure representing 64 bits
#[derive(Debug)]
#[repr(C, align(16))]
pub struct CONTEXT_X64 {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5Home: u64,
    pub P6Home: u64,
    pub ContextFlags: u32,
    pub MxCsr: u32,
    pub SegCs: u16,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub SegSs: u16,
    pub EFlags: u32,
    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub Rsp: u64,
    pub Rbp: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Rip: u64,
    pub Header: [u128; 2],
    pub Legacy: [u128; 8],
    pub Xmm0: u128,
    pub Xmm1: u128,
    pub Xmm2: u128,
    pub Xmm3: u128,
    pub Xmm4: u128,
    pub Xmm5: u128,
    pub Xmm6: u128,
    pub Xmm7: u128,
    pub Xmm8: u128,
    pub Xmm9: u128,
    pub Xmm10: u128,
    pub Xmm11: u128,
    pub Xmm12: u128,
    pub Xmm13: u128,
    pub Xmm14: u128,
    pub Xmm15: u128,
    pub Padding: [u8; 0x60],
    pub VectorRegister: [u128; 26],
    pub VectorControl: u64,
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}

/// CONTEXT structure representing 32 bits
#[derive(Debug)]
#[repr(C)]
pub struct CONTEXT_X86 {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Spare0: u32,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
    pub ExtendedRegisters: [u8;512]
}