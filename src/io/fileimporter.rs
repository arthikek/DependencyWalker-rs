use std::fs::File;
use std::io::{ErrorKind, Read, Result};

pub struct FileImporter {}

impl FileImporter {
    pub fn open_file(path: &str) -> Option<File> {
        match File::open(path) {
            Ok(file) => {
                let file_size = file.metadata().ok().map(|meta| meta.len());

                if let Some(length) = file_size {
                    println!("The file has been parsed and has a length of {}", length);
                } else {
                    println!("The file has been parsed, but there is no metadata available");
                }

                Some(file)
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => println!("Error: File not found"),
                    ErrorKind::PermissionDenied => println!("Error: Permission denied"),
                    _ => println!("Error: {:?}", e),
                }
                None
            }
        }
    }
    fn read_dos_header(file: &mut File, offset:u64, length : usize) -> Option<[u8; 64]>{
        let mut buffer = [0u8; 64];

        match file.read_exact(&mut buffer){
            Ok(..) => {
                 Some(buffer)
            }
            Err(..) => {
                None
            }
        }

    }


}
