use assert_cmd::prelude::*; //imports some useful traits, like cargo_bin(), assert(), success()
use predicates::prelude::*;
use std::process::Command; //Command struct for running program in a newly spawned process

#[test]
fn run_with_defaults() {
    Command::cargo_bin("foxsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Wa-pa-pa-pa-pa-pa-pow!"));
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("foxsay")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();
    Ok(())
}
