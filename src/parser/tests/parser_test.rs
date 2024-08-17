use crate::parser::parser::Parser;
use crate::Word;
use std::fs::File;
use std::path::PathBuf;

#[test]
fn test_parse_dos_header() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("testsources/XInclude.exe");

    let mut file = File::open(path).expect("Failed to open the test executable");

    match Parser::parse_dos_header(&mut file) {
        Ok(dos_header) => {
            assert_eq!(dos_header.e_magic(), [0x4D, 0x5A]); // 'MZ' marker

            let length_file = file.metadata().unwrap().len();

            let offset_nt_header = u32::from_le_bytes(dos_header.e_lfanew());

            assert!(offset_nt_header < length_file as u32)
        }

        Err(e) => {}
    }
}
