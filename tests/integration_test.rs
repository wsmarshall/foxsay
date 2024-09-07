use assert_cmd::prelude::*; //imports some useful traits, like cargo_bin(), assert(), success()
use std::process::Command; //Command struct for running program in a newly spawned process

#[test]
fn run_with_defaults() {
    Command::cargo_bin("foxsay")
        .expect("binary exists")
        .assert()
        .success();
}
