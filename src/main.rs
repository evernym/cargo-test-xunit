extern crate sxd_document;

mod element;
mod parser;

use std::fs::File;
use std::process::Command;
use sxd_document::Package;
use sxd_document::writer::format_document;


fn main() {

    println!("Running tests");

    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("failed to execute command");

    println!("Running tests -> done");

    println!("Parsing tests results");

    let report = parser::parse_test_report(output);

    println!("Parsing tests results -> done");

    println!("Building xunit test report");

    let package = Package::new();
    let document = element::build_xunit_report(&package, report);

    println!("Building xunit test report -> done");

    let mut f = File::create("test-results.xml").unwrap();

    println!("Writing file");

    format_document(&document, &mut f)
        .ok()
        .expect("unable to output XML");

    println!("Writing file -> done");
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