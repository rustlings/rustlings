use assert_cmd::prelude::*;
use predicates::boolean::PredicateBooleanExt;
use std::process::Command;

#[test]
fn runs_without_arguments() {
    let mut cmd = Command::cargo_bin("rustlings").unwrap();
    cmd.assert().success();
}

#[test]
fn fails_when_in_wrong_dir() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .current_dir("tests/")
        .assert()
        .code(1);
}

#[test]
fn verify_all_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("v")
        .current_dir("tests/fixture/success")
        .assert()
        .success();
}

#[test]
fn verify_fails_if_some_fails() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("v")
        .current_dir("tests/fixture/failure")
        .assert()
        .code(1);
}

#[test]
fn run_single_compile_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "compSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_compile_failure() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "compFailure"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_test_failure() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "testFailure"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_not_passed() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "testNotPassed.rs"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_no_filename() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("r")
        .current_dir("tests/fixture/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_no_exercise() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "compNoExercise.rs"])
        .current_dir("tests/fixture/failure")
        .assert()
        .code(1);
}

#[test]
fn get_hint_for_single_test() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["h", "testFailure"])
        .current_dir("tests/fixture/failure")
        .assert()
        .code(0)
        .stdout("Hello!\n");
}

#[test]
fn run_compile_exercise_does_not_prompt() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "pending_exercise"])
        .current_dir("tests/fixture/state")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("I AM NOT DONE").not());
}

#[test]
fn run_test_exercise_does_not_prompt() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "pending_test_exercise"])
        .current_dir("tests/fixture/state")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("I AM NOT DONE").not());
}

#[test]
fn run_single_test_success_with_output() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["--nocapture", "r", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("THIS TEST TOO SHALL PAS"));
}

#[test]
fn run_single_test_success_without_output() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("THIS TEST TOO SHALL PAS").not());
}
