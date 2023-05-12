use std::{
    env::current_dir,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
};

use json_canon::to_string;
use serde_json::{from_str, Value};

#[test]
fn test_json_data_from_file() -> Result<(), io::Error> {
    let test_data_path = current_dir()?.join(Path::new("../../test-data/generated/json.txt"));

    // only run test if generated file exists
    if !test_data_path.exists() {
        return Ok(());
    }

    let file = File::open(test_data_path)?;
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        let expected = line.trim();
        let value = from_str(&line)?;
        test_json(&value, expected);
    }

    Ok(())
}

#[test]
fn test_json_data_from_command() -> Result<(), io::Error> {
    let test_command_path = current_dir()?.join(Path::new("../../js/json-canon-fuzz/src/json"));

    let mut child = Command::new("node")
        .arg(test_command_path)
        .arg("100000")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let reader = io::BufReader::new(stdout);

    for line_result in reader.lines() {
        let line = line_result?;
        let expected = line.trim();
        let value = from_str(&line)?;
        test_json(&value, expected);
    }

    let ecode = child.wait()?;
    assert!(ecode.success());

    Ok(())
}

#[track_caller]
fn test_json(value: &Value, expected: &str) {
    let actual = to_string(value).unwrap();
    assert_eq!(actual, expected);
}
