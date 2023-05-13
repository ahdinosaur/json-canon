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

fn test_err<ExpectedErr, Input>(expected: ExpectedErr, input: Input) -> io::Result<()>
where
    ExpectedErr: AsRef<str> + Debug,
    Input: Serialize,
    String: PartialEq<ExpectedErr>,
{
    let result = to_string(&input);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.to_string(), expected);
    Ok(())
}

#[test]
fn test_works() -> io::Result<()> {
    let expected = r#"{"a":1,"b":[],"c":2}"#;
    let input = &from_str::<Value>(r#"{"c": 2, "a": 1, "b": []}"#)?;
    test_ok(expected, input)
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
fn test_encode_special_utf_ascii() -> io::Result<()> {
    let expected = r#""\n""#;
    let input_json = json!("\u{000a}");
    let input_rs = "\u{000a}";
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs)?;
    Ok(())
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

#[test]
fn test_array_with_large_integer_values() -> io::Result<()> {
    // test numbers are
    //   larger than JavaScript's Number.MAX_SAFE_INTEGER
    //   and less than i64::MAX
    macro_rules! create_input_rs {
        () => {
            vec![
                9_100_000_000_000_000,
                9_000_000_000_000_000,
                9_200_000_000_000_000,
            ]
        };
    }
    let input_rs_u64: Vec<u64> = create_input_rs!();
    let input_rs_u128: Vec<u128> = create_input_rs!();
    let input_rs_i64: Vec<i64> = create_input_rs!();
    let input_rs_i128: Vec<i128> = create_input_rs!();
    test_err("u64 must be less than JSON max safe integer", input_rs_u64)?;
    test_err(
        "u128 must be less than JSON max safe integer",
        input_rs_u128,
    )?;
    test_err("i64 must be less than JSON max safe integer", input_rs_i64)?;
    test_err(
        "i128 must be less than JSON max safe integer",
        input_rs_i128,
    )?;
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
fn test_object_with_none_keys() -> io::Result<()> {
    let expected_err = "key must be a string";
    let input_rs = treemap![
        None => "None",
        Some("Some") => "Some"
    ];
    test_err(expected_err, input_rs)?;
    Ok(())
}

#[test]
fn test_object_with_bool_keys() -> io::Result<()> {
    let expected_err = "key must be a string";
    let input_rs = treemap![
        true => "True",
        false => "False"
    ];
    test_err(expected_err, input_rs)?;
    Ok(())
}

#[test]
fn test_object_with_valid_integer_keys() -> io::Result<()> {
    macro_rules! create_input_rs {
        () => {
            treemap![
                2 => "Two",
                4 => "Four",
                1 => "One",
                3 => "Three"
            ]
        };
    }
    let expected = r#"{"1":"One","2":"Two","3":"Three","4":"Four"}"#;
    let input_json = json!({
        "2": "Two",
        "4": "Four",
        "1": "One",
        "3": "Three"
    });
    let input_rs_u8: BTreeMap<u8, &str> = create_input_rs!();
    let input_rs_u16: BTreeMap<u16, &str> = create_input_rs!();
    let input_rs_u32: BTreeMap<u32, &str> = create_input_rs!();
    let input_rs_u64: BTreeMap<u64, &str> = create_input_rs!();
    let input_rs_u128: BTreeMap<u128, &str> = create_input_rs!();
    let input_rs_i8: BTreeMap<i8, &str> = create_input_rs!();
    let input_rs_i16: BTreeMap<i16, &str> = create_input_rs!();
    let input_rs_i32: BTreeMap<i32, &str> = create_input_rs!();
    let input_rs_i64: BTreeMap<i64, &str> = create_input_rs!();
    let input_rs_i128: BTreeMap<i128, &str> = create_input_rs!();
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs_u8)?;
    test_ok(expected, input_rs_u16)?;
    test_ok(expected, input_rs_u32)?;
    test_ok(expected, input_rs_u64)?;
    test_ok(expected, input_rs_u128)?;
    test_ok(expected, input_rs_i8)?;
    test_ok(expected, input_rs_i16)?;
    test_ok(expected, input_rs_i32)?;
    test_ok(expected, input_rs_i64)?;
    test_ok(expected, input_rs_i128)?;
    Ok(())
}

#[test]
fn test_object_with_large_integer_keys() -> io::Result<()> {
    // test numbers are
    //   larger than JavaScript's Number.MAX_SAFE_INTEGER
    //   and less than i64::MAX
    macro_rules! create_input_rs {
        () => {
            treemap![
                9_100_000_000_000_000 => "OKAYY",
                9_000_000_000_000_000 => "WOWZA",
                9_200_000_000_000_000 => "YIPES"
            ]
        };
    }
    let expected =
        r#"{"9000000000000000":"WOWZA","9100000000000000":"OKAYY","9200000000000000":"YIPES"}"#;
    let input_json = json!({
        "9100000000000000": "OKAYY",
        "9000000000000000": "WOWZA",
        "9200000000000000": "YIPES"
    });
    let input_rs_u64: BTreeMap<u64, &str> = create_input_rs!();
    let input_rs_u128: BTreeMap<u128, &str> = create_input_rs!();
    let input_rs_i64: BTreeMap<i64, &str> = create_input_rs!();
    let input_rs_i128: BTreeMap<i128, &str> = create_input_rs!();
    test_ok(expected, input_json)?;
    test_ok(expected, input_rs_u64)?;
    test_ok(expected, input_rs_u128)?;
    test_ok(expected, input_rs_i64)?;
    test_ok(expected, input_rs_i128)?;
    Ok(())
}

#[test]
fn test_object_with_unit_variant_keys() -> io::Result<()> {
    let expected = r#"{"One":"One","Three":"Three","Two":"Two"}"#;
    #[derive(PartialEq, Eq, PartialOrd, Ord, serde_derive::Serialize)]
    enum Key {
        One,
        Two,
        Three,
    }
    let input_rs = treemap![
        Key::One => "One",
        Key::Two => "Two",
        Key::Three => "Three"
    ];
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_object_with_newtype_keys() -> io::Result<()> {
    let expected = r#"{"One":"One","Three":"Three","Two":"Two"}"#;
    #[derive(PartialEq, Eq, PartialOrd, Ord, serde_derive::Serialize)]
    struct Key<'a>(&'a str);
    let input_rs = treemap![
        Key("One") => "One",
        Key("Two") => "Two",
        Key("Three") => "Three"
    ];
    test_ok(expected, input_rs)?;
    Ok(())
}

#[test]
fn test_object_with_utf_keys() -> io::Result<()> {
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
fn test_bug_utf8_sort() -> io::Result<()> {
    let input = r###"{"�\u0017B��":null,"�\u0017\\�4�":null}"###;
    let input_val: Value = from_str(input)?;
    let expected = input.to_string();
    let actual = to_string(&input_val)?;
    assert_eq!(actual, expected);
    Ok(())
}
