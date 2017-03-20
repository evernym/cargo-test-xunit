extern crate regex;

use self::regex::Regex;
use std::process::Output;
use std::string::String;
use std::str::from_utf8;


pub struct TestReport {
    pub name: String,
    pub total: i32,
    pub failed: i32,
    pub test_results: Vec<TestResult>
}

pub struct TestResult {
    pub name: String,
    pub result: String,
    pub error: Option<String>
}

pub fn parse_test_report(output: Output) -> TestReport {

    let stdout = from_utf8(&output.stdout).unwrap();
    let stderr = from_utf8(&output.stderr).unwrap();

    println!("{}", &stdout);

    let test_results_regex = Regex::new(r"test (?P<name>[\w:]+) \.\.\. (?P<result>[\w]+)").unwrap();
    let mut failed = 0;

    let test_results : Vec<TestResult> = test_results_regex.captures_iter(stdout)
        .map(|test|
            TestResult{
                name: test["name"].to_string(),
                result: test["result"].to_string(),
                error: match &test["result"] {
                    "FAILED" => {
                        failed+=1;

                        let find = &format!(r"---- {} stdout ----[\r\n]+(?P<error>[^\r\n]+)", &test["name"])[..];

                        Regex::new(find).unwrap()
                            .captures(stdout)
                            .and_then(|error_caps|
                                Some(error_caps["error"].to_string())
                            )
                    },
                    _ => None
                }
            }
        )
        .collect();

    let total = &test_results.len();

    let suite_name = Regex::new(r"Running (?P<suite_name>[\w/:-]+)").unwrap()
        .captures(stderr)
        .unwrap();

    TestReport {
        total: *total as i32,
        failed: failed,
        test_results: test_results,
        name: suite_name["suite_name"].to_string()
    }
}