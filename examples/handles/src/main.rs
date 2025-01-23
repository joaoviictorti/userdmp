use userdmp::{error::UserDmpError, UserDump};

fn main() -> Result<(), UserDmpError> {
    let dmp = UserDump::new("C:\\Examples.dmp")?;

    for(_, handle) in dmp.handles() {
        println!("Handle: {}", handle.handle());
        println!("Access: {}", handle.granted_access);
        println!("Type Name: {:?}", handle.type_name().unwrap_or(""));
        println!("Object Name: {:?}", handle.object_name().unwrap_or(""))
    } 

    Ok(())
}
