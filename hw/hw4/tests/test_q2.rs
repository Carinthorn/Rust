use assert_cmd::Command;

// cargo test --test test_score

#[test]
fn score_blank_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.assert().success().stdout("");
}

#[test]
fn score_str_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.arg("fjdaklf").assert().failure().stdout("Enter a numeric score value\n");
}

#[test]
fn score_many_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.arg("99").arg("88").arg("77").arg("66").arg("55").arg("44").arg("-5").arg("120").assert().success().stdout("Excellent with A+ A B C D Failed with F Invalid score Invalid score ");
}

#[test]
fn score_with_str_input(){
    let mut cmd = Command::cargo_bin("q2").unwrap();
    cmd.arg("99").arg("str").assert().failure().stdout("Enter a numeric score value\n");
}






