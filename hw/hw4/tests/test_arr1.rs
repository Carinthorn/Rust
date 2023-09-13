use assert_cmd::Command; 
use std::fs;
//cargo test --test test_arr2

type Outcome = Result<(), Box<dyn std::error::Error>>; 

#[test]
fn arr_blank_input(){
    let mut cmd = Command::cargo_bin("q3").unwrap();
    cmd.assert().failure().stdout("Enter a numeric value");
}


#[test]
fn arr_str_input(){
    let mut cmd = Command::cargo_bin("q3").unwrap();
    cmd.arg("str").assert().failure().stdout("Enter a numeric value");
}

#[test]
fn arr_neg_input(){
    // run(&"-6", "tests/expected/arr_input3.txt");
    run(&"-6", "tests/expected/arrow1_input_6.txt").expect("Test failed");

}

#[test]
fn arr_zero_input(){
    let mut cmd =  Command::cargo_bin("q3").unwrap();
    cmd.arg("0").assert().success().stdout(""); 
}

#[test]
fn arr_input(){//
    run(&"6", "tests/expected/arrow1_input_6.txt").expect("Test failed");
}

fn run(arg: &str, file: &str) -> Outcome {
    let expected = fs::read_to_string(file)?;
    let output = Command::cargo_bin("q3")?.arg(arg).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, expected, "Test failed: Unexpected stdout");
    Ok(())
}