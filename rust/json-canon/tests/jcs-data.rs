use std::error::Error;

use json_canon::to_string;
use serde_json::{from_str, Value};

#[test]
fn test_jcs_data() -> Result<(), Box<dyn Error>> {
    #[track_caller]
    fn test_jcs_data_import(input: &str, expected: &str) -> Result<(), Box<dyn Error>> {
        let actual = to_string(&from_str::<Value>(input.trim())?)?;
        assert_eq!(actual, expected.trim());
        Ok(())
    }

    test_jcs_data_import(
        include_str!("../../../jcs-test-data/input/arrays.json"),
        include_str!("../../../jcs-test-data/output/arrays.json"),
    )?;

    test_jcs_data_import(
        include_str!("../../../jcs-test-data/input/french.json"),
        include_str!("../../../jcs-test-data/output/french.json"),
    )?;

    test_jcs_data_import(
        include_str!("../../../jcs-test-data/input/structures.json"),
        include_str!("../../../jcs-test-data/output/structures.json"),
    )?;

    test_jcs_data_import(
        include_str!("../../../jcs-test-data/input/unicode.json"),
        include_str!("../../../jcs-test-data/output/unicode.json"),
    )?;

    test_jcs_data_import(
        include_str!("../../../jcs-test-data/input/values.json"),
        include_str!("../../../jcs-test-data/output/values.json"),
    )?;

    test_jcs_data_import(
        include_str!("../../../jcs-test-data/input/weird.json"),
        include_str!("../../../jcs-test-data/output/weird.json"),
    )?;

    Ok(())
}
