use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"100
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "203\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"27
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "57\n");
    assert!(output.stderr_str().is_empty());
}
