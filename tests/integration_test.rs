use assert_cmd::prelude::*;
use ast2mdlib::TypeData;
use rstest::rstest;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[test]
fn report_non_existing_input_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("manual").arg("non_existing_file");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn read_input_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.current_dir("tests");
    cmd.arg("manual").arg("first.ast");
    let output = cmd.output().expect("failed to execute command");

    assert!(output.status.success());

    Ok(())
}

#[rstest]
fn test_golden_file(
    #[files("tests/input/*.ast")] input: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let dir = input.parent().expect("Invalid input directory");
    cmd.current_dir(dir);
    let file_name = input.file_name().expect("Invalid input file name");
    cmd.arg("manual").arg(file_name);
    let output = cmd.output().expect("Failed to execute command");

    assert!(output.status.success(), "Unexpected execution status");

    let actual_string = String::from_utf8(output.stdout).expect("Invalid output");
    let actual: Vec<TypeData> =
        serde_json::from_str(&actual_string).expect("Invalid output JSON format");

    let mut output_path = input.clone();
    output_path.pop(); // filename
    output_path.pop(); // input
    output_path.push("output");
    output_path.push(Path::new(file_name).with_extension("json"));
    let expected_string = fs::read_to_string(output_path).expect("Invalid expected output");
    let expected: Vec<TypeData> =
        serde_json::from_str(&expected_string).expect("Invalid expected JSON output");
    assert_eq!(actual, expected, "Actual and expected output don't match");
    println!("Actual: {actual:?}\nExpected: {expected:?}");

    Ok(())
}
