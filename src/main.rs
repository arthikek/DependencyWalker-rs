use crate::io::fileimporter::FileImporter;
use std::fs::File;

mod dto;
mod enums;
mod io;
mod parser;
mod ui;

slint::slint! { import { MainWindow } from "src/ui/main.slint"; }

fn main() {
    let main_window = MainWindow::new().unwrap();
    main_window.run().expect("TODO: panic message");
    let mut file : File = FileImporter::open_file("C:\\Users\\arthike.kandasamy\\RustroverProjects\\DependencyWalker-rs\\testsources\\XInclude.exe").unwrap();
}
