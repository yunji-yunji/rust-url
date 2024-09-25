use std::error::Error;
use idna::*;
use crate::punycode::encode_into;

#[test]
fn quick_test() {
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
        println!("- idna result: {:?}", res);
    } else {
        panic!("input data not found");
    }
}

fn run(data: &str) -> Result<(), Box<dyn Error>> {
    let data_byte = data.as_bytes();
    let mut new_data_bytes = Vec::new();
    for d in data_byte.iter() {
        let invalid_data = if *d <= 35 { d + 36 } else { *d };
        new_data_bytes.push(invalid_data);
    }

    let new_data_string = String::from_utf8(new_data_bytes.clone()).unwrap();
    let new_data = new_data_string.as_str();
    let mut output = String::new();
    let res0 = encode_into(new_data.chars(), &mut output);
    println!("Result: {:?}", res0);

    if let Ok(domain) =  std::str::from_utf8(&new_data_bytes) {
        // Generate a long nested domain by repeating the domain multiple times
        let long_nested_domain = domain.repeat(100);  // Arbitrary large number
        let res = domain_to_ascii(&long_nested_domain);
        println!("Result: {:?}", res);
    } else {
        println!("Invalid domain");
    }

    // Additionally, pass malformed inputs (invalid UTF-8 byte sequences) directly
    // Attempt to convert invalid byte sequences to provoke panics
    let malformed_input = unsafe { std::str::from_utf8_unchecked(data_byte) };
    let res2 = domain_to_ascii(malformed_input);
    println!("Result2: {:?}", res2);
    Ok(())
}

#[test]
fn quick_test2() {
    let data = "http://דוגמה.ישראל:8080/login?username=test&auth=fail#retry\n";
    let res = run(data);
    println!("- idna result: {:?}", res);
}