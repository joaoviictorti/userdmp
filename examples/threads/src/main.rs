use userdmp::{error::UserDmpError, UserDump};

fn main() -> Result<(), UserDmpError> {
    let dmp = UserDump::new("C:\\Examples.dmp")?;

    for (tid, thread) in dmp.threads().iter() {
        println!("[*] TID: {:?}", tid);
        println!("[*] TEB: {:?}", thread.teb);
        println!("[*] CONTEXT: {:#x?}", thread.context());
        // Access the other members ...
    }

    Ok(())
}
