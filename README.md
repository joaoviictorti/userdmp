# userdmp 🦀

![Rust](https://img.shields.io/badge/made%20with-Rust-red)
![Platform](https://img.shields.io/badge/platform-windows-blueviolet)
![Forks](https://img.shields.io/github/forks/joaoviictorti/userdmp)
![Stars](https://img.shields.io/github/stars/joaoviictorti/userdmp)
![License](https://img.shields.io/github/license/joaoviictorti/userdmp)

`userdmp` is library in Rust for parsing Minidump (.dmp) files generated in user mode on Windows

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
    - [Parsing a Minidump File](#parsing-a-minidump-file)
- [Contributing to userdmp](#contributing-to-userdmp)
- [License](#license)

## Features

The **userdmp** format provides support for capturing detailed system state information during a crash or error. Its features include:

- ✅ **Module List Stream (`ModuleListStream`)**: Contains information about all loaded modules (e.g., DLLs), including their file paths, base addresses, and sizes.  
- ✅ **Handle Data Stream (`HandleDataStream`)**: Captures details about open handles in the process, such as references to files, threads, and synchronization objects.  
- ✅ **System Info Stream (`SystemInfoStream`)**: Includes metadata about the operating system (e.g., version, build number) and hardware (e.g., CPU type and number of processors).  
- ✅ **Exception Stream (`ExceptionStream`)**: Records details about the exception that triggered the dump, including the exception code, address, and relevant parameters.  
- ✅ **Memory Stream (`MemoryListStream / MemoryInfoListStream`)**: Provides a list of memory regions that were included in the dump, allowing analysis of process memory contents at the time of the crash.

## Installation

Add `userdmp` to your project by updating your `Cargo.toml`:
```bash
cargo add userdmp
```

Or manually add the dependency:
```toml
[dependencies]
userdmp = "<version>"
```

## Usage

The userdmp library provides tools to parse and analyze Minidump (.dmp) files generated in user mode on Windows. Here's how you can use it:

### Parsing a Minidump File

To start working with a Minidump file, use the `UserDump::new` function to parse the file and create a `UserDump` instance:
```rust, ignore
use std::path::Path;
use userdmp::{UserDump, UserDmpError};

fn main() -> Result<(), UserDmpError> {
    let path = Path::new("example.dmp");

    // Parse the Minidump file
    let dump = UserDump::new(path)?;
    println!("Minidump parsed successfully!");

    Ok(())
}
```

For more examples, see the [`examples`](./examples) folder in this repository. 📂


## Contributing to userdmp

To contribute to **userdmp**, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`.
4. Push your changes to your branch: `git push origin <branch_name>`.
5. Create a pull request.

Alternatively, consult the [GitHub documentation](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests) on how to create a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file for details.