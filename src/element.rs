extern crate sxd_document;

use sxd_document::dom;
use std::fmt;
use std::marker;
use sxd_document::Package;
use parser::TestReport;


struct Element<'a> {
    elem: dom::Element<'a>,
}

impl<'a> Element<'a> {
    fn attr<A: fmt::Display + marker::Sized>(self, k: &str, v: A) -> Element<'a> {
        self.elem.set_attribute_value(k, &format!("{}", v));
        self
    }
    fn append_to<'b>(self, other: &Element<'b>) -> Element<'a> {
        other.elem.append_child(self.elem);
        self
    }
}

pub fn build_xml <'a>(package : &'a Package, report: TestReport) -> sxd_document::dom::Document<'a> {

    let d = package.as_document();

    let test_suites =
        Element { elem: d.create_element("testsuites") }
            .attr("name", &report.name)
            .attr("errors", report.failed)
            .attr("tests", report.total);

    d.root().append_child(test_suites.elem);

    let test_suite =
        Element { elem: d.create_element("testsuite") }
            .attr("name", &report.name)
            .attr("errors", report.failed)
            .attr("failures", report.failed)
            .attr("tests", report.total)
            .append_to(&test_suites);

    for testcase in report.test_results {
        let test_case =
            Element { elem: d.create_element("testcase") }
                .attr("name", testcase.name)
                .append_to(&test_suite);

        if let Some(err) = testcase.error {
            Element { elem: d.create_element("failure") }
                .attr("message", err)
                .append_to(&test_case);
        }
    }

    d
}
