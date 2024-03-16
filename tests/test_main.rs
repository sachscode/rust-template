use assert_cmd::Command;

#[test]
fn test_main() {
    let mut cmd = Command::cargo_bin("hello_world").unwrap();
    cmd.assert()
       .stdout("Hello, world!\n")
       .success();
}