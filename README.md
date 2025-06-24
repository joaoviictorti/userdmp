# userdmp 🦀

![Rust](https://img.shields.io/badge/made%20with-Rust-red)
![crate](https://img.shields.io/crates/v/userdmp.svg)
![docs](https://docs.rs/userdmp/badge.svg)
![Forks](https://img.shields.io/github/forks/joaoviictorti/userdmp)
![Stars](https://img.shields.io/github/stars/joaoviictorti/userdmp)
![License](https://img.shields.io/github/license/joaoviictorti/userdmp)

`userdmp` is library in Rust for parsing Minidump (.dmp) files generated in user mode on Windows

## Features

- ✅ **Module List Stream (`ModuleListStream`)**: Contains information about all loaded modules (e.g., DLLs), including their file paths, base addresses, and sizes.  
- ✅ **Handle Data Stream (`HandleDataStream`)**: Captures details about open handles in the process, such as references to files, threads, and synchronization objects.  
- ✅ **System Info Stream (`SystemInfoStream`)**: Includes metadata about the operating system (e.g., version, build number) and hardware (e.g., CPU type and number of processors).  
- ✅ **Exception Stream (`ExceptionStream`)**: Records details about the exception that triggered the dump, including the exception code, address, and relevant parameters.  
- ✅ **Memory Stream (`MemoryListStream / MemoryInfoListStream`)**: Provides a list of memory regions that were included in the dump, allowing analysis of process memory contents at the time of the crash.

## Getting started

Add `userdmp` to your project by updating your `Cargo.toml`:
```bash
cargo add userdmp
```

## Usage

The userdmp library provides tools to parse and analyze Minidump (.dmp) files generated in user mode on Windows. Here's how you can use it:

### Parsing a Minidump File

To start working with a Minidump file, use the `UserDump::new` function to parse the file and create a `UserDump` instance:
```rust, ignore
use userdmp::{UserDump, UserDmpError};

fn main() -> Result<(), UserDmpError> {
    // Parse the Minidump file
    let dump = UserDump::new("example.dmp")?;
    println!("Minidump parsed successfully!");

    Ok(())
}
```

## Additional Resources

For more examples, check the [examples](/examples) folder in the repository.

## License

This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file for details.
