use userdmp::{error::UserDmpError, UserDump};

fn main() -> Result<(), UserDmpError> {
    let dmp = UserDump::new("C:\\Examples.dmp")?;
    let system = dmp.system;

    println!("Number Of Processors: {}", system.number_of_processors);
    println!("Arch: {:?}", system.processor_architecture);
    println!("BuildNumber: {:?}", system.build_number);
    // Access the other members ...

    Ok(())
}
