extern crate sxd_document;

use sxd_document::dom::{Element, Root, Document};
use std::fmt::Display;
use sxd_document::Package;
use parser::TestReport;


trait ElementBuilder <'a> {
    fn set_attr <T: Display> (self, attr: &str, value: T) -> Element <'a>;
    fn append_to_element  (self, parent: &Element) -> Element <'a>;
    fn append_to_root  (self, parent: &Root) -> Element <'a>;
}

impl <'a> ElementBuilder <'a>  for Element <'a> {
    fn set_attr <T: Display> (self, attr: &str, value: T) -> Element <'a> {
        self.set_attribute_value(attr, &format!("{}", value));
        self
    }

    fn append_to_element (self, parent: &Element) -> Element <'a> {
        parent.append_child(self);
        self
    }

    fn append_to_root (self, root: &Root) -> Element <'a> {
        root.append_child(self);
        self
    }
}

pub fn build_xunit_report <'a>(package : &'a Package, report: TestReport) -> Document<'a> {

    let dom = package.as_document();

    let test_suites =
        dom.create_element("testsuites")
            .set_attr("name", &report.name)
            .set_attr("errors", report.failed)
            .set_attr("tests", report.total)
            .append_to_root(&dom.root());

    let test_suite =
        dom.create_element("testsuite")
            .set_attr("name", &report.name)
            .set_attr("errors", report.failed)
            .set_attr("failures", report.failed)
            .set_attr("tests", report.total)
            .append_to_element(&test_suites);

    for testcase in report.test_results {
        let test_case =
            dom.create_element("testcase")
                .set_attr("name", testcase.name)
                .append_to_element(&test_suite);

        if let Some(err) = testcase.error {
            dom.create_element("failure")
                .set_attr("message", err)
                .append_to_element(&test_case);
        }
    }

    dom
}
