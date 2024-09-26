use std::error::Error;
use idna::*;
use idna::punycode::{encode_into, decode, decode_to_string, encode_str};

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
    println!("original input: {}", data);

    let data_byte = data.as_bytes();

    // test 1
    let mut new_data_bytes = Vec::new();
    for d in data_byte.iter() {
        let invalid_data = if *d <= 35 { d + 36 } else { *d };
        new_data_bytes.push(invalid_data);
    }
    let new_data_string = String::from_utf8(new_data_bytes.clone()).unwrap();
    let new_data = new_data_string.as_str();
    let mut output = String::new();
    encode_into(new_data.chars(), &mut output).unwrap();
    println!("Result1: {:?}, modified input: {:?}", output, new_data);

/*
    // test 2
    if let Ok(domain) =  std::str::from_utf8(&new_data_bytes) {
        let long_nested_domain = domain.repeat(1000);
        let res = domain_to_ascii(&long_nested_domain);
        println!("Result2: {:?}", res);
    } else {
        println!("Invalid domain");
    }
 */
    // test 3
    let malformed_input = unsafe { std::str::from_utf8_unchecked(data_byte) };
    let res3 = domain_to_ascii(malformed_input);
    println!("Result3:{:?}, modified input {:?}", res3, malformed_input);
    let res4 = decode(malformed_input);
    println!("Result4:{:?}", res4);
    let res5 = decode_to_string(malformed_input);
    println!("Result5:{:?}", res5);
    let res6 = encode_str(malformed_input);
    println!("Result6:{:?}", res6);
    Ok(())
}

#[test]
fn quick_test2() {
    let data = "`https://@💩.g메롱.똥.집ws    `";
    // let data = "http://דוגמה.ישראל:8080/login?username=test&auth=fail#retry\n";
    let res = run(data);
    println!("- idna result: {:?}", res);
}