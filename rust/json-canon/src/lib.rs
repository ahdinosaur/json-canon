//! # `json-canon`
//!
//! Serialize JSON into a canonical format.
//!
//! Safe for generating a consistent cryptographic hash or signature across platforms.
//!
//! Follows [RFC8785: JSON Canonicalization Scheme (JCS)]
//!
//! [RFC8785: JSON Canonicalization Scheme (JCS)]: https://tools.ietf.org/html/rfc8785
//!
//! ## Example
//!
//! ```rust
//! # use json_canon::to_string;
//! # use serde_json::{json, Error};
//! # fn main() -> Result<(), Error> {
//! let data = json!({
//!     "from_account": "543 232 625-3",
//!     "to_account": "321 567 636-4",
//!     "amount": 500,
//!     "currency": "USD"
//! });
//!
//! println!("{}", to_string(&data)?);
//! // {"amount":500,"currency":"USD","from_account":"543 232 625-3","to_account":"321 567 636-4"}
//! # Ok(())
//! # }
//! ```
//!
//! ## Caveats
//!
//! `serde_json` deserializes `f64::NAN` and `f64::Infinite` as `None`, so if given a Rust struct with these values, the `json-canon` will currently output `"null"`.
//!

mod object;
mod ser;

pub use self::ser::{to_string, to_vec, to_writer};
