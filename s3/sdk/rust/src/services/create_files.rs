use std::fs::File;
use std::io::prelude::*;
extern crate guid_create;
use guid_create::GUID;

pub fn create_files() -> std::io::Result<()> {
    for i in 1..3 {

        let filename = format!("mydoc{}", i);
        println!("i is: {}", filename);

        let guid = GUID::rand();
        let content = guid.to_string();
        
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        
    }

    Ok(())
    
}