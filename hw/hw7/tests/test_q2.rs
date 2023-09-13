use assert_cmd::Command; 
use std::fs; 

//blank input, neg inp, positive inp, 0, stirng inptu

//problem should return [] []
#[test]
fn test_blank_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&[""]);
    cmd.assert().success().stdout("[(0.0, 0.0)]\n[(0.0, 0.0)]\n");
}

#[test]
fn test_zero_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&["0"]).assert().success().stdout("[]\n[]\n");
}

//problem
#[test]
fn test_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&["-3 2 4 5"]);
    cmd.assert().success().stdout("[(-3.0, 2.0), (4.0, 5.0)]\n[(4.0, 5.0), (-3.0, 2.0)]\n");
}

//problem
#[test]
fn test_3_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&["-3 2 4"]);
    cmd.assert().success().stdout("[(-3.0, 2.0)]\n[(-3.0, 2.0)]\n");
}

//problem
#[test]
fn test_string_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.args(&["nfds"]).assert().failure().stdout("Enter valid integers\n");
}