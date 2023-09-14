use assert_cmd::Command; 
use std::fs; 

//blank input, neg inp, positive inp, 0, stirng inptu
#[test]
fn test_q2_blank_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&[""]).assert().failure().stderr("Need more than 1 input\n");
}

#[test]
fn test_q2_zero_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&["0"]).assert().failure().stderr("Need more than 1 input\n");
}

#[test]
fn test_q2_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&["-3", "2" ,"4" ,"5"]);
    cmd.assert().success().stdout("[(-3.0, 2.0), (4.0, 5.0)]\n[(4.0, 5.0), (-3.0, 2.0)]\n");
}

#[test]
fn test_q2_3_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&["-3", "2", "4"]);
    cmd.assert().success().stdout("[(-3.0, 2.0)]\n[(-3.0, 2.0)]\n");
}

#[test]
fn test_q2_string_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&["nfds"]).assert().failure().stderr("Need more than 1 input\n");
    cmd.args(&["nfds", "nfds"]).assert().failure().stderr("Enter valid integers\n");
}
