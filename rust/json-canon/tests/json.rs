use std::{
    env::current_dir,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use json_canon::to_string;
use serde_json::{from_str, Value};

#[test]
fn test_json_data() -> Result<(), io::Error> {
    #[track_caller]
    fn test_json(value: &Value, expected: &str) {
        let actual = to_string(value).unwrap();
        assert_eq!(actual, expected);
    }

    let test_data_path = current_dir()?.join(Path::new("../../test-data/generated/json.txt"));

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
