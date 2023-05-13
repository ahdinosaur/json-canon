use std::{collections::BTreeMap, fmt::Debug, io};

use json_canon::to_string;
use serde::Serialize;
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

fn test_ok<Expected, Input>(expected: Expected, input: Input) -> io::Result<()>
where
    Expected: AsRef<str> + Debug,
    Input: Serialize,
    String: PartialEq<Expected>,
{
    let actual = to_string(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_works() -> io::Result<()> {
    let expected = r#"{"a":1,"b":[],"c":2}"#;
    let input = &from_str::<Value>(r#"{"c": 2, "a": 1, "b": []}"#)?;
    test_ok(expected, input)
}

#[test]
fn test_empty_array() -> io::Result<()> {
    let expected = "[]";
    let input_json = json!([]);
    let input_rs: Vec<()> = vec![];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_one_element_array() -> io::Result<()> {
    let expected = "[123]";
    let input_json = json!([123]);
    let input_rs = vec![123];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_multi_element_array() -> io::Result<()> {
    let expected = "[123,456]";
    let input_json = json!([123, 456]);
    let input_rs = vec![123, 456];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_multi_element_mixed_array() -> io::Result<()> {
    #[derive(serde_derive::Serialize)]
    #[serde(untagged)]
    enum Val<'a> {
        Str(&'a str),
        Num(u32),
    }
    let expected = r#"[123,456,"hello"]"#;
    let input_json = json!([123, 456, "hello"]);
    let input_rs = vec![Val::Num(123), Val::Num(456), Val::Str("hello")];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_none_values_in_array() -> io::Result<()> {
    let expected = r#"[null,"hello"]"#;
    let input_json = json!([None::<()>, "hello"]);
    let input_rs: Vec<Option<&str>> = vec![None, Some("hello")];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
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
fn test_object_in_array() -> io::Result<()> {
    #[derive(serde_derive::Serialize)]
    #[serde(untagged)]
    enum Val<'a> {
        Str(&'a str),
        Num(u32),
    }
    let expected = r#"[{"a":"string","b":123}]"#;
    let input_json = json!([{ "b": 123, "a": "string" }]);
    let input_rs = vec![treemap![
        "b".to_string() => Val::Num(123),
        "a".to_string() => Val::Str("string")
    ]];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_empty_object() -> io::Result<()> {
    let expected = r#"{}"#;
    let input_json = json!({});
    let input_rs: BTreeMap<(), ()> = treemap![];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

// Undefined is also not possible because `serde_json` has no such thing.

#[test]
fn test_object_with_null_value() -> io::Result<()> {
    let expected = r#"{"test":null}"#;
    let input_json = json!({ "test": None::<()> });
    let input_rs: BTreeMap<&str, Option<&str>> = treemap![
        "test" => None
    ];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_object_with_one_property() -> io::Result<()> {
    let expected = r#"{"hello":"world"}"#;
    let input_json = json!({ "hello": "world" });
    let input_rs = treemap![
        "hello" => "world"
    ];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_object_with_more_than_one_property() -> io::Result<()> {
    let expected = r#"{"goodbye":"test","hello":"world"}"#;
    let input_json = json!({ "hello": "world", "goodbye": "test" });
    let input_rs = treemap![
        "hello" => "world",
        "goodbye" => "test"
    ];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_mixed_object_with_more_than_one_property() -> io::Result<()> {
    #[derive(serde_derive::Serialize)]
    #[serde(untagged)]
    enum Val<'a> {
        Str(&'a str),
        Num(u32),
    }
    let expected = r#"{"hello":"world","number":123}"#;
    let input_json = json!({ "hello": "world", "number": 123 });
    let input_rs = treemap![
        "hello" => Val::Str("world"),
        "number" => Val::Num(123)
    ];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_null() -> io::Result<()> {
    let expected = "null";
    let input_json = json!(None::<()>);
    let input_rs = None::<()>;
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_object_with_number_keys() -> io::Result<()> {
    let expected = r#"{"1":"One","2":"Two","3":"Three","4":"Four"}"#;
    let input_json = json!({
        "2": "Two",
        "4": "Four",
        "1": "One",
        "3": "Three"
    });
    let input_rs = treemap![
        2 => "Two",
        4 => "Four",
        1 => "One",
        3 => "Three"
    ];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_encode_newline_utf() -> io::Result<()> {
    let expected = r#""\n""#;
    let input_json = json!("\u{000a}");
    let input_rs = "\u{000a}";
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_sorting_utf() -> io::Result<()> {
    let expected = r#"{"\n":"Newline","1":"One"}"#;
    let input_json = json!({
        "1": "One",
        "\u{000a}": "Newline",
    });
    let input_rs = treemap![
        "1" => "One",
        "\u{000a}" => "Newline"
    ];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_object_with_wacky_keys() -> io::Result<()> {
    #[derive(PartialEq, Eq, PartialOrd, Ord, serde_derive::Serialize)]
    #[serde(untagged)]
    enum Key<'a> {
        Str(&'a str),
        Num(u32),
    }
    let expected = r#"{"\n":"Newline","1":"One","2":"Two","3":"Three","4":"Four"}"#;
    let input_json = json!({
        "2": "Two",
        "4": "Four",
        "1": "One",
        "3": "Three",
        "\u{000a}": "Newline",
    });
    let input_rs = treemap![
        Key::Num(2) => "Two",
        Key::Str("4") => "Four",
        Key::Str("1") => "One",
        Key::Num(3) => "Three",
        Key::Str("\u{000a}") => "Newline"
    ];
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_utf8_sort_bug() -> io::Result<()> {
    let input = r###"{"�\u0017B��":null,"�\u0017\\�4�":null}"###;
    let input_val: Value = from_str(input)?;
    let expected = input.to_string();
    let actual = to_string(&input_val)?;
    assert_eq!(actual, expected);
    Ok(())
}
