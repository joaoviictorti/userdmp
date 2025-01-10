use std::path::Path;
use userdmp::{error::UserDmpError, UserDump};

fn main() -> Result<(), UserDmpError> {
    let path = Path::new("C:\\Examples.dmp");
    let dmp = UserDump::new(path)?;

    for (_, module) in dmp.modules().iter() {
        println!("[*] Path: {:?}", module.path);
        println!("[*] Range Address: {:?}", module.range);
        println!("[*] Checksum: {:?}", module.checksum);
        // Access the other members ...
    }

    Ok(())
}
