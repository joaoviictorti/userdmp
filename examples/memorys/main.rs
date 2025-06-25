use userdmp::{error::UserDmpError, UserDump};

fn main() -> Result<(), UserDmpError> {
    let dmp = UserDump::new("C:\\Examples.dmp")?;

    for (_, memory) in dmp.memorys() {
        println!("Start: {}", memory.start_addr());
        println!("End: {}", memory.end_addr());
        println!("Data: {:?}", memory.data);
        // Access the other members ...
    }

    Ok(())
}
