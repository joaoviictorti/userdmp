use userdmp::{error::UserDmpError, UserDump};

fn main() -> Result<(), UserDmpError> {
    let dmp = UserDump::new("C:\\Examples.dmp")?;

    for (_, module) in dmp.modules().iter() {
        println!("[*] Path: {:?}", module.path);
        println!("[*] Range Address: {:?}", module.range);
        println!("[*] Checksum: {:?}", module.checksum);
        // Access the other members ...
    }

    Ok(())
}
