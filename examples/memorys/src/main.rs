use std::path::Path;
use userdmp::{error::UserDmpError, UserDump};

fn main() -> Result<(), UserDmpError> {
    let path = Path::new("C:\\Examples.dmp");
    let dmp = UserDump::new(path)?;

    for(_, memory) in dmp.memorys() {
        println!("Start: {}", memory.start_addr());
        println!("End: {}", memory.end_addr());
        println!("Data: {:?}", memory.data);
        // Access the other members ...
    } 

    Ok(())
}
