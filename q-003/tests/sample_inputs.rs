use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5
3 1 4 1 5
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "14\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3
10 20 50
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "80\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10
1 2 3 4 5 6 7 8 9 10
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "55\n");
    assert!(output.stderr_str().is_empty());
}
