use std::{collections::BTreeMap, error::Error};

use json_canon::to_string;
use serde_json::{from_str, json, Value};

macro_rules! treemap {
    () => {
        BTreeMap::new()
    };
    ($($k:expr => $v:expr),+) => {
        {
            let mut m = BTreeMap::new();
            $(
                m.insert($k, $v);
            )+
            m
        }
    };
}

#[test]
fn test_works() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        to_string(&from_str::<Value>(r#"{"c": 2, "a": 1, "b": []}"#)?)?,
        r#"{"a":1,"b":[],"c":2}"#,
    );
    Ok(())
}

#[test]
fn test_empty_array() -> Result<(), Box<dyn Error>> {
    let input = json!([]);
    let expected = "[]".to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_one_element_array() -> Result<(), Box<dyn Error>> {
    let input = json!([123]);
    let expected = "[123]".to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_multi_element_array() -> Result<(), Box<dyn Error>> {
    let input = json!([123, 456, "hello"]);
    let expected = r#"[123,456,"hello"]"#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_null_and_undefined_values_in_array() -> Result<(), Box<dyn Error>> {
    let input = json!([None::<()>, "hello"]);
    let expected = r#"[null,"hello"]"#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

/*

NaN and Infinity are not possible because

```
let input = Value::Number(Number::from_f64(f64::NAN));
```

is not possible.

`serde_json::Number::from_f64` returns None on NaN or Infinity.

*/

#[test]
fn test_object_in_array() -> Result<(), Box<dyn Error>> {
    let input = json!([{ "b": 123, "a": "string" }]);
    let expected = r#"[{"a":"string","b":123}]"#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_empty_object() -> Result<(), Box<dyn Error>> {
    let input = json!({});
    let expected = "{}".to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

// Undefined is also not possible because `serde_json` has no such thing.

#[test]
fn test_object_with_null_value() -> Result<(), Box<dyn Error>> {
    let input = json!({ "test": None::<()> });
    let expected = r#"{"test":null}"#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_object_with_one_property() -> Result<(), Box<dyn Error>> {
    let input = json!({ "hello": "world" });
    let expected = r#"{"hello":"world"}"#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_object_with_more_than_one_property() -> Result<(), Box<dyn Error>> {
    let input = json!({ "hello": "world", "number": 123 });
    let expected = r#"{"hello":"world","number":123}"#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_null() -> Result<(), Box<dyn Error>> {
    let input = json!(None::<()>);
    let expected = "null".to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_object_with_number_key() -> Result<(), Box<dyn Error>> {
    let input = json!({ "42": "foo" });
    let expected = r#"{"42":"foo"}"#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_encode_newline_utf() -> Result<(), Box<dyn Error>> {
    let input = json!("\u{000a}");
    let expected = r#""\n""#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_sorting_utf() -> Result<(), Box<dyn Error>> {
    let input = json!({
        "1": "One",
        "\u{000a}": "Newline",
    });
    let expected = r#"{"\n":"Newline","1":"One"}"#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_sorting_number_keys() -> Result<(), Box<dyn Error>> {
    let input = treemap![
        2 => "Two",
        4 => "Four",
        1 => "One",
        3 => "Three"
    ];
    let expected = r#"{"1":"One","2":"Two","3":"Three","4":"Four"}"#.to_string();
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_utf8_sort_bug() -> Result<(), Box<dyn Error>> {
    let input = r###"{"�\u0017B��":null,"�\u0017\\�4�":null}"###;
    let input_val: Value = from_str(input)?;
    let expected = input.to_string();
    let actual = to_string(&input_val)?;
    assert_eq!(actual, expected);
    Ok(())
}
