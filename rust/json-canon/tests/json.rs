use std::{
    env::current_dir,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use assert_text::assert_text_eq;
use json_canon::to_string;
use serde_json::{from_str, Value};
use similar_asserts::assert_eq;

#[test]
fn test_json_data() -> Result<(), io::Error> {
    #[track_caller]
    fn test_json(value: &Value, expected: &str) {
        let actual = to_string(value).unwrap();
        let actual_chars: Vec<char> = actual.chars().collect();
        let expected_chars: Vec<char> = expected.chars().collect();
        // assert_eq!(actual_chars, expected_chars);
        // assert_text_eq!(&actual.to_hex(), &expected.to_hex());
        assert_text_eq!(actual, expected);
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

/// A trait for converting a value to hexadecimal encoding
pub trait ToHex {
    /// Converts the value of `self` to a hex value, returning the owned
    /// string.
    fn to_hex(&self) -> String;
}

static CHARS: &'static [u8] = b"0123456789abcdef";

impl ToHex for [u8] {
    /// Turn a vector of `u8` bytes into a hexadecimal string.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate rustc_serialize;
    /// use rustc_serialize::hex::ToHex;
    ///
    /// fn main () {
    ///     let str = [52,32].to_hex();
    ///     println!("{}", str);
    /// }
    /// ```
    fn to_hex(&self) -> String {
        let mut v = Vec::with_capacity(self.len() * 2);
        for &byte in self.iter() {
            v.push(CHARS[(byte >> 4) as usize]);
            v.push(CHARS[(byte & 0xf) as usize]);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}

impl ToHex for &str {
    fn to_hex(&self) -> String {
        self.as_bytes().to_hex()
    }
}

impl ToHex for String {
    fn to_hex(&self) -> String {
        self.as_bytes().to_hex()
    }
}

impl<'a, T: ?Sized + ToHex> ToHex for &'a T {
    fn to_hex(&self) -> String {
        (**self).to_hex()
    }
}
