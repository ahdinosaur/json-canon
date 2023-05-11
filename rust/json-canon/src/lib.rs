//! JSON Canonicalization Scheme (JCS)
//!
//! ### References
//!
//! [RFC 8785](https://tools.ietf.org/html/rfc8785)
//!
mod ser;

pub use self::ser::{to_string, to_vec, to_writer};
