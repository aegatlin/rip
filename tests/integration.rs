use predicates::prelude::*;

#[test]
fn prettier() {
    let mut cmd = assert_cmd::Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.arg("help")
        .assert()
        .stdout(predicate::str::contains("rip runs a variety of actions"))
        .success();
}
