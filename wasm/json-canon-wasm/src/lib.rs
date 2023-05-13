mod utils;

use js_sys::{Array, JsString, Object, JSON};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

const UNDEFINED_TYPE: JsString = JsString::from_str("undefined").unwrap();
const SYMBOL_TYPE: JsString = JsString::from_str("symbol").unwrap();
const BOOLEAN_TYPE: JsString = JsString::from_str("boolean").unwrap();
const NUMBER_TYPE: JsString = JsString::from_str("number").unwrap();
const STRING_TYPE: JsString = JsString::from_str("string").unwrap();
const FUNCTION_TYPE: JsString = JsString::from_str("function").unwrap();
const OBJECT_TYPE: JsString = JsString::from_str("object").unwrap();

#[wasm_bindgen]
pub fn serialize(value: &JsValue) -> Result<JsString, JsValue> {
    let value_type: JsString = JsString::unchecked_from_js(value.js_typeof());

    match &value_type {
        &UNDEFINED_TYPE | &SYMBOL_TYPE => {
            return serialize_null();
        }
        &BOOLEAN_TYPE => {
            return serialize_boolean(value.as_bool().unwrap());
        }
        &NUMBER_TYPE => {
            return serialize_number(value.as_f64().unwrap());
        }
        &STRING_TYPE => {
            let string = JsValue::dyn_ref::<JsString>(value).ok_or::<JsValue>(
                JsError::new("Strings must be valid Unicode and not contain any surrogate pairs")
                    .into(),
            )?;
            return serialize_string(string);
        }
        &FUNCTION_TYPE => return serialize_function(value),
        &OBJECT_TYPE => {}
        _ => {
            return JSON::stringify(value);
        }
    };

    if value.loose_eq(&JsValue::NULL) {
        return Ok(NULL_STR);
    }

    if value.is_array() {
        serialize_array(Array::unchecked_from_js_ref(value))
    } else {
        serialize_object(Object::unchecked_from_js_ref(value))
    }
}

const NULL_STR: JsString = JsString::from_str("null").unwrap();

fn serialize_null() -> Result<JsString, JsValue> {
    Ok(NULL_STR)
}

const TRUE_STR: JsString = JsString::from_str("true").unwrap();
const FALSE_STR: JsString = JsString::from_str("false").unwrap();

fn serialize_boolean(value: bool) -> Result<JsString, JsValue> {
    if value {
        Ok(TRUE_STR)
    } else {
        Ok(FALSE_STR)
    }
}

fn serialize_number(value: f64) -> Result<JsString, JsValue> {
    if value.is_nan() {
        Err(JsError::new("NaN is not allowed").into())
    } else if value.is_infinite() {
        Err(JsError::new("Infinity is not allowed").into())
    } else {
        JSON::stringify(&JsValue::from_f64(value))
    }
}

fn serialize_string(value: &JsString) -> Result<JsString, JsValue> {
    if !value.is_valid_utf16() {
        Err(
            JsError::new("Strings must be valid Unicode and not contain any surrogate pairs")
                .into(),
        )
    } else {
        JSON::stringify(&value.dyn_into::<JsValue>().unwrap())
    }
}

fn serialize_function(value: &JsValue) -> Result<JsString, JsValue> {
    JSON::stringify(value)
}

fn serialize_array(arr: &Array) -> Result<JsString, JsValue> {
    let ret = String::new();
    let length = arr.length();
    for i in 0..length {
        let val = arr.get(i);
        if i != 0 {
            ret.push_str(",");
        }
        ret.push_str()
    }
}

fn serialize_object(value: &Object) -> Result<JsString, JsValue> {}

/*
function serializeArray(arr) {
  let str = '['
  const length = arr.length
  for (let i = 0; i < length; i++) {
    const val = arr[i]
    if (i !== 0) str += ','
    str += serialize(val)
  }
  return str + ']'
}

function serializeObject(obj) {
  const sortedKeys = sort(Object.keys(obj))
  let str = '{'
  const length = sortedKeys.length
  for (let i = 0; i < length; i++) {
    const key = sortedKeys[i]
    const val = obj[key]
    if (val === undefined || typeof val === 'symbol') {
      continue
    }
    if (i !== 0 && str.length !== 0) {
      str += ','
    }
    str += serialize(key) + ':' + serialize(val)
  }
  return str + '}'
}

// https://github.com/BridgeAR/safe-stable-stringify/blob/26dc000/index.js#L35-L51
function sort(array) {
  // Insertion sort is very efficient for small input sizes but it has a bad
  // worst case complexity. Thus, use native array sort for bigger values.
  if (array.length > 2e2) {
    return array.sort()
  }
  for (let i = 1; i < array.length; i++) {
    const currentValue = array[i]
    let position = i
    while (position !== 0 && array[position - 1] > currentValue) {
      array[position] = array[position - 1]
      position--
    }
    array[position] = currentValue
  }
  return array
}

const stringSurrogateRegex = /\p{Surrogate}/u

function isWellFormed(str) {
  if (typeof String.prototype.isWellFormed === 'function') {
    return str.isWellFormed()
  }

  return !stringSurrogateRegex.test(str)
}
*/
