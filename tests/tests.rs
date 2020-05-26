use assert_cmd::Command;

#[test]
fn it_helps() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn it_works() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("Wayne")
        .args(&["--gimme", "Brady"])
        .assert()
        .success();
}

#[test]
fn it_errors_on_bandinput() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("BadInput")
        .args(&["--gimme", "BadInput"])
        .assert()
        .failure();
}

#[test]
fn it_errors_without_gimme() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn it_fails_on_bad_command() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("foo")
        .assert()
        .failure();
}
