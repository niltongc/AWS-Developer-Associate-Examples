use std::fs::File;
use std::io::prelude::*;
extern crate guid_create;
use guid_create::GUID;

pub fn create_files() -> std::io::Result<Vec<String>> {

    std::fs::create_dir_all("src/my_files")?;

    let mut file_paths = Vec::new();

    for i in 1..3 {

        let filename = format!("src/my_files/mydoc{}", i);
        println!("i is: {}", filename);

        let guid = GUID::rand();
        let content = guid.to_string();
        
        let mut file = File::create(&filename)?;
        file.write_all(content.as_bytes())?;

        file_paths.push(filename);
        
    }

    Ok(file_paths)
    
}