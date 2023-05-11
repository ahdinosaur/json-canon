use std::io::Error;

use json_canon::to_string;
use serde_json::{from_str, Value};

#[track_caller]
fn test_data_import(input: &str, expected: &str) -> Result<(), Error> {
    let actual = to_string(&from_str::<Value>(input.trim())?)?;
    assert_eq!(actual, expected.trim());
    Ok(())
}

#[test]
fn arrays() -> Result<(), Error> {
    test_data_import(
        include_str!("../../../test-data/input/arrays.json"),
        include_str!("../../../test-data/output/arrays.json"),
    )?;
    Ok(())
}

#[test]
fn french() -> Result<(), Error> {
    test_data_import(
        include_str!("../../../test-data/input/french.json"),
        include_str!("../../../test-data/output/french.json"),
    )?;
    Ok(())
}

#[test]
fn structures() -> Result<(), Error> {
    test_data_import(
        include_str!("../../../test-data/input/structures.json"),
        include_str!("../../../test-data/output/structures.json"),
    )?;
    Ok(())
}

#[test]
fn unicode() -> Result<(), Error> {
    test_data_import(
        include_str!("../../../test-data/input/unicode.json"),
        include_str!("../../../test-data/output/unicode.json"),
    )?;
    Ok(())
}

#[test]
fn values() -> Result<(), Error> {
    test_data_import(
        include_str!("../../../test-data/input/values.json"),
        include_str!("../../../test-data/output/values.json"),
    )?;
    Ok(())
}

#[test]
fn weird() -> Result<(), Error> {
    test_data_import(
        include_str!("../../../test-data/input/weird.json"),
        include_str!("../../../test-data/output/weird.json"),
    )?;
    Ok(())
}

#[test]
fn fuzzy_1() -> Result<(), Error> {
    test_data_import(
        include_str!("../../../test-data/fuzzies/1.json"),
        include_str!("../../../test-data/fuzzies/1.json"),
    )?;
    Ok(())
}
