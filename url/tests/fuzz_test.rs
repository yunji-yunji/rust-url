use std::cell::Cell;
use url::{Url, SyntaxViolation};
use std::error::Error;

#[cfg(any(unix, windows, target_os = "redox", target_os = "wasi"))]

// MIRIFLAG=-Zmiri-disable-isolation cargo +fuzz miri test --package url --test fuzz_test -- fuzz_syntax_violation_callback_types --exact --show-output data=url_seddf//
fn run(data: &str) -> Result<Option<SyntaxViolation>, Box<dyn Error>> {
    let violation = Cell::new(None);
    Url::options()
        .syntax_violation_callback(Some(&|v| violation.set(Some(v))))
        .parse(data)?;

    let v = violation.take();
    Ok(v)
}

#[test]
fn fuzz_syntax_violation_callback_types() {
    let args: Vec<String> = std::env::args().collect();
    let mut data_arg: Option<String> = None;
    for arg in args.iter().skip(1) {
        if arg.starts_with("data=") {
            data_arg = Some(arg.chars().skip(5).collect());
            break;
        }
    }
    if let Some(data) = data_arg {
        println!("\n- input data: {:?}", data);
        let res = run(data.as_str());
        println!("- parse result: {:?}", res);
    } else {
        panic!("input data not found");
    }
}