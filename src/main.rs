use std::fs::File;
use crate::io::fileimporter::FileImporter;

mod io;
mod ui;
mod parser;
mod dto;
mod enums;

slint::slint! { import { MainWindow } from "src/ui/main.slint"; }

fn main() {
    let main_window = MainWindow::new().unwrap();
    main_window.run().expect("TODO: panic message");
    let mut file : File = FileImporter::open_file("C:\\Users\\arthike.kandasamy\\RustroverProjects\\DependencyWalker-rs\\testsources\\XInclude.exe").unwrap();
    let bytebuffer:[u8;9] = FileImporter::test_read_bytes(file).unwrap_or([0u8;9]);
    println!("Bytes read: {:02X} {:02X}", bytebuffer[0], bytebuffer[1]);
}
