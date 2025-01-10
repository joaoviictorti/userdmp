use std::path::Path;
use userdmp::{error::UserDmpError, UserDump};

fn main() -> Result<(), UserDmpError> {
    let path = Path::new("C:\\Examples.dmp");
    let dmp = UserDump::new(path)?;
    let system = dmp.system;
    
    println!("Number Of Processors: {}", system.number_of_processors);
    println!("Arch: {:?}", system.processor_architecture);
    println!("BuildNumber: {:?}", system.build_number);
    // Access the other members ...

    Ok(())
}
