use assert_cmd::Command;
use std::fs;

//delta and inputs
type Outcome = Result<(), Box<dyn std::error::Error>>;

#[test]
fn temp_blank_input(){
    let mut cmd = Command::cargo_bin("q1").unwrap();
    cmd.assert().success().stdout("Input: <start_f> <end_f> <delta>");
}

#[test]
fn temp_input_more_than_3(){
    let mut cmd: Command = Command::cargo_bin("q1").unwrap();
    cmd.arg("300").arg("20").arg("23").arg("12").assert().success().stdout("Input: <start_f> <end_f> <delta>");
}

#[test]
fn temp_input_less_than_3(){
    let mut cmd: Command = Command::cargo_bin("q1").unwrap();
    cmd.arg("300").arg("20").assert().success().stdout("Input: <start_f> <end_f> <delta>");
}

#[test]
fn temp_str_input(){
    let mut cmd = Command::cargo_bin("q1").unwrap();
    cmd.arg("str").arg("0").arg("20").assert().failure().stdout("Input a numeric value for start");
}

#[test]
fn temp_str_input_2(){
    let mut cmd: Command = Command::cargo_bin("q1").unwrap();
    cmd.arg("300").arg("str").arg("20").assert().failure().stdout("Input a numeric value for end");
}


#[test]
fn temp_str_input_3(){
    let mut cmd: Command = Command::cargo_bin("q1").unwrap();
    cmd.arg("300").arg("0").arg("str").assert().failure().stdout("Input a numeric value for delta");
}

#[test] 
fn temp_start_less_than_end(){
    run(&["0", "10", "2"], "tests/expected/temp_start_less_than_end.txt").expect("Test failed");
} 


#[test] 
fn temp_start_more_than_end(){
    run(&["10", "0", "2"], "tests/expected/temp_start_more_than_end.txt").expect("Test failed");
} 


fn run(args: &[&str], file: &str) -> Outcome {
    let expected = fs::read_to_string(file)?;
    Command::cargo_bin("q1")?.args(args).assert().success().stdout(expected);
    Ok(())
}