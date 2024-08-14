use std::fs::File;
use std::io::{Error, ErrorKind};

struct FileImporter {
    file: Option<File>,
}

impl FileImporter {
    // Method to open a file and store it in the `file` field
    fn openfile(&mut self, path: &str) -> Result<(), Error> {
        match File::open(path) {
            Ok(file) => {
                println!(
                    "File at path: {} opened successfully with size: {} bytes",
                    path,
                    file.metadata()?.len()
                );
                self.file = Some(file); // Store the opened file in the struct
                Ok(())
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        println!("Error: File not found");
                    }
                    ErrorKind::PermissionDenied => {
                        println!("Error: Permission denied");
                    }
                    _ => {
                        println!("Error: {:?}", e);
                    }
                }
                Err(e) // Propagate the error
            }
        }
    }
}

