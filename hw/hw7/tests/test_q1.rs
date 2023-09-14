use assert_cmd::Command;
use std::fs; 
//blank input, neg inp, positive inp, 0, string input
//problem
#[test]
fn test_q1_blank_input(){
    let mut cmd = Command::cargo_bin("q1").unwrap();
    cmd.args(&[""]).assert().failure().stderr("Enter valid integers\n");
}

#[test]
fn test_q1_zero_input(){
    let mut cmd = Command::cargo_bin("q1").unwrap();
    cmd.args(&["0"]).assert().success().stdout("[0]\n[0]\n");
}

#[test]
fn test_q1_neg_input(){
    let mut cmd = Command::cargo_bin("q1").unwrap();
    cmd.args(&["-2", "-1", "-3", "-4"]).assert().success().stdout("[-4, -3, -2, -1]\n[-1, -2, -3, -4]\n");
}

#[test]
fn test_q1_pos_input(){
    let mut cmd = Command::cargo_bin("q1").unwrap();
    cmd.args(&["2", "1", "3", "4"]).assert().success().stdout("[1, 2, 3, 4]\n[4, 3, 2, 1]\n");
}

#[test]
fn test_q1_string_input(){
    let mut cmd = Command::cargo_bin("q1").unwrap();
    cmd.args(&["nfds"]).assert().failure().stderr("Enter valid integers\n");
    cmd.args(&["nfds", "nfds"]).assert().failure().stderr("Enter valid integers\n");

}

