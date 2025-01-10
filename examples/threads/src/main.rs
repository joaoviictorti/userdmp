use std::path::Path;
use userdmp::{error::UserDmpError, UserDump};

fn main() -> Result<(), UserDmpError> {
    let path = Path::new("C:\\Examples.dmp");
    let dmp = UserDump::new(path)?;

    for (tid, thread) in dmp.threads().iter() {
        println!("[*] TID: {:?}", tid);
        println!("[*] TEB: {:?}", thread.teb);
        println!("[*] CONTEXT: {:#x?}", thread.context());
        // Access the other members ...
    }

    Ok(())
}
