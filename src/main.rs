extern crate sxd_document;

mod element;
mod parser;

use std::fs;
use std::process::Command;
use sxd_document::Package;
use sxd_document::writer::format_document;

fn main() {

    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("failed to execute command");

    let report = parser::parse_test_report(output);

    let package = Package::new();
    let document = element::build_xml(&package, report);

    let mut f = fs::File::create("test-results.xml").unwrap();

    format_document(&document, &mut f)
        .ok()
        .expect("unable to output XML");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn it_does_not_works() {
        assert!(false);
    }
}